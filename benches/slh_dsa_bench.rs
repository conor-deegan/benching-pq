use criterion::{Criterion, black_box, criterion_group, criterion_main};
use ml_dsa::signature::{Keypair, SignerMut};
use rand_core::OsRng;
use slh_dsa::signature::Verifier;
use slh_dsa::{
    Sha2_128f, Sha2_128s, Sha2_192f, Sha2_192s, Sha2_256f, Sha2_256s, SigningKey, VerifyingKey,
};

macro_rules! bench_slh_dsa_variant {
    ($c:expr, $name:expr, $variant:ty) => {
        let mut group = $c.benchmark_group($name);
        group.measurement_time(std::time::Duration::from_secs(10));
        group.sample_size(10);

        let mut rng = OsRng;
        let msg = b"Hello world";

        group.bench_function("key_gen", |b| {
            b.iter(|| {
                let mut rng = OsRng;
                black_box(SigningKey::<$variant>::new(&mut rng))
            })
        });

        let mut sk = SigningKey::<$variant>::new(&mut rng);
        let vk = sk.verifying_key();

        group.bench_function("sign", |b| b.iter(|| black_box(sk.sign(msg))));

        group.bench_function("verify", |b| {
            let sig = sk.sign(msg);
            let vk_bytes = vk.to_bytes();
            let vk_deserialized = VerifyingKey::<$variant>::try_from(&vk_bytes[..]).unwrap();
            b.iter(|| black_box(vk_deserialized.verify(msg, &sig)))
        });

        group.finish();
    };
}

fn bench_slh_dsa(c: &mut Criterion) {
    bench_slh_dsa_variant!(c, "slh_dsa_sha2_128s", Sha2_128s);
    bench_slh_dsa_variant!(c, "slh_dsa_sha2_128f", Sha2_128f);
    bench_slh_dsa_variant!(c, "slh_dsa_sha2_192s", Sha2_192s);
    bench_slh_dsa_variant!(c, "slh_dsa_sha2_192f", Sha2_192f);
    bench_slh_dsa_variant!(c, "slh_dsa_sha2_256s", Sha2_256s);
    bench_slh_dsa_variant!(c, "slh_dsa_sha2_256f", Sha2_256f);
}

criterion_group!(benches, bench_slh_dsa);
criterion_main!(benches);
