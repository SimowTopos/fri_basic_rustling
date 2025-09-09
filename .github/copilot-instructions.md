# FRI Basic Rustling - GitHub Copilot Instructions

**FRI Basic Rustling** is a Rust educational library implementing the FRI (Fast Reed-Solomon Interactive) algorithm for cryptographic polynomials. The library includes polynomial evaluation, commitment schemes using Merkle trees, and interactive proof systems.

**ALWAYS reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.**

## Working Effectively

### Bootstrap and Build
- **Dependencies**: Rust toolchain is already installed. Current versions:
  - `rustc 1.89.0`
  - `cargo 1.89.0`
- **Build the project**:
  - `cargo build` - Development build (22 seconds). NEVER CANCEL.
  - `cargo build --release` - Release build (9 seconds, optimized). NEVER CANCEL.
- **Run the demonstration**:
  - `cargo run` - Executes the FRI commitment/decommitment demo (< 1 second).

### Testing
- **Run all tests**: `cargo test` (< 1 second). **EXPECT 1 FAILING TEST** - this is a known issue.
- **Run module-specific tests**:
  - `cargo test polynome` - Polynomial tests (< 1 second, 5 tests pass)
  - `cargo test fri_code_layer` - FRI layer tests (< 1 second, 6 pass, 1 FAILS - known symmetry issue)
  - `cargo test channel` - Channel tests (< 1 second, 1 test passes)
- **Known failing test**: `test_eval_domain_symetry` fails due to coset_offset calculation issue (documented in README).

### Code Quality
- **Format code**: `cargo fmt` - Formats all Rust code
- **Check formatting**: `cargo fmt --check` - Validates formatting without changes
- **Lint code**: `cargo clippy` - Static analysis with warnings (currently shows 7 warnings, not errors)
- **Fix lint issues**: `cargo clippy --fix --allow-dirty --allow-staged` - Automatically fixes some issues

### Validation Scenarios
After making changes, **ALWAYS**:
1. Run `cargo build` to ensure compilation
2. Run `cargo run` to verify the FRI demo works correctly:
   - Should display "COMMITMENT PHASE" followed by Merkle roots
   - Should display "DECOMMITMENT PHASE" with proof paths and hashes
   - Should complete without panics (except for known symmetry test)
3. Run `cargo test` and verify only the known `test_eval_domain_symetry` test fails
4. Run `cargo fmt` and `cargo clippy` before committing

## Architecture and Key Components

### Core Modules (755 lines of Rust source code total)
- **`field_provider_v1.rs`** (7 lines): Defines `FieldElement` using BLS12-381 prime field
- **`polynome.rs`** (224 lines): Polynomial operations and evaluation
- **`fri_code_layer.rs`** (404 lines): FRI commitment/decommitment implementation with Merkle trees  
- **`channel.rs`** (63 lines): Verifier interaction interface for challenges
- **`main.rs`** (57 lines): Demonstration of full FRI protocol

### Dependencies (Cargo.toml)
```toml
[dependencies]
ff = { version = "0.13.0", features = ["derive"] }  # Finite field arithmetic
hex = "0.4.3"                                       # Hex encoding utilities
rand = "0.8.5"                                      # Random number generation
rs_merkle = "1.4.2"                                # Merkle tree implementation
```

### Repository Structure
```
/home/runner/work/fri_basic_rustling/fri_basic_rustling/
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
├── README.md           # Project documentation
├── src/                # Source code
│   ├── main.rs         # FRI demo application
│   ├── field_provider_v1.rs
│   ├── polynome.rs
│   ├── fri_code_layer.rs
│   └── channel.rs
└── slide/              # Contains FRI_ALAOUI_RD_CRYPTOGRAPHY-V1-LAST.pdf
    └── FRI_ALAOUI_RD_CRYPTOGRAPHY-V1-LAST.pdf
```

## Common Tasks and Expected Behavior

### Running the FRI Demo
```bash
cargo run
```
**Expected output**:
- "COMMITMENT PHASE" with domain size 48 and Merkle commitment roots
- "DECOMMITMENT PHASE" with 20 query proofs showing layer evaluations and authentication paths
- Completes successfully demonstrating FRI interactive proof

### Working with Polynomials
- The demo uses a polynomial with coefficients `[1, 2, 3, 3, 3, 3, 3]` (degree 6)
- Domain size is 48 (8 × polynomial degree for proper FRI operation)
- All polynomial operations use finite field arithmetic over BLS12-381

### Known Issues and Limitations
- **Symmetry test failure**: `test_eval_domain_symetry` fails due to incorrect coset_offset calculation
- **Code style**: Some clippy warnings exist but don't prevent compilation
- **Educational purpose**: This is for learning - contact author for production implementation

### Performance Characteristics
- **Initial build**: ~22 seconds (includes dependency compilation)
- **Incremental builds**: 2-4 seconds  
- **Tests**: < 1 second for all test suites
- **Demo execution**: < 0.1 seconds

## Troubleshooting

### Build Issues
- If build fails, ensure Rust toolchain is current: `rustc --version`
- Clean build artifacts: `cargo clean && cargo build`

### Test Failures
- **Expected**: Only `test_eval_domain_symetry` should fail
- **Unexpected failures**: Check for breaking changes to finite field operations

### Code Quality Issues
- Run `cargo fmt` to fix formatting
- Address clippy warnings with `cargo clippy --fix --allow-dirty --allow-staged`
- The field_provider_v1 hash/equality warning can be ignored (derives from PrimeField macro)

## Development Guidelines

### Making Changes
1. **Always build first**: `cargo build` 
2. **Test your changes**: `cargo test`
3. **Validate the demo**: `cargo run` 
4. **Format code**: `cargo fmt`
5. **Check style**: `cargo clippy`

### Module Interactions
- `FieldElement` is the foundation type used throughout
- `Polynome` provides polynomial arithmetic over finite fields
- `FriCodeLayer` implements the core FRI protocol with Merkle tree commitments
- `Channel` simulates verifier challenges for the interactive proof
- Always validate polynomial operations against the finite field constraints

### Performance Considerations
- Release builds are significantly faster for cryptographic operations
- Test builds include debug symbols and are unoptimized
- FRI operations scale with polynomial degree and domain size

**Remember**: This is an educational implementation. Always verify the demo works after changes and expect the symmetry test to fail.