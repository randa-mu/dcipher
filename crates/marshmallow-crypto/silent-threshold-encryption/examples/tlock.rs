use anyhow::Result;
use ark_ec::bls12::Bls12Config;
use ark_ec::hashing::{curve_maps::wb::WBMap, HashToCurve};
use ark_ec::pairing::{Pairing, PairingOutput};
use ark_ff::{field_hashers::DefaultFieldHasher, UniformRand};
use ark_poly::univariate::DensePolynomial;
use ark_serialize::CanonicalSerializeHashExt;
use ark_std::rand::Rng;
use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305, KeyInit,
};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{digest::generic_array::GenericArray, Keccak256, Shake128};
use silent_threshold_encryption::{
    decryption::agg_dec,
    encryption::{encrypt_with_gamma_g2, Ciphertext},
    kzg::{UniversalParams, KZG10},
    setup::{AggregateKey, SecretKey},
};
use std::{thread::sleep, time::Duration};

type E = ark_bls12_381::Bls12_381;
type G2 = <E as Pairing>::G2;
type UniPoly381 = DensePolynomial<<E as Pairing>::ScalarField>;

struct TlockCiphertext {
    silent_ct: Ciphertext<E>,
    ct: Vec<u8>,
    nonce: Vec<u8>,
}

fn round_to_gamma_g2(round: u64) -> <E as Pairing>::G2 {
    let mut round_hash = [0u8; 48]; // read 48 bytes to prevent bias
    let round = round.to_be_bytes();

    Shake128::default()
        .chain(round)
        .finalize_xof()
        .read(&mut round_hash);

    let hasher = ark_ec::hashing::map_to_curve_hasher::MapToCurveBasedHasher::<
        G2,
        DefaultFieldHasher<Keccak256, 128>,
        WBMap<<ark_bls12_381::Config as Bls12Config>::G2Config>,
    >::new(b"test")
    .unwrap();

    hasher.hash(&round_hash).unwrap().into()
}

fn tlock_encrypt(
    plaintext: &[u8],
    round: u64,
    threshold: usize,
    ak: &AggregateKey<E>,
    params: &UniversalParams<E>,
) -> TlockCiphertext {
    println!(
        "Encrypting message `{}` for decryption at time {round}\n\n",
        std::str::from_utf8(plaintext).unwrap()
    );

    // Gamma is the hash of the current round number
    let gamma = round_to_gamma_g2(round);

    // Get ciphertext using the entire quorum with a threshold of 5
    let rand_pairing = PairingOutput::<E>::rand(&mut OsRng);
    let chacha_key = rand_pairing.hash::<Keccak256>();
    let cipher = ChaCha20Poly1305::new(&chacha_key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let chacha_ct = cipher.encrypt(&nonce, plaintext).unwrap();
    let tlock_ct = encrypt_with_gamma_g2::<E>(ak, threshold, rand_pairing, params, &gamma);

    TlockCiphertext {
        silent_ct: tlock_ct,
        ct: chacha_ct,
        nonce: nonce.to_vec(),
    }
}

fn main() -> Result<()> {
    let mut rng = ark_std::test_rng();

    // Number of parties + one dummy party
    const N: usize = 31 + 1;
    const _: () = assert!(N != 0 && (N & (N - 1)) == 0); // N non-zero and power of 2

    // Setup() w/ n = M + 1. May need to be a power of t: "Henceforth, we will consider that M + 1 is a power of 2 and we set the subgroup H = {ω0, ω1, . . . , ωM} of roots of unity to be such that |H| = M + 1. M is the maximum number of decryptors participating in the system"
    // todo: understand why power of 2
    // Likely need to set a high number of parties in prevision since we somehow need to safely agree upon a tau and compute the pairings
    // Still a one-time setup, but then we can onboard new parties by somply computing a different aggregation key in PreProcess.
    let params = KZG10::<E, UniPoly381>::setup(N, &mut rng).unwrap();
    // params = pp = CRS = ([\tau^1]_{1,2}, ..., [\tau^n]_{1,2})

    let mut sk = Vec::with_capacity(N);
    let mut pk = Vec::with_capacity(N);

    // Dummy party with sk0 = 1, pk0 = ([1]_1 = G, hint_i = HintGen(CRS, sk_i, i, M))
    sk.push(SecretKey::<E>::new(&mut rng));
    sk[0].nullify(); // sk0 = 1
    pk.push(sk[0].get_pk(0, &params, N)?);

    // Simulate the key generation for the n parties
    for i in 1..N {
        sk.push(SecretKey::<E>::new(&mut rng));
        pk.push(sk[i].get_pk(i, &params, N)?)
    }

    // Publish the public keys and hints to other parties
    // not sure what can of model is assumed, likely a verifiable broadcast from the games

    // This is basically preprocess, compute the sums for ak, ek. Rest of parameters are already in hints, i.e., pks
    let agg_key = AggregateKey::<E>::new(pk, &params);
    // Here, however, an encryptor could choose to exclude parties / add their own parties other than the LoE
    // Likely better to just publish the public key and encryptor computes it themselves

    // There is a polynomial evaluated at a point \tau, \tau^2, \tau^3
    // The coefficients of the polynomial is interpolated on-the-fly with each of the secret keys by computing \sum sk_i L_i(\tau) with L_i lagrange coefficients
    // cool, but can we decrypt with a different set of signers??
    // Yes: you do the aggregation for all the signers, and can decrypt if you provide a threshold number of partial sigs

    let threshold = 5; // do not account for dummy party
    let round = 5;
    let plaintext = format!("This ciphertext will be decrypted at round {}", round);
    let ct_tlock = tlock_encrypt(plaintext.as_bytes(), round, threshold, &agg_key, &params);

    let mut current_round = 1u64;
    loop {
        let gamma_g2 = round_to_gamma_g2(current_round);

        let mut partial_decs = Vec::new();
        let mut parties = Vec::new();
        let mut selector = [false; N];
        for _ in 0..threshold {
            // get unique random party
            let party = loop {
                let p = OsRng.gen_range(1..sk.len());
                if !selector[p] {
                    break p;
                }
            };

            partial_decs.push(sk[party].partial_decryption_gamma_g2(&gamma_g2));
            parties.push(party);
            selector[party] = true;
        }

        // We now have t partial decs, try to decrypt
        // Executed by whatever party that wants to decrypt tlock ciphertext
        let dec_key = agg_dec(
            &partial_decs,
            &parties,
            &ct_tlock.silent_ct,
            &agg_key,
            &params,
        );
        if let Ok(dec_key) = dec_key {
            println!("Ciphertext successfully decrypted at round {current_round}:");

            let chacha_key = dec_key.hash::<Keccak256>();
            let cipher = ChaCha20Poly1305::new(&chacha_key);
            let plaintext = cipher
                .decrypt(
                    GenericArray::from_slice(&ct_tlock.nonce),
                    ct_tlock.ct.as_ref(),
                )
                .expect("chacha ciphertext was altered");
            // Decryption should always work once the decryption key is obtained
            // due to correctness
            println!("{}", std::str::from_utf8(&plaintext).unwrap());
        } else {
            println!("Failed to decrypt ciphertext at round {current_round}");
        }

        sleep(Duration::from_millis(500));
        current_round += 1;
    }
}
