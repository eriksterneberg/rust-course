# The Crate Detective: Navigating the Ecosystem

Here's the deal. You've been asked to build a small utility that needs to handle JSON, make HTTP requests, and parse command-line arguments. Simple enough, right?

Wrong. There are literally dozens of crates that do each of these things. Some are abandoned. Some are over-engineered. Some are perfect for your use case. And if you pick wrong, you're going to be maintaining workarounds for years, or worse - migrating to a different crate mid-project when you discover yours hasn't been updated since 2019.

This isn't hypothetical. I've seen teams burn weeks migrating off abandoned crates. I've seen production systems vulnerable because nobody checked if the dependency was maintained. The Rust ecosystem is incredible, but it requires you to be a detective.

Your job: investigate, evaluate, and make informed decisions about dependencies.

## Learning Objectives

- [ ] Navigate crates.io to find and evaluate crates
- [ ] Use docs.rs to understand crate APIs before adding dependencies
- [ ] Evaluate crate health (maintenance, downloads, recent updates)
- [ ] Understand version specification syntax and make appropriate choices
- [ ] Use `cargo add` and manual Cargo.toml editing effectively
- [ ] Understand feature flags and when to use them

## The Mission

You're building `webprobe` - a CLI tool that:
1. Takes a URL as a command-line argument
2. Makes an HTTP GET request to that URL
3. Parses the JSON response
4. Prints specific fields in a formatted way

But before you write any code, you need to choose your dependencies wisely.

## Phase 1: Investigation

For each category below, research the options and document your findings.

### JSON Parsing

- [ ] Search crates.io for JSON crates
- [ ] Compare at least `serde_json` and `json` crates
- [ ] Check docs.rs for API documentation quality
- [ ] Record: downloads, last update, GitHub stars/issues
- [ ] Document your choice and reasoning in DECISIONS.md

### HTTP Client

- [ ] Search crates.io for HTTP client crates
- [ ] Compare at least `reqwest`, `ureq`, and `attohttpc`
- [ ] Note which ones are async vs blocking (this matters for a simple CLI!)
- [ ] Check for TLS/SSL support and what features are required
- [ ] Document your choice and reasoning in DECISIONS.md

### Command-Line Parsing

- [ ] Search crates.io for argument parsing crates
- [ ] Compare at least `clap`, `structopt`, and `argh`
- [ ] Note: `structopt` is now part of `clap` - what does that tell you?
- [ ] Consider: do you need a full framework or would `std::env::args` suffice?
- [ ] Document your choice and reasoning in DECISIONS.md

## Phase 2: Setup

- [ ] Create the project using `cargo new webprobe`
- [ ] Add your chosen dependencies to Cargo.toml
- [ ] Use appropriate version specifications (NOT wildcards!)
- [ ] If a crate has features, decide which ones you need
- [ ] Run `cargo build` to verify everything resolves correctly

## Phase 3: Implementation

Create a minimal working implementation:

- [ ] Parse a URL from command-line arguments
- [ ] Make an HTTP GET request (use httpbin.org/json as a test endpoint)
- [ ] Parse the JSON response
- [ ] Print at least one field from the response
- [ ] Handle errors gracefully (no panics on bad input)

## Phase 4: Documentation

- [ ] Run `cargo doc --open` to see the generated docs
- [ ] Verify your dependencies' documentation is accessible
- [ ] Add a doc comment to your main function explaining usage

## Success Criteria

1. DECISIONS.md contains well-reasoned justifications for each dependency choice
2. Cargo.toml uses proper version specifications
3. `cargo build` succeeds
4. `cargo run -- https://httpbin.org/json` produces readable output
5. `cargo run` (no args) shows a helpful error message, not a panic
6. `cargo doc` generates documentation without warnings

## Why This Matters

The Rust ecosystem is a superpower, but only if you use it wisely. Every dependency is:
- Code you didn't write but are responsible for
- A potential security surface
- A maintenance burden if abandoned
- A learning curve for your team

I've seen production incidents caused by:
- Depending on a crate that was a single person's hobby project (they got busy)
- Using a crate with known vulnerabilities because nobody checked
- Picking an async HTTP client for a simple CLI (now you need a runtime)
- Not understanding feature flags and bloating binary size 10x

Be deliberate. Be a detective. Your future self will thank you.

## Resources

- https://crates.io - The official package registry
- https://docs.rs - Auto-generated documentation for all crates
- https://lib.rs - Alternative crate search with better categorization
- https://blessed.rs - Curated list of recommended crates

## Hints

<details>
<summary>Hint: Async vs Blocking</summary>

For a simple CLI that makes one HTTP request, you probably don't need async.
`ureq` is a great blocking HTTP client. `reqwest` has a blocking feature.
If you choose async, you'll need a runtime like `tokio` - more complexity.
</details>

<details>
<summary>Hint: Feature Flags</summary>

Many crates have optional features. For example:
```toml
reqwest = { version = "0.11", features = ["json", "blocking"] }
```

Check the crate's docs.rs page for available features.
</details>

<details>
<summary>Hint: serde is the standard</summary>

For JSON in Rust, `serde` + `serde_json` is the de facto standard.
It's maintained, well-documented, and used by almost everyone.
Sometimes the obvious choice is obvious for good reasons.
</details>

## Time Estimate

45-60 minutes. The investigation phase should take as long as the coding phase.
That's intentional - choosing dependencies well saves time in the long run.
