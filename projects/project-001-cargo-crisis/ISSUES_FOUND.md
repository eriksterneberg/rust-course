# Issues Found and Fixed

Document the issues you discover and how you fixed them. This is your incident report.

## Cargo.toml Issues

1. **Issue**: The version for the package logforge was wrong
   **Why it's wrong**: 0.1 is not a valid package
   **Fix**: Changed to 0.1.0

2. **Issue**: The edition was too old
   **Why it's wrong**: Not really wrong per say, but no reason to not use the latest version
   **Fix**: Changed to 2024

3. **Issue**: The task was to make the additional binaries callable using the names logforge-validate and logforge-cli
   **Why it's wrong**: There were no [[bin]] sections
   **Fix**: Added [[bin]] sections with the correct paths

## Project Structure Issues

1. **Issue**: The library code was in the main.rs file
   **Why it's wrong**: In order the binaries to be able to be compiled separately, they cannot depend on each other
   **Fix**: Move the library code to lib.rs

2. **Issue**: The binaries were not in the right folder: src/bin/
   **Why it's wrong**: I think rustc cannot find them if that is the case, at least not without specifying the deviating paths under the [[bin]] sections in Cargo.toml
   **Fix**: Moved the files

## Cargo.lock Decision

**Decision**: (Should Cargo.lock be committed or gitignored?) -- It should not, because this package is mainly about the library.

**Reasoning**:

## Verification Steps Completed

- [x] `cargo check` passes
- [x] `cargo build` produces all expected artifacts
- [x] `cargo test` all tests pass
- [x] `cargo doc` generates documentation
- [x] `cargo run --bin logforge-cli` works
- [x] `cargo run --bin logforge-validate` works

## Lessons Learned
Before committing your code, you need to run cargo check!
