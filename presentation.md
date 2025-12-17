---
marp: true
theme: default
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
style: |
  section {
    font-size: 28px;
  }
  h1 {
    color: #CE422B;
  }
  h2 {
    color: #CE422B;
  }
  code {
    background: #f4f4f4;
  }
---

# AI-Powered Learning System
## Teaching Rust Through Agent Collaboration

A progressive curriculum delivered by four specialized AI agents

Erik Sterneberg
2025-12-17

---

## The Problem

**Traditional Learning Approaches**

- 📚 Theory-heavy courses → Students can't apply knowledge
- 💻 Practice-heavy bootcamps → Students lack deep understanding
- ❌ No feedback loop → Students don't know what they don't know
- 🔄 One-size-fits-all → Doesn't adapt to individual struggles

**Result:** Knowledge that doesn't stick, code that doesn't work

---

## The Solution: Four Agents, One Pipeline

```
1. GRAYDON HOARE (Course Creator)
   ↓ Generates curriculum based on language philosophy

2. UNCLE BOB (Teacher)
   ↓ Teaches theory, conducts quizzes (>90% required)

3. BRYAN CANTRILL (Exercise Generator)
   ↓ Creates real-world coding challenges

4. CAROL NICHOLS (Code Reviewer)
   ↓ Reviews solutions, provides feedback

→ Theory + Practice = Mastery
```

---

## Meet the Agents

| Agent | Persona | Responsibility |
|-------|---------|----------------|
| **Graydon Hoare** | Rust creator | Curriculum design |
| **Uncle Bob** | Master teacher | Theory delivery |
| **Bryan Cantrill** | Systems programmer | Challenge creation |
| **Carol Nichols** | Rust Book co-author | Code review |

Each persona shapes how the agent approaches its task

---

## Agent 1: Graydon Hoare
### The Curriculum Designer

**Role:** Creates progressive lesson plans

**Output:**
- `chapter-001.yaml` - Structured lessons with:
  - Learning objectives
  - Key concepts
  - Common pitfalls
  - Spaced repetition prompts

**Philosophy:** Safety and correctness before convenience

---

## Agent 2: Uncle Bob
### The Teacher

**Role:** Delivers interactive lessons

**Workflow:**
1. Presents theory with analogies and stories
2. Quizzes student (must score >90%)
3. Re-teaches weak areas if needed
4. Automatically invokes exercise generator on success

**Style:** Warm, patient, conversational

---

## Agent 3: Bryan Cantrill
### The Exercise Generator

**Role:** Creates practical coding challenges

**Approach:**
- Real-world scenarios with stakes
- "Here's the deal..." narrative framing
- Intentional bugs to debug
- Checklist-based tasks

**Output:** `projects/project-XXX-*/` with EXERCISE.md

---

## Agent 4: Carol Nichols
### The Code Reviewer

**Role:** Validates student solutions

**Review Process:**
1. Runs `cargo check`, `cargo build`, `cargo test`
2. Reviews for correctness, idiomaticity, safety
3. Provides structured feedback
4. Marks complete (if passing) OR guides improvements

**Style:** Constructive, encouraging, educational

---

## The Learning Flow

```bash
# Step 1: Learn Theory
$ /teach
# Uncle Bob teaches, quizzes, creates exercises

# Step 2: Complete Exercises
$ cd projects/project-001-cargo-crisis/
$ # ... write code ...

# Step 3: Get Validated
$ "Please validate my project-001-cargo-crisis"
# Carol reviews and provides feedback

# Step 4: Advance (if exercises pass)
# Next lesson unlocks automatically
```

---

## Example: Chapter 1 Journey

**Theory Lesson:** The Rust Ecosystem
- Toolchain (rustc, rustup, cargo)
- Project structure (crates vs packages)
- Dependency management
- Quiz: 10 questions, 90% required

**Generated Exercises:**
1. **Cargo Crisis** - Fix broken project structure
2. **Crate Detective** - Research and choose dependencies

**Validation:** Code must compile, tests pass, demonstrate understanding

---

## Real Feedback Example

```markdown
## What You Did Well ✨
- ✅ Perfect project structure - src/lib.rs + src/bin/
- ✅ Correct [[bin]] declarations in Cargo.toml
- ✅ Used Rust 2024 Edition (latest!)

## Areas for Improvement 📚
### Cargo.lock Handling
**What I noticed:** Cargo.lock not in .gitignore
**Why it matters:** Libraries shouldn't force versions
**Suggestion:** Add it to .gitignore

## Decision: NEEDS REVISION ❌
Fix the above and resubmit!
```

---

## Progress Tracking: The Oxidation Scale

```yaml
oxidation_level: 7           # 0-100 scale
oxidation_tier: "Raw Iron"   # Current mastery tier

Tiers:
- Raw Iron (0-20%):     Syntax, toolchain
- Forged Steel (21-40%): Ownership, borrowing
- Tempered Alloy (41-60%): Traits, generics
- Conducting Metal (61-80%): Lifetimes, concurrency
- Fully Oxidized (81-100%): Async, unsafe, macros
```

Progress requires **both** theory and practice validation

---

## Why This Works

