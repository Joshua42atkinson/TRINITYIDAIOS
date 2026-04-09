---
description: Pete's Onboarding + Gameplay Loop — make the Iron Road playable end-to-end
---

# Session: First Playable Demo

// turbo-all

> **Goal**: When this session ends, you can pick a subject, meet Pete, build a character,
> complete Analysis objectives, tame a creep, and advance to Design — all narratively.

## Boot Stack

1. Start longcat-sglang (Mistral Small 3.1 / whatever is configured):
```bash
# Check MODEL_REGISTRY.md for the current launch command
cat /home/joshua/Workflow/desktop_trinity/trinity-genesis/MODEL_REGISTRY.md | head -40
```

2. Start Trinity backend:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo run --release 2>&1 &
```

3. Start frontend dev server:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis/crates/trinity/frontend && npm run dev &
```

4. Wait for everything, then open browser:
```bash
sleep 5 && echo "Open http://localhost:5173 in browser"
```

## Phase A: Pete's Session Zero (Character Creation via Narrative)

5. Update Pete's welcome message in PhaseWorkspace.jsx to start character creation:
   - After subject pick, Pete asks 3 Socratic questions:
     - "What's your teaching experience level?"
     - "Who are your students? (age, context)"  
     - "What does success look like for this lesson?"
   - Answers populate the Character Sheet card in the HUD
   - This IS the tutorial — Pete explains the UI as he asks

6. Add a `/api/character` POST endpoint in main.rs:
   - Accepts { experience, audience, success_vision }
   - Stores on game_state.character_sheet
   - Pete references these in future responses

## Phase B: Visible Consequences in Narrative

7. Add coal/steam events to the narrative:
   - When user sends a message → system msg: "🪨 Coal: -2 (attention spent)"
   - When Pete responds → system msg: "💨 Steam: +5 (momentum building)"
   - When objective completed → system msg with XP gain
   - When coal < 20 → Pete narratively suggests a break

8. Hook phase advancement to narrative ceremony:
   - Already built (PhaseTransition component) — verify it triggers

## Phase C: Creep Encounter Flow

9. Verify bestiary scan_text fires during chat:
   - Send a message with domain vocabulary
   - Scope Creep modal should appear
   - Hope/Nope should update the Bestiary card

## Phase D: Gameplay Test

10. Open browser and play through:
    - Pick subject → meet Pete → character creation
    - Chat through Analysis → complete objectives
    - Tame a creep → see it in Bestiary
    - Advance to Design → see phase transition ceremony

## Commit

11. Commit all changes:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && git add -A && GIT_TERMINAL_PROMPT=0 git commit --no-gpg-sign -m "feat: Pete onboarding + visible consequences + first playable"
```
