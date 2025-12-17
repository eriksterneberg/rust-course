# Rust Course: A Claude Code Learning System

A progressive Rust curriculum delivered through Claude Code agents. This repository contains a **custom teaching system** that uses AI agents to create lessons and guide students through mastering Rust.

## What This Is

This is a **Claude Code extension** that provides a complete learning system with four specialized AI agents:

1. **Course Creator** (Graydon Hoare) - Designs and generates structured Rust curriculum
2. **Teacher** (Uncle Bob) - Delivers theory lessons with interactive quizzes and feedback
3. **Exercise Generator** (Bryan Cantrill) - Creates practical coding challenges based on real-world scenarios
4. **Code Reviewer** (Carol Nichols) - Reviews your solutions and provides constructive feedback

These agents work together in a coordinated learning pipeline that ensures you both **understand** Rust concepts AND can **apply** them in real code.

## Important: Repository Structure

### 📁 **Core Components** (The Important Parts)

```
.claude/
├── agents/
│   ├── rust-course-creator.md        # Graydon: Generates curriculum
│   ├── rust-exercise-generator.md    # Bryan: Creates coding challenges
│   └── rust-exercise-validator.md    # Carol: Reviews your solutions
└── commands/
    └── teach.md                       # Uncle Bob: Teaches lessons
```

These files define the behavior of the learning system. They are the **reusable components** that make this teaching system work. Each agent has a distinct persona and responsibility in the learning pipeline.

### 📝 **Generated Artifacts** (Practice & Progress Tracking)

```
INSTRUCTIONS.md           # Teaching philosophy and course structure (generated)
chapter-*.yaml           # Lesson files with topics and quizzes (generated)
progress.yaml            # Your oxidation level and mastered concepts (generated)
projects/                # Practice code and exercises (your work)
```

These files are **created during use** and track your personal learning journey. They are artifacts of the learning process, not the core system.

## How the Four Agents Work Together

The learning system uses a **coordinated pipeline** where each agent plays a specific role:

```
1. GRAYDON HOARE (Course Creator)
   ↓ Creates chapter-XXX.yaml files with theory content

2. UNCLE BOB (Teacher)
   ↓ Teaches the lesson via /teach command
   ↓ Quizzes you (must score >90%)
   ↓ Automatically invokes Bryan when you pass

3. BRYAN CANTRILL (Exercise Generator)
   ↓ Creates practical coding challenges in projects/project-XXX-*/
   ↓ Exercises have EXERCISE.md with tasks and success criteria

4. YOU (Student)
   ↓ Complete the coding exercises
   ↓ Request validation when ready

5. CAROL NICHOLS (Code Reviewer)
   ↓ Reviews your code via validation request
   ↓ Provides structured feedback
   ↓ Marks exercise complete if passing (updates progress.yaml)
   ↓ OR guides you on improvements and invites resubmission

→ Next lesson unlocks only after BOTH theory AND exercises are complete
```

This ensures you can't advance by just memorizing theory - you must prove you can write actual Rust code.

## How to Use

### Initial Setup

1. Clone this repository
2. Ensure you have [Claude Code](https://claude.com/claude-code) installed
3. Navigate to the repository directory

### The Learning Flow

**Step 1: Learn Theory**

Run the teaching command:
```bash
/teach
```

Uncle Bob will:
- Present the current lesson with explanations and analogies
- Quiz you to verify understanding (must score >90%)
- Automatically create coding exercises when you pass
- Track your progress and mastered concepts

**Step 2: Practice with Code**

After passing the quiz, Bryan creates exercises in `projects/project-XXX-*/`. Each exercise includes:
- `EXERCISE.md` - Problem description and checklist of tasks
- Starter code and project structure
- Success criteria

Complete the coding challenges by working through the tasks.

**Step 3: Get Validated**

When you've completed an exercise, request validation:
```bash
Please validate my project-001-exercise-name
```

Carol will:
- Review your code for correctness, idiomaticity, and understanding
- Run cargo check, build, test to verify it works
- Provide detailed feedback on what's good and what needs improvement
- Mark the exercise complete (if passing) or guide you on fixes

**Step 4: Advance**

Once ALL exercises for a chapter are validated, Uncle Bob unlocks the next theory lesson. The cycle repeats!

### Creating New Curriculum

When all lessons are complete, Uncle Bob automatically invokes Graydon to create the next chapter. You can also manually request:
```bash
Create the next Rust lesson
```

Graydon analyzes your progress and designs the next lesson based on:
- Your current oxidation level
- Concepts you've mastered
- Areas where you struggled
- The natural progression of Rust topics

## The Learning Philosophy

The course uses an **"oxidation scale"** to measure mastery:

- **Raw Iron (0-20%)**: Syntax and basic constructs
- **Forged Steel (21-40%)**: Ownership and borrowing
- **Tempered Alloy (41-60%)**: Traits and generics
- **Conducting Metal (61-80%)**: Lifetimes and concurrency
- **Fully Oxidized (81-100%)**: Async, unsafe, macros

Key principles:
- **Spaced repetition**: Concepts are revisited to ensure retention
- **Mastery-based**: You must score >90% on quizzes to advance
- **Compiler-friendly**: Learn to embrace Rust's compiler as an ally
- **Systems thinking**: Understand the "why" behind Rust's design

## Why These Personas?

Each agent embodies a real person known for specific expertise:

- **Graydon Hoare** (creator of Rust) - Understands the language philosophy and design decisions
- **Uncle Bob** (Robert C. Martin) - Master teacher known for clear explanations and mentoring
- **Bryan Cantrill** (systems programmer) - Passionate about correctness with great war stories
- **Carol Nichols** (Rust Book co-author) - Empathetic code reviewer who balances encouragement with honesty

The personas aren't just flavor - they shape how each agent approaches their task, making the learning experience more engaging and memorable.

## Customization

### Adapting for Other Topics

The agent system can be adapted for teaching other subjects:

1. Modify agent personas in `.claude/agents/` to match your domain experts
2. Update `.claude/commands/teach.md` to adjust teaching style
3. Change the YAML lesson structure for your topic
4. Adjust oxidation scale to match your learning progression

### Changing Personas

You can swap any persona by editing the agent definition:
- Want a different teacher? Edit `teach.md`
- Prefer different exercise scenarios? Edit `rust-exercise-generator.md`
- Want stricter/gentler code review? Edit `rust-exercise-validator.md`

## Progress Tracking

Your learning journey is tracked in `progress.yaml`:

```yaml
oxidation_level: 7                    # Current progress (0-100)
oxidation_tier: "Raw Iron"            # Current mastery tier
current_chapter: 1                    # Active chapter

mastered_concepts:                    # Topics you've mastered
  - "Rust toolchain components"
  - "Cargo commands and workflows"
  - ...

completed_exercises:                  # Validated coding exercises
  - exercise: "project-001-cargo-crisis"
    completed_date: "2025-12-17"
    concepts_demonstrated:
      - "Hybrid library/binary project configuration"
      - "Cargo.toml [[bin]] declarations"
      - ...

struggle_points: [...]                # Areas needing review
```

Your oxidation level increases through both theory lessons AND coding exercises. Both are required to advance.

## Contributing

This is a personal learning repository, but you're welcome to:

- Fork and customize the agents for your own learning style
- Suggest improvements to the curriculum structure
- Share your own lesson files or teaching approaches

## License

The agent definitions and teaching system are available for reuse and modification. Generated lesson content and practice code are your own work.

---

**Note**: The magic is in `.claude/agents/` and `.claude/commands/` - everything else is just the journey! 🦀
