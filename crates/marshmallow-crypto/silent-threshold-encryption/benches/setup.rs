use ark_ec::pairing::Pairing;
use ark_poly::univariate::DensePolynomial;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use silent_threshold_encryption::{kzg::KZG10, setup::SecretKey};

type E = ark_bls12_381::Bls12_381;
type UniPoly381 = DensePolynomial<<E as Pairing>::ScalarField>;

fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup");
    group.sample_size(10);
    let mut rng = ark_std::test_rng();
    for size in 3..=10 {
        let n = 1 << size; // actually n-1 total parties. one party is a dummy party that is always true
        let params = KZG10::<E, UniPoly381>::setup(n, &mut rng).unwrap();

        let sk = SecretKey::<E>::new(&mut rng);

        group.bench_with_input(BenchmarkId::from_parameter(n), &params, |b, inp| {
            b.iter(|| sk.get_pk(0, inp, n).unwrap());
        });
    }

    group.finish();
}

criterion_group!(benches, bench_setup);
criterion_main!(benches);
