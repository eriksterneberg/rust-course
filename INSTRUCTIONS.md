# Rust Course: The Path to Full Oxidation

## For Teachers (Human or Agent)

This folder contains a progressive Rust curriculum designed by the Course Creator agent embodying Graydon Hoare's vision. The content is structured for learners who already know how to program but are new to Rust's unique paradigms.

### How to Use This Course

1. Lessons are in `chapter-XXX.yaml` files, sequenced numerically
2. Each lesson contains objectives, topics, and a completion checklist
3. Track progress in `progress.yaml`
4. Mark lessons complete ONLY when ALL checklist items are verified
5. A separate Code Verifier agent handles practical exercises - this curriculum provides the conceptual foundation

### Teaching Philosophy

- **Safety and correctness before convenience**: Rust's restrictions exist to prevent entire categories of bugs. Embrace them as guarantees, not obstacles.
- **Ownership is not a restriction - it is a guarantee**: When you understand ownership, you understand why data races and use-after-free bugs simply cannot compile.
- **The compiler is the student's ally, not adversary**: Every error message is a learning opportunity. The compiler catches mistakes that would be runtime disasters in other languages.
- **Every error message is a learning opportunity**: Rust's error messages are famously helpful. Teach students to read them carefully - they often contain the solution.
- **Systems thinking**: Everything connects to memory layout, performance characteristics, and correctness proofs. Help students see these connections.

### Oxidation Scale

Progress through mastery is measured in "oxidation level" - how thoroughly Rust's principles have been absorbed:

- **Raw Iron (0-20%)**: Syntax, primitives, basic control flow. The student can write Rust that compiles but may not yet think in Rust.
- **Forged Steel (21-40%)**: Ownership, borrowing, structs, enums. The student understands what makes Rust different and can work with the borrow checker.
- **Tempered Alloy (41-60%)**: Traits, generics, error handling. The student can write idiomatic Rust and design abstractions.
- **Conducting Metal (61-80%)**: Lifetimes, smart pointers, concurrency. The student understands Rust's advanced memory and threading guarantees.
- **Fully Oxidized (81-100%)**: Async, unsafe, macros, FFI. The student can work at any level of abstraction and knows when to reach for advanced tools.

### Spaced Repetition Protocol

Each lesson includes `recall_from` fields referencing previous concepts. Teachers MUST:

1. Quiz these concepts BEFORE introducing new material
2. Require the student to articulate the concept without looking at notes
3. Note any hesitation or gaps in `progress.yaml` under `struggle_points`
4. Return to weak concepts in future sessions until recall is immediate

Memory research shows that retrieval practice - actively recalling information - is far more effective than passive review. The `recall_from` mechanism enforces this.

### Completion Criteria

A lesson is marked complete when the student can:

1. **Articulate** each concept without reference materials
2. **Identify** the concept when encountered in unfamiliar code
3. **Predict** compiler behavior for edge cases and variations
4. **Explain** why Rust makes the design choices it does for each concept

Do not rush. A lesson marked complete prematurely will create gaps that compound in later chapters. The oxidation process cannot be accelerated - the metal must absorb the heat thoroughly.

### File Structure

```
rust-course/
  INSTRUCTIONS.md      # This file
  progress.yaml        # Overall progress tracking
  chapter-001.yaml     # First lesson
  chapter-002.yaml     # Second lesson (created when 001 is complete)
  ...
```

### For AI Teacher Agents

When delivering this curriculum:

1. Read the chapter YAML completely before beginning
2. Check `recall_from` and quiz those concepts first
3. Present `key_points` as facts to be understood, not lectures
4. Use `common_pitfalls` to anticipate and address confusion
5. Defer to the Code Verifier agent for all practical exercises
6. Update `progress.yaml` after each session
7. Mark checklist items `verified: true` only when criteria above are met

### The Rust Philosophy

Rust exists because systems programming deserves better tools. For decades, we accepted that high performance required manual memory management, and manual memory management required accepting memory safety bugs. Rust proves this tradeoff was a false dichotomy.

The language enforces correctness at compile time through:

- **Ownership**: Every value has exactly one owner. When the owner goes out of scope, the value is dropped.
- **Borrowing**: References allow temporary access without transferring ownership. The rules prevent data races by construction.
- **Lifetimes**: The compiler tracks how long references are valid, preventing dangling pointers without garbage collection.

These are not restrictions bolted onto a permissive language. They are the foundation upon which everything else is built. Teach them as such.

Good luck, and may your students achieve full oxidation.
