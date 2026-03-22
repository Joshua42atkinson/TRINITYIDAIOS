---
description: Fill in quest objectives for all 12 ADDIECRAPEYE phases — one phase at a time
---

# Fill ADDIECRAPEYE Objectives

## When to use
When adding or fixing objectives in `crates/trinity-quest/src/quest_system.rs`.
Currently only Analysis objectives exist per chapter. The 11 other phases (Design, Development, etc.)
all fall through to a generic `_` arm. This workflow fills them one phase at a time.

## The scope
- 12 Chapter stages × 12 phases = 144 possible combinations
- Priority: the 5 ADDIE phases for all 12 chapters (60 combos) first
- Then CRAP (48) then EYE (36) — since users hit ADDIE first

## Steps per phase (repeat for each target phase)

1. Identify the phase+chapter combination to fill (e.g. OrdinaryWorld + Design).

2. Add a new match arm ABOVE the `_` catch-all in `objectives_for_chapter()`:
```rust
(HeroStage::OrdinaryWorld, Phase::Design) => vec![
    obj(ch, p, 1, "Draft learning objectives using Bloom's verbs (remember → create)"),
    obj(ch, p, 2, "Sketch the learner journey in 3 acts (hook → practice → reflect)"),
    obj(ch, p, 3, "Choose your primary delivery mechanic (game loop, story branch, quiz)"),
],
```

3. Compile-check immediately (do not add more arms until this one compiles):
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo check -p trinity-quest 2>&1 | grep "^error"
```

4. Test the new arm by calling the API with that phase active:
```bash
curl -s -X POST http://localhost:3000/api/quest/advance -H "Content-Type: application/json" -d '{}'
curl -s http://localhost:3000/api/quest | python3 -c "import sys,json; d=json.load(sys.stdin); [print(o['description']) for o in d['objectives']]"
```

5. Add the next phase arm only after step 3 confirms zero errors.

## Naming convention for objectives
- Use active verbs from Bloom's taxonomy matching the phase's level
- Keep under 80 characters
- Make them specific to the PEARL's subject context where possible
- Analysis = Remember/Identify/List | Design = Plan/Sketch/Draft | Development = Build/Create/Implement
- Implementation = Deploy/Test/Run | Evaluation = Review/Measure/Compare
