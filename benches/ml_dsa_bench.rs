use criterion::{Criterion, black_box, criterion_group, criterion_main};
use ml_dsa::{
    KeyGen, MlDsa44, MlDsa65, MlDsa87,
    signature::{Signer, Verifier},
};
use rand_core::OsRng;

macro_rules! bench_ml_dsa_variant {
    ($c:expr, $name:expr, $variant:ty) => {
        let mut group = $c.benchmark_group($name);
        group.measurement_time(std::time::Duration::from_secs(10));
        group.sample_size(10);

        let mut rng = OsRng;
        let kp = <$variant>::key_gen(&mut rng);
        let msg = b"Hello world";

        group.bench_function("key_gen", |b| {
            b.iter(|| {
                let mut rng = OsRng;
                black_box(<$variant>::key_gen(&mut rng))
            })
        });

        group.bench_function("sign", |b| b.iter(|| black_box(kp.signing_key().sign(msg))));

        group.bench_function("verify", |b| {
            let sig = kp.signing_key().sign(msg);
            b.iter(|| black_box(kp.verifying_key().verify(msg, &sig)))
        });

        group.finish();
    };
}

fn bench_ml_dsa(c: &mut Criterion) {
    bench_ml_dsa_variant!(c, "ml_dsa_44", MlDsa44);
    bench_ml_dsa_variant!(c, "ml_dsa_65", MlDsa65);
    bench_ml_dsa_variant!(c, "ml_dsa_87", MlDsa87);
}

criterion_group!(benches, bench_ml_dsa);
criterion_main!(benches);
