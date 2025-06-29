# Post-Quantum Signature Scheme Benchmarks

## System Specs

- **CPU**: Apple M3 Max (14 cores, 14 threads)
- **Memory**: 96 GB
- **Architecture**: arm64
- **OS**: macOS 15.5
- **Kernel**: 24.5.0
- **Rust**: 1.86.0 (05f9846f8 2025-03-31)
- **Cargo**: 1.86.0 (adf9b6ad1 2025-02-28)

## Size Comparison

| Variant | Pub Key (bytes) | Sig (bytes) |
|---------|-----------------|-------------|
| ECDSA | 33 (compressed) | 64 |
| ML-DSA-44 | 1312 (39.8x ECDSA) | 2420 (37.8x ECDSA) |
| ML-DSA-65 | 1952 (59.2x ECDSA) | 3309 (51.7x ECDSA) |
| ML-DSA-87 | 2592 (78.5x ECDSA) | 4627 (72.3x ECDSA) |
| SHA2-128s | 32 (1.0x ECDSA) | 7856 (122.8x ECDSA) |
| SHA2-128f | 32 (1.0x ECDSA) | 17088 (267.0x ECDSA) |
| SHA2-192s | 48 (1.5x ECDSA) | 16224 (253.5x ECDSA) |
| SHA2-192f | 48 (1.5x ECDSA) | 35664 (557.2x ECDSA) |
| SHA2-256s | 64 (1.9x ECDSA) | 29792 (465.5x ECDSA) |
| SHA2-256f | 64 (1.9x ECDSA) | 49856 (779.0x ECDSA) |

### Summary

- ML-DSA balances a more moderate increase in public-key size and signature size compared to ECDSA.
- SLH-DSA keeps public keys nice and small but at the cost of very large signatures (10–50 KB).

Choice will largely depend on the application and constraints.

## Running the Size Comparison

To run the size comparison:

```bash
cargo run
```

## Benchmark Results

### ECDSA - secp256k1 (K-256) (Baseline)

| Variant | Operation | Time (mean) |
|---------|-----------|-------------|
| ECDSA | Key Generation | 29.822 µs |
| ECDSA | Signing | 30.490 µs |
| ECDSA | Verification | 49.811 µs |

### ML-DSA Variants

| Variant | Operation | Time (mean) |
|---------|-----------|-------------|
| ML-DSA-44 | Key Generation | 120.33 µs |
| ML-DSA-44 | Signing | 446.58 µs |
| ML-DSA-44 | Verification | 23.071 µs |
| ML-DSA-65 | Key Generation | 216.66 µs |
| ML-DSA-65 | Signing | 258.54 µs |
| ML-DSA-65 | Verification | 33.080 µs |
| ML-DSA-87 | Key Generation | 322.89 µs |
| ML-DSA-87 | Signing | 450.90 µs |
| ML-DSA-87 | Verification | 48.162 µs |

### SLH-DSA Variants

| Variant | Operation | Time (mean) |
|---------|-----------|-------------|
| SHA2-128s | Key Generation | 78.857 ms |
| SHA2-128s | Signing | 601.07 ms |
| SHA2-128s | Verification | 585.83 µs |
| SHA2-128f | Key Generation | 1.2359 ms |
| SHA2-128f | Signing | 28.583 ms |
| SHA2-128f | Verification | 1.7090 ms |
| SHA2-192s | Key Generation | 114.68 ms |
| SHA2-192s | Signing | 1.0399 s |
| SHA2-192s | Verification | 860.23 µs |
| SHA2-192f | Key Generation | 1.7356 ms |
| SHA2-192f | Signing | 45.729 ms |
| SHA2-192f | Verification | 2.3920 ms |
| SHA2-256s | Key Generation | 73.360 ms |
| SHA2-256s | Signing | 1.2020 s |
| SHA2-256s | Verification | 1.2094 ms |
| SHA2-256f | Key Generation | 4.6854 ms |
| SHA2-256f | Signing | 96.998 ms |
| SHA2-256f | Verification | 2.5678 ms |

### Grouped by OP

#### Key Generation

| Variant | Time (mean) |
|---------|-------------|
| ECDSA | 29.822 µs |
| ML-DSA-44 | 120.33 µs |
| ML-DSA-65 | 216.66 µs |
| ML-DSA-87 | 322.89 µs |
| SHA2-128s | 78.857 ms |
| SHA2-128f | 1.2359 ms |
| SHA2-192s | 114.68 ms |
| SHA2-192f | 1.7356 ms |
| SHA2-256s | 73.360 ms |
| SHA2-256f | 4.6854 ms |

#### Signing

| Variant | Time (mean) |
|---------|-------------|
| ECDSA | 30.490 µs |
| ML-DSA-44 | 446.58 µs |
| ML-DSA-65 | 258.54 µs |
| ML-DSA-87 | 450.90 µs |
| SHA2-128s | 601.07 ms |
| SHA2-128f | 28.583 ms |
| SHA2-192s | 1.0399 s |
| SHA2-192f | 45.729 ms |
| SHA2-256s | 1.2020 s |
| SHA2-256f | 96.998 ms |

#### Verification

| Variant | Time (mean) |
|---------|-------------|
| ECDSA | 49.811 µs |
| ML-DSA-44 | 23.071 µs |
| ML-DSA-65 | 33.080 µs |
| ML-DSA-87 | 48.162 µs |
| SHA2-128s | 585.83 µs |
| SHA2-128f | 1.7090 ms |
| SHA2-192s | 860.23 µs |
| SHA2-192f | 2.3920 ms |
| SHA2-256s | 1.2094 ms |
| SHA2-256f | 2.5678 ms |

### Summary

1. **ML-DSA Performance**:
   - All ML-DSA variants show consistent performance across operations
   - Verification is significantly faster than signing across all variants
   - ML-DSA-65 offers a good balance between security and performance

2. **SLH-DSA Performance**:
   - Fast variants (f) are significantly faster than small variants (s) (no surprises there)
   - Key generation is much slower in small variants
   - Signing operations are the most time-consuming
   - Verification is relatively fast across all variants

3. **Comparison**:
   - ML-DSA is generally faster than SLH-DSA across all operations

## Running the Benchmarks

To run the benchmarks:

```bash
cargo bench
```

To get system specifications:

```bash
./get_specs.sh
``` 
