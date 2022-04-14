# poseidon-rs [![Crates.io](https://img.shields.io/crates/v/poseidon-rs.svg)](https://crates.io/crates/poseidon-rs) [![Test](https://github.com/arnaucube/poseidon-rs/workflows/Test/badge.svg)](https://github.com/arnaucube/poseidon-rs/actions?query=workflow%3ATest)

Poseidon hash implementation in Rust, a zkSNARK friendly hash function.

https://eprint.iacr.org/2019/458.pdf

Compatible with the Poseidon implementations:
- Sage (reference implementation): https://extgit.iaik.tugraz.at/krypto/hadeshash
- Go: https://github.com/iden3/go-iden3-crypto
- Js & circom: https://github.com/iden3/circomlib

## Notes

Modified to work with BLS12-377 curve.

Constants have been updated for the case of use of a binary merkle tree

