# FRI basic code in rust for exploration

## Introduction

Welcome to the FRI Basic Code project!

This project aims to provide a basic implementation of the FRI algorithm in Rust. FRI is a powerful algorithm used in various fields, including cryptography and error correction. By leveraging polynomial composition and commitment schemes, FRI allows for efficient verification and evaluation of polynomials.

In this project, you will find four modules that play crucial roles in the FRI algorithm.

**field_provider_v1** : The `field_provider_v1` module enables you to define a finite field for the project. Currently the injection is hard coded.

**polynome** : The `polynome` module provides a basic implementation for evaluating a polynomial for this basic FRI algo.

**fri_code_layer** : The `fri_code_layer` module implements the commit and decomit operations, which are fundamental to the FRI algorithm.

**channel** : The `channel` module provides an interface for interacting with the verifier, allowing you to supply the beta value challenge for folding and also manage some query verification.

To get started :

- Run the tests globally :

```rust
cargo test polynome
```

- Run the tests for each module :
(See improvment section as a panic bug persist in the code and not yet fixed)

```rust
cargo test fri_code_layer
```

```rust
cargo test channel
```

- Simply run the `main` function, which demonstrates a step-by-step process of committing and decomitting on a polynomial composition. (See the log). You can execute the program by running :

```rust
cargo run
```

Please note that there are still some cleaning tasks remaining, and one test is currently failing, specifically the one that checks the symmetry of the domain. Additionally, you can find the slides for this project in the `slide` directory.

## Improvment

- Some part of code should be refactorized for more lisibility
- Some code can be optimized and more secured by better usage of ownership and borrowing
- A panic bug should be fixed on the domain evaluation as the symmetry test do not pass. The coset_offset is probably false so it need some investigation

## Versions

Used with :

```code
cargo --version
cargo 1.80.1 (376290515 2024-07-16)
```

## Notes

This project use those dependancies : 

```code
[dependencies]
ff = { version = "0.13.0", features = ["derive"] }
hex = "0.4.3"
rand = "0.8.5"
rs_merkle = "1.4.2"
```

**hex** & **rand** added for some facilities

**ff** is for finite fields tooling : <https://crates.io/crates/ff>

**rs_merkleis** for Merkle tree tooling : <https://docs.rs/rs_merkle/latest/rs_merkle/>