# The Cargo Crisis: A Production Incident

Here's the deal. It's 2 AM and your pager just went off. The deployment pipeline for `logforge` - your company's internal logging library - is completely broken. Someone pushed changes without testing, the CI is red, and three other teams are blocked waiting for a fix because they depend on this crate.

You're the on-call engineer. The original author is on vacation in a place with no cell service (convenient, right?). You need to untangle this mess, and you need to do it now.

The good news: this is a Cargo problem, not a Rust problem. The code itself is fine - it's the project structure and configuration that got mangled. You know Cargo. You've got this.

## Learning Objectives

- [x] Demonstrate mastery of Cargo.toml configuration and dependency management
- [x] Correctly structure a project that is both a library AND a binary
- [x] Understand when to commit Cargo.lock (hint: this is a library)
- [x] Use `cargo check`, `cargo build`, `cargo test`, and `cargo doc` effectively
- [x] Navigate and fix common project structure mistakes

## The Situation

The `logforge` project is supposed to be:
1. A **library** that other teams import to handle structured logging
2. A **CLI binary** (`logforge-cli`) that ops uses to query and analyze logs
3. A **second binary** (`logforge-validate`) that validates log format configurations

Someone refactored the project structure and broke... everything. The Cargo.toml has issues. The file layout is wrong. The tests don't even run.

## Your Mission

### Phase 1: Triage

- [x] Run `cargo check` and understand ALL the errors (don't just fix blindly)
- [x] Identify what's wrong with the Cargo.toml
- [x] Identify what's wrong with the project structure
- [x] Document the issues you find (add comments to ISSUES_FOUND.md)

### Phase 2: Fix the Configuration

- [x] Fix the Cargo.toml to properly declare this as both a library and binary package
- [x] Ensure the edition is set to 2021 (someone downgraded it for "compatibility")
- [x] Fix the dependency declarations (check for version specification issues)
- [x] Ensure the package metadata is correct (name, version, description)

### Phase 3: Fix the Structure

- [x] Move files to their correct locations per Cargo conventions
- [x] Ensure `src/lib.rs` is the library entry point
- [x] Ensure `src/main.rs` OR `src/bin/` contains the binaries (your choice, but be consistent)
- [x] Make sure the library code is importable by the binaries

### Phase 4: Verify the Fix

- [x] `cargo check` passes with NO warnings
- [x] `cargo build` produces the library AND both binaries
- [x] `cargo test` runs and passes
- [x] `cargo doc --open` generates documentation without errors
- [x] Verify you can run both binaries: `cargo run --bin logforge-cli` and `cargo run --bin logforge-validate`

### Phase 5: Clean Up

- [x] Decide: should Cargo.lock be committed for this project? Add or remove from .gitignore accordingly
- [x] Add a brief explanation to ISSUES_FOUND.md about your Cargo.lock decision
- [x] Ensure `target/` is in .gitignore (it better be)

## Success Criteria

When you're done:
1. `cargo check` - zero errors, zero warnings
2. `cargo build --release` - produces `logforge-cli` and `logforge-validate` binaries
3. `cargo test` - all tests pass
4. `cargo doc` - generates clean documentation
5. The library can be imported by external crates (the entry point is `src/lib.rs`)
6. ISSUES_FOUND.md documents what was broken and why your fixes are correct

## Why This Matters

Every single one of these mistakes happens in real codebases. I've seen production deployments blocked because someone:
- Put library code in `main.rs` instead of `lib.rs`
- Used wildcard versions and a breaking change came through
- Forgot to add a binary to `[[bin]]` in Cargo.toml
- Committed `Cargo.lock` for a library, causing dependency hell downstream

Cargo is the foundation. If you don't understand project structure and configuration, you will create these problems for your future self and your teammates.

Now fix this mess and get those teams unblocked.

## Hints (Use Only If Stuck)

<details>
<summary>Hint 1: Library vs Binary Structure</summary>

A package can have:
- ONE library crate (entry point: `src/lib.rs`)
- MULTIPLE binary crates (entry point: `src/main.rs` for default, or `src/bin/*.rs` for additional)

If you want a default binary AND a library, you need both `src/lib.rs` and `src/main.rs`.
</details>

<details>
<summary>Hint 2: Cargo.toml [[bin]] sections</summary>

Additional binaries beyond the default need `[[bin]]` sections:
```toml
[[bin]]
name = "my-tool"
path = "src/bin/my_tool.rs"
```
</details>

<details>
<summary>Hint 3: Cargo.lock for Libraries</summary>

Libraries should NOT commit Cargo.lock. Why? Because consumers of your library need to resolve dependencies themselves. Your locked versions might conflict with theirs.

Binaries SHOULD commit Cargo.lock for reproducible builds.

This package is primarily a library, so...
</details>

## Time Estimate

30-45 minutes if you're methodical. Longer if you try to fix things without understanding them first. Trust me - read the errors, understand the structure, then fix.

Good luck. The other teams are counting on you.
