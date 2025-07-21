#[cfg(not(feature = "serde"))]
fn main() {
    println!("serde feature is required.");
}

#[cfg(feature = "serde")]
fn main() -> anyhow::Result<()> {
    use std::fs::File;
    use std::io::Write;

    use ark_ec::pairing::Pairing;
    use ark_poly::univariate::DensePolynomial;
    use silent_threshold_encryption::{
        kzg::KZG10,
        setup::{AggregateKey, SecretKey},
    };

    type E = ark_bn254::Bn254;
    type UniPoly = DensePolynomial<<E as Pairing>::ScalarField>;

    let mut rng = ark_std::test_rng();

    // Number of parties + one dummy party
    const N: usize = 31 + 1;
    const _: () = assert!(N != 0 && (N & (N - 1)) == 0); // N non-zero and power of 2

    let params = KZG10::<E, UniPoly>::setup(N, &mut rng).unwrap();

    {
        let params_ser = serde_json::to_string(&params).unwrap();
        println!("Params:");
        println!("{}", params_ser);
        let mut f = File::create("tess_universal_params.json")?;
        _ = f.write(params_ser.as_bytes())?;
        f.flush()?;
    }

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
        pk.push(sk[i].get_pk(i, &params, N)?);

        let ski = serde_json::to_string(&sk[i]).unwrap();
        let pki = serde_json::to_string(&pk[i]).unwrap();
        let mut fsk = File::create(format!("tess_sk_{i}.json"))?;
        let mut fpk = File::create(format!("tess_pk_{i}.json"))?;
        _ = fsk.write(ski.as_bytes())?;
        _ = fpk.write(pki.as_bytes())?;
        fsk.flush()?;
        fpk.flush()?;
    }

    // Publish the public keys and hints to other parties
    // not sure what can of model is assumed, likely a verifiable broadcast from the games

    // This is basically preprocess, compute the sums for ak, ek. Rest of parameters are already in hints, i.e., pks
    let _agg_key = AggregateKey::<E>::new(pk, &params);

    Ok(())
}