**1. Enforced Application**
- Can't advance without proving you can code

**2. Immediate Feedback**
- Carol catches mistakes before they become habits

**3. Adaptive Learning**
- Progress tracks weak points for future review
- Spaced repetition built into curriculum

**4. Persona-Driven Engagement**
- Real experts make learning memorable

---

## Technical Implementation

**Built on Claude Code (.claude/ directory)**

```
.claude/
├── agents/
│   ├── rust-course-creator.md     (Graydon)
│   ├── rust-exercise-generator.md (Bryan)
│   └── rust-exercise-validator.md (Carol)
└── commands/
    └── teach.md                    (Uncle Bob)
```

**Each file:** Markdown with agent instructions
**Invocation:** Task tool with specialized subagent types

---

## Key Technical Features

**Autonomous Agent Coordination**
- Uncle Bob automatically invokes Bryan after quiz
- Agents update shared state (progress.yaml)
- No manual orchestration required

**Structured Evaluation**
- Compiles and tests code automatically
- Validates against success criteria
- Updates progress atomically

**Extensible Design**
- New agents can be added
- Personas can be swapped
- Works for any technical topic

---

## Customization Potential

**Adapt for Other Topics:**
- Python data science course
- DevOps/Kubernetes training
- System design curriculum
- SQL/Database mastery

**Change Components:**
- Swap personas (different teaching styles)
- Adjust difficulty (oxidation scale)
- Add more agents (security auditor, performance analyzer)

---

## Real Results

**Chapter 1 Completion:**
- Theory lesson: Passed quiz (83% → 100% after re-teaching)
- Exercise 1: Failed → Fixed → Passed
- Concepts mastered:
  - Cargo project structure
  - Dependency management
  - .gitignore best practices

**Oxidation:** 0 → 7 points

**Student feedback:** "Forced me to actually understand, not just memorize"

---

## What Makes This Different

| Traditional | This System |
|------------|-------------|
| Static content | Dynamic, adaptive |
| Theory OR practice | Theory AND practice required |
| No feedback loop | Continuous validation |
| Generic curriculum | Personalized to struggles |
| Passive learning | Active coding + review |
| Pass/fail grades | Growth-oriented feedback |

---

## Scalability

**One Repository → Many Students**
- Each student gets own progress.yaml
- Agents adapt to individual weak points
- Parallel learning journeys

**Continuous Improvement**
- Add chapters as needed
- Refine agent instructions
- Community contributions

**Cost-Effective**
- No human TAs needed
- Available 24/7
- Consistent quality

---

## Lessons Learned

**What Worked:**
✅ Personas make feedback more engaging
✅ Blocking progression forces mastery
✅ Real-world scenarios (Bryan's war stories)
✅ Structured feedback (Carol's reviews)

**Challenges:**
⚠️ Agents need clear separation of concerns
⚠️ Exercise generator must not give away solutions
⚠️ Balance thoroughness vs. overwhelming feedback

---

## Future Enhancements

**Potential Additions:**
1. **Performance Analyzer** - Benchmarks code, suggests optimizations
2. **Security Auditor** - Reviews for vulnerabilities
3. **Pair Programming Mode** - Real-time collaboration
4. **Community Hub** - Share solutions, compare approaches

**Advanced Features:**
- Video explanations generated for tough concepts
- Interactive debugging sessions
- Peer review integration

---

## How to Get Started

```bash
# Clone the repo
git clone <your-repo-url>
cd rust-course

# Start learning
/teach

# Complete exercises
cd projects/project-001-*/

# Get validated
"Please validate my project-001-exercise"
```

**Open source and customizable!**

---

## The Big Idea

> **AI agents can collaborate to create learning experiences that are:**
> - More personalized than MOOCs
> - More scalable than human tutors
> - More effective than either alone

**This is just the beginning.**

What other domains could benefit from multi-agent teaching systems?

---

## Thank You!

**Questions?**

📧 Contact: [Your contact]
🔗 Repo: github.com/yourusername/rust-course
📖 Try it: Just clone and run `/teach`

**The magic is in `.claude/agents/` and `.claude/commands/`**
**Everything else is just the journey.** 🦀

---

## Appendix: Sample Files

**chapter-001.yaml** structure:
```yaml
chapter: 1
title: "The Rust Ecosystem"
objectives: [...]
topics:
  - name: "The Rust Toolchain"
    key_points: [...]
    common_pitfalls: [...]
checklist:
  - item: "Can explain crate vs package"
    verified: true
```

---

## Appendix: Agent Communication

**How agents share state:**

1. All read/write `progress.yaml`
2. Uncle Bob updates after quiz
3. Bryan reads to create appropriate exercises
4. Carol updates after validation
5. Graydon reads to design next chapter

**Atomic updates prevent conflicts**

---

## Appendix: Example Exercise

**The Cargo Crisis**

"It's 2 AM. Production is blocked. A junior dev committed a Rust project that won't build. Three teams are waiting..."

**Broken elements:**
- Invalid Cargo.toml
- Library code in wrong file
- Binaries in wrong directories
- Bad version specifications

**Student must:** Fix, verify, document reasoning
