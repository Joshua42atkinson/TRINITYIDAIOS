# 🚂 HOW TO USE TRINITY — The Yardmaster's First Ride

> *"The Iron Road doesn't ask your major. It asks you to hold on."*

**Version 1.2** — April 2, 2026

> 🌐 **Try It Now**: [https://LDTAtkinson.com/trinity/](https://LDTAtkinson.com/trinity/)

---

## Who Is This For?

This guide is for **first-time users** — educators, instructional designers, and students — who want to use Trinity to build a learning experience. You don't need to know Rust, React, or AI. You need a subject you care about and 30 minutes.

For **installation** (building from source), see [INSTALL.md](INSTALL.md).
For **architecture** (how it works under the hood), see [TRINITY_FANCY_BIBLE.md](TRINITY_FANCY_BIBLE.md).
For **institutional evaluation**, see [PROFESSOR.md](PROFESSOR.md).

---

## Before You Start

Trinity needs two things running:

1. **Trinity Server** — the application itself (port 3000)
2. **An LLM Backend** — the AI brain. Trinity supports any OpenAI-compatible backend:
   - **LM Studio** (recommended, port 1234) — download from [lmstudio.ai](https://lmstudio.ai)
   - **Ollama** (port 11434) — download from [ollama.com](https://ollama.com)
   - **longcat-sglang** (port 8080) — build from [llama.cpp](https://github.com/ggml-org/llama.cpp)

If someone else set this up for you (e.g., you're using the live demo at LDTAtkinson.com), skip to **Step 1**.

### Quick Local Start
```bash
# 1. Open LM Studio → load any model → start server on port 1234
# 2. In a terminal:
cd trinity-genesis
TRINITY_HEADLESS=1 cargo run -p trinity --release
# 3. Open your browser: http://localhost:3000/trinity/
```

---

## Step 1: Choose Your Subject (The PEARL)

When Trinity opens, you'll see the **Setup Wizard**. It asks one question:

> *"What subject do you want to build a learning experience for?"*

Type your subject — for example: **Chemistry**, **World History**, **Python Programming**, **Music Theory**

This becomes your **PEARL** (Perspective Engineering Aesthetic Research Layout) — the seed that the entire experience grows from. Don't overthink it. You can always refine it later.

**You'll also choose:**
- **Medium**: How the experience will be delivered (Game, Lesson Plan, Storyboard, Simulation)
- **Vision**: A one-sentence description of what success looks like for the learner

> 💡 **Tip**: The PEARL card appears in the top-right corner of the screen throughout your journey. It shows your subject, phase, and alignment progress.

---

## Step 2: Meet Pete (Session Zero)

After selecting your subject, you enter **Session Zero** — Trinity's onboarding conversation.

**Pete** is your AI mentor. He's a Socratic guide — he asks questions, he doesn't give answers. His job is to help *you* think, not to think for you.

Pete will ask three questions:
1. **What's your teaching experience level?** (first-year, 10 years K-12, college adjunct…)
2. **What grade level / audience?** (5th graders, undergraduate, corporate training…)
3. **What's your biggest challenge with this subject?**

Answer honestly. Pete uses your responses to calibrate his scaffolding intensity — new teachers get more guidance, experienced designers get more challenge.

> 🎭 **The Two Mentors**: Trinity has two AI personas sharing one brain:
> - **The Great Recycler 🔮** — The Socratic mirror. Asks WHY. Challenges assumptions. Makes you *think*.
> - **Programmer Pete ⚙️** — The executor. Builds lesson plans, rubrics, artifacts. Makes *things*.
>
> You'll mostly talk to Pete. The Recycler appears during deep reflection moments.

---

## Step 3: The Iron Road (Your Journey)

After Session Zero, you enter the **Iron Road** — Trinity's main workspace. Here's what you see:

### The Three Columns

| Column | What It Shows |
|--------|---------------|
| **Left: Chapter Rail** | All 12 phases of the journey. Your current phase is highlighted. |
| **Center: Workspace** | Quest objectives at the top, conversation with Pete below. This is where you do the work. |
| **Right: Dashboard** | PEARL card, LOCOMOTIVE gauges, Engine Diagnostics, Yardmaster identity. |

### The 12 Phases (ADDIECRAPEYE)

Your journey follows 12 stations, grouped into three acts:

**EXTRACT — ADDIE** (What are you building?)
| # | Phase | You Will... |
|---|-------|-------------|
| 1 | **Analysis** | Define your audience, identify pain points, list what students struggle with |
| 2 | **Design** | Sketch the learning journey, write measurable objectives, choose delivery format |
| 3 | **Development** | Draft the hook, build practice activities, write feedback loops |
| 4 | **Implementation** | Run through your draft, find friction, fix it |
| 5 | **Evaluation** | Define success metrics, compare to your original vision |

**PLACE — CRAP** (How does it look and feel?)
| # | Phase | You Will... |
|---|-------|-------------|
| 6 | **Contrast** | Compare your design to bad and good examples — find your differentiation |
| 7 | **Repetition** | Identify the core concept that must repeat, design multiple encounters |
| 8 | **Alignment** | Verify your hook, practice, and assessment all serve the same objective |
| 9 | **Proximity** | Group related content, remove what doesn't belong, design spatial layout |

**REFINE — EYE** (Where does it go from here?)
| # | Phase | You Will... |
|---|-------|-------------|
| 10 | **Envision** | Write your vision statement, describe the emotional arc |
| 11 | **Yoke** | Connect learning to real-world moments, find memorable metaphors |
| 12 | **Evolve** | Reflect on growth, write your narrative chapter, commit to the Book |

> 💡 **You don't have to complete all 12 in one sitting.** Trinity saves your progress. Come back anytime.

---

## Step 4: Complete Quest Objectives

Each phase has **3 bespoke quest objectives** at the top of the workspace. These are specific tasks — not generic prompts.

For example, in **Chapter 1: Analysis**, you'll see:
- ☐ Describe yourself: What do you teach? Who are your students?
- ☐ Identify a lesson that could be more engaging
- ☐ List 3 things your students struggle with

**How to complete objectives:**
1. Read the objective
2. Talk to Pete about it in the chat — he'll ask follow-up questions
3. When Pete is satisfied you've genuinely engaged with the objective, it marks as complete
4. Complete all 3 objectives to advance to the next phase

> 🎯 **The objectives are pedagogically real.** They're not busywork — each one produces insight or an artifact you'll use later. There are 432 unique objectives across all 12 chapters.

---

## Step 5: Watch Your Locomotive

The **LOCOMOTIVE** panel on the right tracks your progress:

| Gauge | What It Measures | How It Changes |
|-------|-----------------|----------------|
| 🔥 **Coal** | Your available effort/energy | Starts at 100%. Burns when you work. Replenished by good feedback (RLHF). |
| 💨 **Steam** | Your productive momentum | Increases when Pete calls tools, generates artifacts, or advances the quest. |
| ⚡ **XP** | Your experience points | Increases when you complete objectives, tame scope creeps, or pass quality reviews. |
| ✨ **Resonance** | Your alignment quality | Measures how well your work aligns with your PEARL vision. |
| 🧊 **Product** | Your artifact maturity | Shows 0% → 100% as your learning experience matures from "Raw Material" to "Polished." |

> 💡 **Running low on Coal?** That's okay — it means you're working hard. Pete will adjust his scaffolding if you're burning too fast.

---

## Step 6: Handle Scope Creep

During conversation, Pete may detect that you're adding features that weren't in your original PEARL. When this happens, a **Scope Creep** appears — a vocabulary creature that represents the off-topic idea.

You have two choices:

| Choice | What Happens |
|--------|-------------|
| **🔭 Scout (Hope)** | Tame the Creep — adopt the idea into your design. It becomes a resource. |
| **🎯 Sniper (Nope)** | Bag and tag — the idea is logged for later but removed from current scope. |

Both choices are valid. The system tracks your Scope Creep decisions and awards XP for making deliberate choices (either way).

---

## Step 7: Use the Three Modes

Trinity has three operating modes, accessible from the navigation bar:

### 🚂 Iron Road (Default)
The full gamified experience. 12 phases, quest objectives, locomotive economy, narrative chapters. This is the main learning experience.

### ⚡ Express
Skip the game mechanics. A streamlined 3-step wizard:
1. Define your PEARL (subject + audience + objectives)
2. Pete generates a structured lesson plan
3. Export as a document

Use this when you need a quick lesson plan in 10 minutes.

### 🔧 Yardmaster
The power-user IDE. Multi-turn agentic chat where Pete can:
- Read and write files on your system
- Execute shell commands (sandboxed)
- Generate images, music, video, and 3D assets
- Score documents against Quality Matters standards
- Search your knowledge base (RAG)

Use this when you're comfortable with Trinity and want full control.

---

## Step 8: Track Your Portfolio (Character Sheet)

Click the **Character** tab (or the 🎓 icon) to see your **LDT Portfolio** — a living record of everything you've built.

The Character Sheet tracks:
- **Cognitive Metrics**: Coal/Steam/Friction/Vulnerability — how your learning engine is performing
- **Academic Progress**: Which of the 12 phases you've completed
- **Standards Alignment**: IBSTPI, ATD, AECT, and Quality Matters scores
- **Artifact Vault**: Your completed portfolio artifacts
- **Shadow Status**: Your engagement pattern (Clear → Stirring → Active → Processed)

> 🚂 **The Ghost Train**: If Pete detects you're struggling (repeated negative sentiment, abandoning objectives), the **Ghost Train** mechanic activates. This isn't punishment — it's support. Pete shifts to gentler scaffolding and offers reflection prompts. Trinity treats the struggle as *data*, not failure.

---

## Step 9: Export Your Work

When you complete a chapter (or any time you want), you can export your work:

- **📄 Design Document**: Your quest state compiled into a structured Game Design Document
- **🎮 HTML5 Export**: A self-contained interactive quiz, text adventure, or lesson — runs in any browser
- **📊 Quality Scorecard**: Your work evaluated across 5 pedagogical dimensions

Access exports via the **EYE Export** button or through the Yardmaster.

---

## Step 10: The Creative Studio (ART Tab)

Click the **ART Studio** tab to access Trinity's creative pipeline:

| Tool | What It Does |
|------|-------------|
| **Image Generation** | Create visuals for your learning experience (vLLM Omni) |
| **Music Composition** | Generate original music for modules, soundtracks, or presentations |
| **Voice Narration** | Have Pete read content aloud (Kokoro TTS, 6 voices) |
| **Video Generation** | Create short instructional videos from text or images |

> 💡 These features require optional sidecars. If they're not running, Trinity gracefully disables them — the core experience works without any creative tools.

---

## The Navigation Bar

| Element | What It Does |
|---------|-------------|
| **← PORTFOLIO** | Return to the LDTAtkinson.com portfolio landing page |
| **ID · Learning** | Iron Road — the main instructional design workspace |
| **AI · Fun** | ART Studio — creative tools (images, music, video) |
| **OS · Work** | Yardmaster — agentic terminal for power users |
| **🔵 Analysis** | Shows your current ADDIECRAPEYE phase |
| **NETWORK: ONLINE** | Shows LLM connection status |
| **Ch 1** | Shows your current Hero's Journey chapter |
| **⟳ NEW JOURNEY** | Reset and start a new PEARL |

---

## Keyboard Shortcuts & Quick Actions

| Action | How |
|--------|-----|
| Send a message to Pete | Type in the chat box, press **Enter** |
| View quest objectives | Always visible at the top of the workspace |
| Switch phases | Click any phase on the left Chapter Rail |
| Open the Journal | Click **📓 JOURNAL** below the chat |
| Toggle Voice | Click **🔊 VOICE OFF** to enable Pete's spoken responses |
| View quest log | Click **◆ QUESTS** in the header |
| Open Game Design Document | Click **📋 GDD** in the header |
| Take a Quiz | Click **📝 Quiz** in the header |
| Switch to Adventure mode | Click **◈ Adventure** in the header |

---

## Tips for Getting the Most Out of Trinity

1. **Be honest with Pete.** He's Socratic — the more genuine your responses, the better his questions become. Don't try to give "right" answers. Give *real* answers.

2. **Don't rush the Analysis phase.** Chapter 1 is the foundation. Everything else builds on it. Spend time here.

3. **Embrace the Scope Creep mechanic.** When a Creep appears, stop and think: is this idea essential, or am I avoiding the hard work? The act of deciding is the lesson.

4. **Check the LOCOMOTIVE gauges.** If your Coal is low and Steam is high, you're working productively. If Coal is low and Steam is zero, you might be spinning your wheels — ask Pete for help.

5. **Use Journal bookmarks.** When Pete says something insightful, bookmark it. You'll want it later when writing your reflection.

6. **The Ghost Train is your friend.** If the Shadow activates, it means Trinity detected a struggle pattern. Let Pete guide you through it. The reflection journal that follows is often the most valuable part of the experience.

7. **Export early and often.** Don't wait until you're "done." Export at each phase to see your progress materializing.

---

## Frequently Asked Questions

### "Is my data sent to the cloud?"
No. Trinity runs 100% locally. Your conversations, artifacts, and portfolio never leave your machine. The AI model runs on your hardware (or your institution's server). No API keys are needed.

### "What if I don't have a powerful computer?"
Trinity scales. On a laptop with 16 GB RAM, use a smaller model (7B or 8B) via Ollama or LM Studio. You'll get slower responses and less nuanced scaffolding, but the full workflow is functional. The live demo at [LDTAtkinson.com/trinity/](https://LDTAtkinson.com/trinity/) runs on a dedicated server, so anyone can try it.

### "Can I use Trinity for college courses?"
Yes. Trinity was designed at Purdue University's Learning Design & Technology program. The Character Sheet tracks alignment to IBSTPI, ATD, AECT, and Quality Matters standards. Completed PEARLs produce verified portfolio artifacts suitable for academic submission.

### "What subjects work best?"
Any subject works. Trinity has been tested with Chemistry, History, Computer Science, Music Theory, and K-12 general education. The system adapts to your subject matter expertise — you are the Subject Matter Expert, and Pete is the instructional design scaffold.

### "How long does a full journey take?"
A single PEARL (all 12 phases) takes approximately **4-8 hours** of focused work, spread across multiple sessions. Express Mode can produce a structured lesson plan in **10-20 minutes**.

### "What if Pete gives bad advice?"
Pete is an AI scaffold — not an authority. His Socratic questions are designed to prompt *your* thinking, not to dictate answers. If his suggestion doesn't fit your context, tell him. The RLHF system (👍/👎 buttons on every response) lets you train Pete to better match your needs.

---

## Where to Go Next

| Resource | What It Covers |
|----------|---------------|
| [ASK_PETE_FIELD_MANUAL.md](ASK_PETE_FIELD_MANUAL.md) | Pete's persona, Socratic protocol, and cognitive logistics |
| [TRINITY_FANCY_BIBLE.md](TRINITY_FANCY_BIBLE.md) | Complete technical architecture (12 "train cars" of documentation) |
| [PROFESSOR.md](PROFESSOR.md) | Institutional evaluation, standards alignment, and theoretical grounding |
| [INSTALL.md](INSTALL.md) | Building Trinity from source |
| [README.md](README.md) | Quick-start for developers |

---

> **Nomenclature Note**: Trinity uses metaphorical frameworks drawn from multiple cultural and intellectual traditions — Pythagorean mathematics, Kabbalistic mythology ("Golem"), Sanskrit philosophy, and Jungian psychology ("Shadow"). These references are academic in nature and serve as structural analogies for instructional design concepts. They do not represent religious endorsement or spiritual practice.

> ⚠️ **Disclaimer**: Trinity's Shadow/Ghost Train mechanic is an instructional design scaffold inspired by published psychological research (Stutz, Jung, Brown). It is not a clinical tool and does not diagnose, treat, or replace professional mental health services. Users experiencing genuine distress should contact their institution's counseling center or the 988 Suicide & Crisis Lifeline.

---

*"Educate the children and it won't be necessary to punish the men."* — Pythagoras

**TRINITY** — *Textbook · Reflective · Instructional · Narrative · Intelligence · Technology — Yours*

*The Iron Road awaits. All aboard.*
