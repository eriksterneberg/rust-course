# Rust Course: A Claude Code Learning System

A progressive Rust curriculum delivered through Claude Code agents. This repository contains a **custom teaching system** that uses AI agents to create lessons and guide students through mastering Rust.

## What This Is

This is a **Claude Code extension** that provides:

1. **A Course Creator Agent** (Graydon Hoare) - Generates structured Rust curriculum
2. **A Teaching Command** (Uncle Bob) - Delivers lessons interactively with quizzes and feedback

The agents work together to provide a personalized, adaptive learning experience that tracks progress and adjusts to your understanding.

## Important: Repository Structure

### 📁 **Core Components** (The Important Parts)

```
.claude/
├── agents/
│   └── rust-course-creator.md    # Agent that generates curriculum content
└── commands/
    └── teach.md                   # Teaching command with interactive lessons
```

These files define the behavior of the learning system. They are the **reusable components** that make this teaching system work.

### 📝 **Generated Artifacts** (Practice & Progress Tracking)

```
INSTRUCTIONS.md           # Teaching philosophy and course structure (generated)
chapter-*.yaml           # Lesson files with topics and quizzes (generated)
progress.yaml            # Your oxidation level and mastered concepts (generated)
projects/                # Practice code and exercises (your work)
```

These files are **created during use** and track your personal learning journey. They are artifacts of the learning process, not the core system.

## How to Use

### Initial Setup

1. Clone this repository
2. Ensure you have [Claude Code](https://claude.com/claude-code) installed
3. Navigate to the repository directory

### Start Learning

Run the teaching command:

```bash
/teach
```

This launches **Uncle Bob** (the teaching persona), who will:
- Check your progress
- Present the current lesson
- Teach concepts with analogies and examples
- Quiz you to verify understanding
- Mark progress when you demonstrate mastery

### Create New Lessons

When you complete all available lessons, Uncle Bob will automatically invoke the course creator agent to generate the next chapter. You can also manually trigger it:

```bash
Create the next Rust lesson
```

This launches **Graydon Hoare** (the course creator), who will:
- Analyze your completed lessons
- Design the next chapter based on your progress
- Create structured lesson files with topics, quizzes, and teaching notes

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

## Customization

### Adapting for Other Languages

The agent system can be adapted for teaching other topics:

1. Modify `.claude/agents/rust-course-creator.md` to change the curriculum focus
2. Update `.claude/commands/teach.md` to adjust the teaching style
3. The YAML lesson structure can be reused for any technical topic

### Changing Teaching Personas

The teaching command uses "Uncle Bob" (Robert C. Martin) as the instructor persona. You can modify this in `teach.md` to use a different teaching style or personality.

## Progress Tracking

Your learning journey is tracked in `progress.yaml`:

```yaml
oxidation_level: 4        # Current progress (0-100)
oxidation_tier: "Raw Iron"
mastered_concepts: [...]   # Topics you've mastered
struggle_points: [...]     # Areas needing review
```

## Contributing

This is a personal learning repository, but you're welcome to:

- Fork and customize the agents for your own learning style
- Suggest improvements to the curriculum structure
- Share your own lesson files or teaching approaches

## License

The agent definitions and teaching system are available for reuse and modification. Generated lesson content and practice code are your own work.

---

**Note**: The magic is in `.claude/agents/` and `.claude/commands/` - everything else is just the journey! 🦀
