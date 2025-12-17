# Dependency Decisions

Document your research and reasoning for each dependency choice.

## JSON Parsing

### Options Investigated

| Crate | Downloads | Last Updated | Docs Quality | Notes |
|-------|-----------|--------------|--------------|-------|
| serde_json | | | | |
| json | | | | |
| (other) | | | | |

### Decision

**Chosen crate**:

**Version specification**:

**Reasoning**:

---

## HTTP Client

### Options Investigated

| Crate | Downloads | Last Updated | Async/Blocking | TLS Support | Notes |
|-------|-----------|--------------|----------------|-------------|-------|
| reqwest | | | | | |
| ureq | | | | | |
| attohttpc | | | | | |
| (other) | | | | | |

### Decision

**Chosen crate**:

**Version specification**:

**Features enabled**:

**Reasoning**:

---

## Command-Line Parsing

### Options Investigated

| Crate | Downloads | Last Updated | Complexity | Notes |
|-------|-----------|--------------|------------|-------|
| clap | | | | |
| argh | | | | |
| std::env::args | N/A | N/A | | |
| (other) | | | | |

### Decision

**Chosen crate**:

**Version specification**:

**Reasoning**:

---

## Overall Cargo.toml

Paste your final `[dependencies]` section here:

```toml
[dependencies]
# Your dependencies here
```

## Lessons Learned

What did you learn about evaluating crates that you'll apply to future projects?

1.

2.

3.
