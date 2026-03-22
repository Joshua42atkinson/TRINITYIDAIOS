# Trinity AI OS - Purdue Demo Walkthrough
## Educational Game Development Platform

**Duration**: 15 minutes  
**Audience**: Educators, instructional designers, game developers  
**Goal**: Show how Trinity transforms lesson plans into playable educational games

---

## Demo Script

### Part 1: The Ordinary World (2 min)

**Narrator**: "Meet Sarah, a 5th grade science teacher. She has a great lesson on the water cycle, but her students are disengaged. She's heard about educational games but doesn't know how to code."

**Show**: Trinity login screen at http://localhost:3000

**Action**:
1. Open Trinity in browser
2. Character creation screen appears
3. Enter name: "Sarah"
4. Select subject: "Science - Water Cycle"
5. Click "Begin Journey"

**Result**: Character sheet shows:
- Strength: 5 (content knowledge)
- Agility: 5 (tech comfort)
- Wisdom: 5 (pedagogy)
- Charisma: 5 (student engagement)

---

### Part 2: Call to Adventure (3 min)

**Narrator**: "Trinity asks Sarah one question: What game do you want to build?"

**Show**: Quest board with Chapter 1 objectives

**Action**:
1. Sarah types: "A puzzle game where students arrange the water cycle stages"
2. Trinity's Great Recycler responds with Scope Check
3. Shows potential Scope Creeps: "What if we add weather patterns? Climate zones? Ocean currents?"
4. Sarah confirms: "Just the basic water cycle for now"

**Result**: Quest accepted - "Design Water Cycle Puzzle Game"

---

### Part 3: Meeting the Mentor (4 min)

**Narrator**: "Trinity introduces Sarah to her AI Party - specialized agents who will help build the game."

**Show**: Party selection screen

**Action**:
1. Click "View Party Members"
2. Show 6 agents:
   - 🎓 The Evaluator (56GB) - ADDIE expert
   - 🎨 The Artist (15GB) - Game designer
   - ⚙️ The Engineer (15GB) - Code generator
   - 🛡️ The Brakeman (15GB) - QA tester
   - 🎭 Ask Pete (15GB) - Socratic guide
   - 👁️ The Visionary (35GB) - Visual analyst

3. Select Artist for design phase
4. Click "Start Quest"

**Result**: Artist sidecar loads (4 seconds), quest begins

---

### Part 4: The Quest (3 min)

**Narrator**: "Watch as the Artist agent generates a complete Game Design Document in real-time."

**Show**: Quest progress screen with ADDIE phases

**Action**:
1. Analysis phase (30s): Artist analyzes water cycle learning objectives
2. Design phase (2min): Generates GDD with:
   - Game mechanics (drag-and-drop puzzle)
   - Learning objectives (identify 4 stages)
   - UI layout (Bevy ECS architecture)
   - Asset list (evaporation, condensation, precipitation, collection sprites)
   - Color palette (WCAG AA compliant)
   - Cognitive load estimate (appropriate for 5th grade)

3. Show timeout safety: "Step 2 of 5 - 45s elapsed (max 300s)"

**Result**: `docs/game_templates/water_cycle_gdd.md` created

---

### Part 5: The Reward (2 min)

**Narrator**: "With the design complete, Sarah can now generate the actual game code."

**Show**: Generated GDD document

**Action**:
1. Review GDD highlights:
   - Mechanic: "Students drag water droplet through 4 stages"
   - Assessment: "3 attempts, formative feedback on errors"
   - Accessibility: "Keyboard navigation, screen reader support"
   
2. Click "Generate Code" → Engineer sidecar quest
3. Show code generation (fast-forward for demo)

**Result**: Working Bevy game in `templates/water-cycle-game/`

---

### Part 6: Return with Elixir (1 min)

**Narrator**: "Sarah's game is ready to deploy."

**Show**: WASM build and browser deployment

**Action**:
1. Run: `./scripts/build_wasm.sh templates/water-cycle-game`
2. Open browser: http://localhost:8000
3. Play game: Drag water droplet through cycle
4. Show student feedback: "Great! You completed the water cycle!"

**Result**: Playable educational game, no coding required

---

## Key Talking Points

### For Educators
- "No coding experience needed - just describe your lesson"
- "AI handles game design, programming, and accessibility"
- "Full control over learning objectives and assessment"

### For Instructional Designers
- "Built on ADDIE methodology - Analysis → Design → Development → Implementation → Evaluation"
- "Bloom's taxonomy integration ensures appropriate cognitive levels"
- "QM rubric compliance for course quality"

### For Developers
- "100% Rust - no Python dependencies"
- "Local-first - runs on your hardware, no cloud required"
- "128GB unified memory enables 4 concurrent AI agents"
- "Bevy 0.14 for modern game engine architecture"

---

## Technical Specs Shown

**Hardware**: GMKtec EVO-X2
- AMD Ryzen AI MAX+ 395
- 128GB LPDDR5X unified memory
- XDNA 2 NPU (52 TOPS)

**AI Models**:
- Qwen3.5-27B-Claude-4.6-Opus (planning/review)
- Qwen3-Coder-REAP-25B (code generation)
- GPT-OSS-20B (narrative orchestration)

**Performance**:
- Sidecar load time: 4 seconds
- Quest execution: 5-10 minutes (with timeout safety)
- WASM build: 30 seconds
- Browser deployment: Instant

---

## Demo Variations

### Short Version (5 min)
- Skip character creation
- Show pre-generated GDD
- Focus on WASM deployment

### Technical Deep Dive (30 min)
- Show sidecar architecture
- Explain Sword & Shield dual-model system
- Demonstrate timeout safety with long quest
- Show PostgreSQL quest state persistence
- Live code review of generated Bevy game

### Educator Workshop (60 min)
- Hands-on: Each participant creates their own game
- Discuss pedagogy and learning objectives
- Review assessment strategies
- Export and share games

---

## Questions & Answers

**Q: Does this require internet?**  
A: No - Trinity runs 100% locally. Your data never leaves your machine.

**Q: What subjects does it support?**  
A: Any K-12 subject. The AI adapts to your content.

**Q: Can I customize the generated games?**  
A: Yes - all code is yours to modify. Trinity generates a starting point.

**Q: How much does it cost?**  
A: Free and open source. Hardware requirement: 128GB RAM recommended.

**Q: What about accessibility?**  
A: Every game includes WCAG 2.1 AA compliance, keyboard navigation, and screen reader support.

---

## Next Steps

After the demo:
1. Provide Trinity installation guide
2. Share example game templates
3. Offer workshop scheduling
4. Connect to consciousframework.com for game hosting

**Call to Action**: "Transform your lessons into games. Start your Trinity journey today."
