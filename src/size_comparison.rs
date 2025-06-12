use k256::ecdsa::{Signature, SigningKey, VerifyingKey};
use ml_dsa::{KeyGen, MlDsa44, MlDsa65, MlDsa87, signature::Signer};
use rand_core::OsRng;
use signature::Keypair;
use slh_dsa::{
    Sha2_128f, Sha2_128s, Sha2_192f, Sha2_192s, Sha2_256f, Sha2_256s, SigningKey as SlhSigningKey,
};

pub fn print_size_comparison() {
    let mut rng = OsRng;
    let msg = b"Hello world";

    // ECDSA sizes (baseline)
    let sk = SigningKey::random(&mut rng);
    let vk = VerifyingKey::from(&sk);
    let sig: Signature = sk.sign(msg);
    let ecdsa_pk_size = vk.to_sec1_bytes().len();
    let ecdsa_sig_size = sig.to_bytes().len();

    println!("\n=== Size Comparisons ===\n");
    println!("ECDSA (baseline):");
    println!("  Compressed Public Key: {} bytes", ecdsa_pk_size);
    println!("  Signature: {} bytes\n", ecdsa_sig_size);

    // ML-DSA sizes
    println!("ML-DSA Variants:");

    // ML-DSA-44
    let kp44 = MlDsa44::key_gen(&mut rng);
    let sig44 = kp44.signing_key().sign(msg);
    let pk_size44 = kp44.verifying_key().encode().len();
    let sig_size44 = sig44.encode().len();
    println!("ML-DSA-44:");
    println!(
        "  Public Key: {} bytes ({:.1}x ECDSA)",
        pk_size44,
        pk_size44 as f64 / ecdsa_pk_size as f64
    );
    println!(
        "  Signature: {} bytes ({:.1}x ECDSA)",
        sig_size44,
        sig_size44 as f64 / ecdsa_sig_size as f64
    );

    // ML-DSA-65
    let kp65 = MlDsa65::key_gen(&mut rng);
    let sig65 = kp65.signing_key().sign(msg);
    let pk_size65 = kp65.verifying_key().encode().len();
    let sig_size65 = sig65.encode().len();
    println!("ML-DSA-65:");
    println!(
        "  Public Key: {} bytes ({:.1}x ECDSA)",
        pk_size65,
        pk_size65 as f64 / ecdsa_pk_size as f64
    );
    println!(
        "  Signature: {} bytes ({:.1}x ECDSA)",
        sig_size65,
        sig_size65 as f64 / ecdsa_sig_size as f64
    );

    // ML-DSA-87
    let kp87 = MlDsa87::key_gen(&mut rng);
    let sig87 = kp87.signing_key().sign(msg);
    let pk_size87 = kp87.verifying_key().encode().len();
    let sig_size87 = sig87.encode().len();
    println!("ML-DSA-87:");
    println!(
        "  Public Key: {} bytes ({:.1}x ECDSA)",
        pk_size87,
        pk_size87 as f64 / ecdsa_pk_size as f64
    );
    println!(
        "  Signature: {} bytes ({:.1}x ECDSA)",
        sig_size87,
        sig_size87 as f64 / ecdsa_sig_size as f64
    );

    // SLH-DSA sizes
    println!("\nSLH-DSA:");

    // SLH-DSA-128s
    let sk128s = SlhSigningKey::<Sha2_128s>::new(&mut rng);
    let vk128s = sk128s.verifying_key();
    let sig128s = sk128s.sign(msg);
    let pk_size128s = vk128s.to_bytes().len();
    let sig_size128s = sig128s.to_bytes().len();
    println!("SLH-DSA-128s:");
    println!(
        "  Public Key: {} bytes ({:.1}x ECDSA)",
        pk_size128s,
        pk_size128s as f64 / ecdsa_pk_size as f64
    );
    println!(
        "  Signature: {} bytes ({:.1}x ECDSA)",
        sig_size128s,
        sig_size128s as f64 / ecdsa_sig_size as f64
    );

    // SLH-DSA-128f
    let sk128f = SlhSigningKey::<Sha2_128f>::new(&mut rng);
    let vk128f = sk128f.verifying_key();
    let sig128f = sk128f.sign(msg);
    let pk_size128f = vk128f.to_bytes().len();
    let sig_size128f = sig128f.to_bytes().len();
    println!("SLH-DSA-128f:");
    println!(
        "  Public Key: {} bytes ({:.1}x ECDSA)",
        pk_size128f,
        pk_size128f as f64 / ecdsa_pk_size as f64
    );
    println!(
        "  Signature: {} bytes ({:.1}x ECDSA)",
        sig_size128f,
        sig_size128f as f64 / ecdsa_sig_size as f64
    );

    // SLH-DSA-192s
    let sk192s = SlhSigningKey::<Sha2_192s>::new(&mut rng);
    let vk192s = sk192s.verifying_key();
    let sig192s = sk192s.sign(msg);
    let pk_size192s = vk192s.to_bytes().len();
    let sig_size192s = sig192s.to_bytes().len();
    println!("SLH-DSA-192s:");
    println!(
        "  Public Key: {} bytes ({:.1}x ECDSA)",
        pk_size192s,
        pk_size192s as f64 / ecdsa_pk_size as f64
    );
    println!(
        "  Signature: {} bytes ({:.1}x ECDSA)",
        sig_size192s,
        sig_size192s as f64 / ecdsa_sig_size as f64
    );

    // SLH-DSA-192f
    let sk192f = SlhSigningKey::<Sha2_192f>::new(&mut rng);
    let vk192f = sk192f.verifying_key();
    let sig192f = sk192f.sign(msg);
    let pk_size192f = vk192f.to_bytes().len();
    let sig_size192f = sig192f.to_bytes().len();
    println!("SLH-DSA-192f:");
    println!(
        "  Public Key: {} bytes ({:.1}x ECDSA)",
        pk_size192f,
        pk_size192f as f64 / ecdsa_pk_size as f64
    );
    println!(
        "  Signature: {} bytes ({:.1}x ECDSA)",
        sig_size192f,
        sig_size192f as f64 / ecdsa_sig_size as f64
    );

    // SLH-DSA-256s
    let sk256s = SlhSigningKey::<Sha2_256s>::new(&mut rng);
    let vk256s = sk256s.verifying_key();
    let sig256s = sk256s.sign(msg);
    let pk_size256s = vk256s.to_bytes().len();
    let sig_size256s = sig256s.to_bytes().len();
    println!("SLH-DSA-256s:");
    println!(
        "  Public Key: {} bytes ({:.1}x ECDSA)",
        pk_size256s,
        pk_size256s as f64 / ecdsa_pk_size as f64
    );
    println!(
        "  Signature: {} bytes ({:.1}x ECDSA)",
        sig_size256s,
        sig_size256s as f64 / ecdsa_sig_size as f64
    );

    // SLH-DSA-256f
    let sk256f = SlhSigningKey::<Sha2_256f>::new(&mut rng);
    let vk256f = sk256f.verifying_key();
    let sig256f = sk256f.sign(msg);
    let pk_size256f = vk256f.to_bytes().len();
    let sig_size256f = sig256f.to_bytes().len();
    println!("SLH-DSA-256f:");
    println!(
        "  Public Key: {} bytes ({:.1}x ECDSA)",
        pk_size256f,
        pk_size256f as f64 / ecdsa_pk_size as f64
    );
    println!(
        "  Signature: {} bytes ({:.1}x ECDSA)",
        sig_size256f,
        sig_size256f as f64 / ecdsa_sig_size as f64
    );
}
