# Dependency Decisions

Document your research and reasoning for each dependency choice.

## JSON Parsing

### Options Investigated

| Crate | Downloads | Last Updated | Docs Quality | Notes |
|-------|-----------|--------------|--------------|-------|
| serde_json | 655 803 814  | 3 months ago | Lots of examples, easy to understand | |
| json | 20 135 526 | 6 years ago |  | Doesn't look too bad |

### Decision

**Chosen crate**: serde_json

**Version specification**: "1.0.145" - the newest

**Reasoning**: json hasn't been updated in 6 years. It is abandoned. serde_json not only has more downloads and is regularly updated, it also reached the 1.X.X version milestone, signalling stability.

---

## HTTP Client

### Options Investigated

| Crate | Downloads | Last Updated | Async/Blocking | TLS Support | Notes |
|-------|-----------|--------------|----------------|-------------|-------|
| reqwest | 330 585 898 | 3 days ago | Both | Yes, where the async functions requires Tokio | Latest version doesn't look stable: "0.12.26" |
| ureq |  80 568 910 | 1 month ago | Both | Yes | No documentation in docs.io|
| attohttpc | 18 617 574 | 5 months ago | Blocking only | Yes | Minimalist |

### Decision

**Chosen crate**: reqwest

**Version specification**: reqwest = "0.12.26"

**Features enabled**: default-tls, blocking

**Reasoning**: reqwest is the most downloaded crate, and it is actively maintained. For now I will only need blocking requests, but if I need async in the future this library supports it as well, and there might be a smaller change. The only problem is that the v1.0.0 milestone has not been hit.

---

## Command-Line Parsing

### Options Investigated

| Crate | Downloads | Last Updated | Complexity | Notes |
|-------|-----------|--------------|------------|-------|
| clap | 603 444 595 | 29 days ago | | Actively updated |
| argh | 10 040 220 | 12 months ago | | Abandonware |
| std::env::args | N/A | N/A | | |

### Decision

**Chosen crate**: clap

**Version specification**: clap = "4.5.53"

**Reasoning**: I've used clap before, and it is quite nice. Very intuitive. argh is abandoned. Standard args would probably suffice right now, but quite soon we will grow out of this use case.

---

## Overall Cargo.toml

Paste your final `[dependencies]` section here:

```toml
[dependencies]
clap = "4.5.53"
reqwest = { version = "0.12.26", features = ["default-tls", "blocking"] }
serde_json = "1.0.14"
```

## Lessons Learned

What did you learn about evaluating crates that you'll apply to future projects?

1. Beware of inactivity, we don't want abandonware.

2. You can make a program smaller by not including optional features you don't need in crates.
