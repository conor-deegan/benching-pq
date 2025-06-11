use criterion::{black_box, criterion_group, criterion_main, Criterion};
use k256::{
    ecdsa::{SigningKey, VerifyingKey, signature::{Signer, Verifier}, Signature},
};
use rand_core::OsRng;

fn bench_ec_dsa(c: &mut Criterion) {
    let mut group = c.benchmark_group("ec_dsa_k256");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(10);

    let mut rng = OsRng;
    let msg = b"Hello world";

    group.bench_function("key_gen", |b| {
        b.iter(|| {
            let mut rng = OsRng;
            black_box(SigningKey::random(&mut rng))
        })
    });

    let sk = SigningKey::random(&mut rng);
    let vk = VerifyingKey::from(&sk);

    group.bench_function("sign", |b| {
        b.iter(|| {
            let sig: Signature = sk.sign(msg);
            black_box(sig)
        })
    });

    group.bench_function("verify", |b| {
        let sig: Signature = sk.sign(msg);
        b.iter(|| {
            black_box(vk.verify(msg, &sig))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_ec_dsa);
criterion_main!(benches);
