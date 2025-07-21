use ark_ec::pairing::Pairing;
use ark_poly::univariate::DensePolynomial;
use criterion::{criterion_group, criterion_main, Criterion};
use silent_threshold_encryption::{
    encryption::encrypt,
    kzg::KZG10,
    setup::{AggregateKey, PublicKey, SecretKey},
};

type E = ark_bls12_381::Bls12_381;
type UniPoly381 = DensePolynomial<<E as Pairing>::ScalarField>;

fn bench_encrypt(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();
    let n = 8;
    let t = 2;
    let params = KZG10::<E, UniPoly381>::setup(n, &mut rng).unwrap();

    let mut sk: Vec<SecretKey<E>> = Vec::new();
    let mut pk: Vec<PublicKey<E>> = Vec::new();

    for i in 0..n {
        sk.push(SecretKey::<E>::new(&mut rng));
        pk.push(sk[i].get_pk(0, &params, n).unwrap())
    }

    let ak = AggregateKey::<E>::new(pk, &params);

    c.bench_function("encrypt", |b| b.iter(|| encrypt::<E>(&ak, t, &params)));
}

criterion_group!(benches, bench_encrypt);
criterion_main!(benches);
