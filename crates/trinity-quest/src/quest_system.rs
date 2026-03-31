// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-quest/src/quest_system.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        quest_system.rs
// PURPOSE:     Quest objectives generation and database persistence
//
// ARCHITECTURE:
//   • objectives_for_chapter: generates quest objectives for each Hero's Journey stage
//   • Database functions: ensure_quest_tables, save/load game state
//   • NO HTTP handlers here - those belong in trinity crate
//
// CHANGES:
//   2026-03-16  Cascade  Removed duplicate types, kept only DB functions
//
// ═══════════════════════════════════════════════════════════════════════════════

use sqlx::SqlitePool;
use tracing::info;

// Import canonical types from sibling modules
use crate::hero::{HeroStage, Phase};
use crate::party::default_party;
use crate::state::{GameState, Objective, PlayerStats, QuestState};

/// Generate objectives for a specific chapter and phase
pub fn objectives_for_chapter(stage: HeroStage, phase: Phase) -> Vec<Objective> {
    let ch = stage.chapter();
    let p = phase.label();

    match (stage, phase) {

        // ═══════════════════════════════════════════════════════════════════
        // CH 1: THE ORDINARY WORLD — The Yardmaster sees the problem clearly
        // Hero context: You know something is wrong with how learning works.
        // ═══════════════════════════════════════════════════════════════════

        // ADDIE — EXTRACT the PEARL
        (HeroStage::OrdinaryWorld, Phase::Analysis) => vec![
            obj(ch, p, 1, "Describe yourself: What do you teach? Who are your students?"),
            obj(ch, p, 2, "Identify a lesson that could be more engaging"),
            obj(ch, p, 3, "List 3 things your students struggle with"),
        ],
        (HeroStage::OrdinaryWorld, Phase::Design) => vec![
            obj(ch, p, 1, "Sketch the learning journey in 3 moments: hook — practice — aha"),
            obj(ch, p, 2, "Write one measurable objective using a Bloom's verb (e.g. 'students will identify...')"),
            obj(ch, p, 3, "Choose your delivery format: game, storyboard, simulation, or lesson plan"),
        ],
        (HeroStage::OrdinaryWorld, Phase::Development) => vec![
            obj(ch, p, 1, "Draft the opening 60 seconds of your experience (the hook)"),
            obj(ch, p, 2, "Build one practice activity that lets learners attempt the skill"),
            obj(ch, p, 3, "Write the feedback loop: what does the learner see when they fail? When they succeed?"),
        ],
        (HeroStage::OrdinaryWorld, Phase::Implementation) => vec![
            obj(ch, p, 1, "Run through your draft experience yourself — time it"),
            obj(ch, p, 2, "Identify one moment of friction (confusing, slow, unclear)"),
            obj(ch, p, 3, "Fix the friction and implement the corrected version"),
        ],
        (HeroStage::OrdinaryWorld, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Define success: what metric proves this experience worked?"),
            obj(ch, p, 2, "Compare your original 'Ordinary World' description to what you built"),
            obj(ch, p, 3, "Write one sentence: 'I know this works when I see a learner...'"),
        ],

        // CRAP — PLACE the PEARL (visual/spatial design thinking)
        (HeroStage::OrdinaryWorld, Phase::Contrast) => vec![
            obj(ch, p, 1, "Find a bad example of teaching your subject — name exactly what makes it forgettable"),
            obj(ch, p, 2, "Find a great example — name the one thing that makes it stick"),
            obj(ch, p, 3, "List how your design differs from both: what is your Contrast principle?"),
        ],
        (HeroStage::OrdinaryWorld, Phase::Repetition) => vec![
            obj(ch, p, 1, "Identify the ONE core concept that must be encountered multiple times"),
            obj(ch, p, 2, "Design 3 different contexts where learners meet that concept (Pythagorean breadth)"),
            obj(ch, p, 3, "Assign each encounter to a different sense or modality (see / do / explain)"),
        ],
        (HeroStage::OrdinaryWorld, Phase::Alignment) => vec![
            obj(ch, p, 1, "Check: does your hook connect directly to your measurable objective?"),
            obj(ch, p, 2, "Check: does your practice activity match the verb in your objective (identify, build, compare...)?"),
            obj(ch, p, 3, "Check: does your success metric match what the activity actually produces?"),
        ],
        (HeroStage::OrdinaryWorld, Phase::Proximity) => vec![
            obj(ch, p, 1, "Cluster related content together — what belongs in Act 1 vs Act 2 vs Act 3?"),
            obj(ch, p, 2, "Remove one thing that doesn't belong in Ch 1's scope"),
            obj(ch, p, 3, "Draw (or describe) the spatial layout of your experience: where does the learner look first?"),
        ],

        // EYE — REFINE the PEARL (vision + meaning)
        (HeroStage::OrdinaryWorld, Phase::Envision) => vec![
            obj(ch, p, 1, "Write your PEARL Vision statement: 'When this works, the learner will feel...'"),
            obj(ch, p, 2, "Describe the emotional arc: bored/confused → engaged → capable → proud"),
            obj(ch, p, 3, "Ask Pete: 'Does my experience create this arc?' — log Pete's Socratic response"),
        ],
        (HeroStage::OrdinaryWorld, Phase::Yoke) => vec![
            obj(ch, p, 1, "Connect your learning objective to a real-world moment the student will face"),
            obj(ch, p, 2, "Name the stakeholder who benefits most when the student masters this (parent, employer, community)"),
            obj(ch, p, 3, "Yoke the abstract concept to a concrete, memorable metaphor — write it in one sentence"),
        ],
        (HeroStage::OrdinaryWorld, Phase::Evolve) => vec![
            obj(ch, p, 1, "Compare Ch 1's PEARL to what you imagined when you started — what changed?"),
            obj(ch, p, 2, "Write the opening paragraph of your LitRPG chapter: 'In the Ordinary World, the teacher saw...'"),
            obj(ch, p, 3, "Commit the Ch 1 design to your Book — the Iron Road continues to Ch 2"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 2: THE CALL TO ADVENTURE — AI changes everything
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::CallToAdventure, Phase::Analysis) => vec![
            obj(ch, p, 1, "Research how AI could transform this learning experience"),
            obj(ch, p, 2, "List 3 'impossible' things AI now makes possible for your subject"),
            obj(ch, p, 3, "Draft your Call: 'What if learning could be...?'"),
        ],
        (HeroStage::CallToAdventure, Phase::Design) => vec![
            obj(ch, p, 1, "Design how the AI party member serves this lesson (Pete? The Visionary? The Engineer?)"),
            obj(ch, p, 2, "Write 3 prompts you would ask the AI to unlock this learning"),
            obj(ch, p, 3, "Prototype the AI interaction loop: student input → AI response → student action"),
        ],
        (HeroStage::CallToAdventure, Phase::Development) => vec![
            obj(ch, p, 1, "Build the AI prompt template for this lesson's core activity"),
            obj(ch, p, 2, "Test the prompt with 3 different student starting points"),
            obj(ch, p, 3, "Refine the prompt based on the worst response you received"),
        ],
        (HeroStage::CallToAdventure, Phase::Implementation) => vec![
            obj(ch, p, 1, "Integrate the AI interaction into your Ch 1 prototype"),
            obj(ch, p, 2, "Record one full student-AI exchange — what surprised you?"),
            obj(ch, p, 3, "Identify where learners trusted the AI vs where they pushed back"),
        ],
        (HeroStage::CallToAdventure, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Did the AI interaction increase or decrease student confidence? Evidence?"),
            obj(ch, p, 2, "What was the most unexpected AI output — and what does it teach you about your design?"),
            obj(ch, p, 3, "Update your PEARL Vision: has the Call changed what success looks like?"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 3: REFUSAL OF THE CALL — confronting doubt and scope creep
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::RefusalOfTheCall, Phase::Analysis) => vec![
            obj(ch, p, 1, "List your fears: 'I can't because...' (be honest, no filter)"),
            obj(ch, p, 2, "Identify your first Creep — what did you add that wasn't in the PEARL?"),
            obj(ch, p, 3, "Apply the Coal-Steam principle: pick ONE 20-minute win that builds momentum"),
        ],
        (HeroStage::RefusalOfTheCall, Phase::Design) => vec![
            obj(ch, p, 1, "Design a 'minimum viable lesson' — what is the simplest version that still works?"),
            obj(ch, p, 2, "Cut any feature that requires more than 20 minutes of new learning to implement"),
            obj(ch, p, 3, "Document what you're NOT building — the Nope List"),
        ],
        (HeroStage::RefusalOfTheCall, Phase::Development) => vec![
            obj(ch, p, 1, "Build only the minimum viable lesson you designed"),
            obj(ch, p, 2, "Time the development — track coal burned vs steam generated"),
            obj(ch, p, 3, "Note every moment you wanted to add something extra — log them as future Creeps"),
        ],
        (HeroStage::RefusalOfTheCall, Phase::Implementation) => vec![
            obj(ch, p, 1, "Deploy the minimum viable lesson to one real learner"),
            obj(ch, p, 2, "Observe without explaining — watch where they get stuck"),
            obj(ch, p, 3, "Record the timestamp of their first 'aha' moment"),
        ],
        (HeroStage::RefusalOfTheCall, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Did the minimum viable lesson produce the learning outcome? Yes/No — why?"),
            obj(ch, p, 2, "Review your Creep log — which ones were actually worthwhile?"),
            obj(ch, p, 3, "Apply the Heavilon Algorithm to one failure: 'one brick higher' — what's the next iteration?"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 4: MEETING THE MENTOR — the Great Recycler appears
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::MeetingTheMentor, Phase::Analysis) => vec![
            obj(ch, p, 1, "Read the ADDIE framework documentation — note 3 things that surprise you"),
            obj(ch, p, 2, "Map your current problem to all 5 ADDIE phases — where are you now?"),
            obj(ch, p, 3, "Choose your AI Party roles: Conductor, Engineer, Artist, Evaluator"),
        ],
        (HeroStage::MeetingTheMentor, Phase::Design) => vec![
            obj(ch, p, 1, "Design the learning relationship: how will Pete guide without telling?"),
            obj(ch, p, 2, "Write 3 Socratic questions Pete should ask at this phase"),
            obj(ch, p, 3, "Map which AI party member leads each ADDIECRAPEYE station"),
        ],
        (HeroStage::MeetingTheMentor, Phase::Development) => vec![
            obj(ch, p, 1, "Build the first Pete interaction in your experience (the mentor moment)"),
            obj(ch, p, 2, "Write the mentor's opening question — it must reframe the learner's current belief"),
            obj(ch, p, 3, "Create the 'Mentor's Tool' the learner receives: a framework, a checklist, a metaphor"),
        ],
        (HeroStage::MeetingTheMentor, Phase::Implementation) => vec![
            obj(ch, p, 1, "Test the mentor interaction with a learner — did they feel guided or lectured?"),
            obj(ch, p, 2, "Log the exact moment the learner shifted from passive to active"),
            obj(ch, p, 3, "Refine Pete's question based on what actually caused the shift"),
        ],
        (HeroStage::MeetingTheMentor, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Did the AI mentor produce the 'I never thought of it that way' moment?"),
            obj(ch, p, 2, "What would a mentor who OVER-explains look like? How is yours different?"),
            obj(ch, p, 3, "Update PEARL: does the mentor's guidance align with the original Vision?"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 5: CROSSING THE THRESHOLD — first real commitment
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::CrossingTheThreshold, Phase::Analysis) => vec![
            obj(ch, p, 1, "Write 3 learner personas: who will encounter this experience in the real world?"),
            obj(ch, p, 2, "Define the entry prerequisite: what must learners already know to start?"),
            obj(ch, p, 3, "Set your 'coal budget' — realistic time per week you can commit to this build"),
        ],
        (HeroStage::CrossingTheThreshold, Phase::Design) => vec![
            obj(ch, p, 1, "Design the onboarding: how does a learner cross the threshold in your experience?"),
            obj(ch, p, 2, "Write the 'point of no return' moment — when the learner is committed to the journey"),
            obj(ch, p, 3, "Design the early win: what can a learner succeed at in the first 5 minutes?"),
        ],
        (HeroStage::CrossingTheThreshold, Phase::Development) => vec![
            obj(ch, p, 1, "Build the onboarding sequence (first 5 minutes of the experience)"),
            obj(ch, p, 2, "Implement the early win — make it achievable but not trivial"),
            obj(ch, p, 3, "Test: does the early win teach the concept or just feel good?"),
        ],
        (HeroStage::CrossingTheThreshold, Phase::Implementation) => vec![
            obj(ch, p, 1, "Deploy the onboarding to a first-time learner — observe without coaching"),
            obj(ch, p, 2, "Measure time-to-first-success — if > 7 minutes, redesign"),
            obj(ch, p, 3, "Log the exact words learners use to describe the experience after onboarding"),
        ],
        (HeroStage::CrossingTheThreshold, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Did learners cross the threshold with confidence or confusion?"),
            obj(ch, p, 2, "Compare observed behavior to your 3 personas — which persona struggled most?"),
            obj(ch, p, 3, "Revise the threshold for the struggling persona — what one change helps them most?"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 6: TESTS, ALLIES & ENEMIES — building the AI party
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::TestsAlliesEnemies, Phase::Analysis) => vec![
            obj(ch, p, 1, "Activate each AI party member (Pete, Visionary, Engineer, Brakeman) for their specialty"),
            obj(ch, p, 2, "Complete your first micro-learning prototype in 20 minutes exactly"),
            obj(ch, p, 3, "Defeat a Creep by applying the Heavilon Algorithm: name it, contain it, use it"),
        ],
        (HeroStage::TestsAlliesEnemies, Phase::Design) => vec![
            obj(ch, p, 1, "Design the 'test' sequence: 3 escalating challenges that build on each other"),
            obj(ch, p, 2, "Assign an AI ally to each challenge: who coaches what?"),
            obj(ch, p, 3, "Write the 'enemy' for this lesson: the misconception learners must defeat"),
        ],
        (HeroStage::TestsAlliesEnemies, Phase::Development) => vec![
            obj(ch, p, 1, "Build the 3-challenge sequence with escalating difficulty"),
            obj(ch, p, 2, "Build the misconception encounter — make the wrong answer attractive, not obvious"),
            obj(ch, p, 3, "Wire the AI ally response for each challenge outcome"),
        ],
        (HeroStage::TestsAlliesEnemies, Phase::Implementation) => vec![
            obj(ch, p, 1, "Run 3 learners through the challenge sequence — log failure points"),
            obj(ch, p, 2, "Which enemy (misconception) survived? Did learners defeat it or carry it forward?"),
            obj(ch, p, 3, "Identify which AI ally was most effective — and why"),
        ],
        (HeroStage::TestsAlliesEnemies, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Did the escalating challenges create flow (challenge matched ability)?"),
            obj(ch, p, 2, "Did the misconception get corrected permanently or temporarily?"),
            obj(ch, p, 3, "Rate your AI party: who was an ally, who was still an enemy to the design?"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 7: APPROACH TO THE INMOST CAVE — deep design work
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::ApproachToInmostCave, Phase::Analysis) => vec![
            obj(ch, p, 1, "Map learning objectives to Bloom's taxonomy: are you reaching Create?"),
            obj(ch, p, 2, "Design the assessment rubric using Quality Matters standards"),
            obj(ch, p, 3, "Create a learner journey map through the complete experience"),
        ],
        (HeroStage::ApproachToInmostCave, Phase::Design) => vec![
            obj(ch, p, 1, "Design the 'cave entrance' — the hardest concept in your lesson"),
            obj(ch, p, 2, "Write the scaffold: what preparation did the learner receive to attempt this?"),
            obj(ch, p, 3, "Design the 'backup path': what does a learner who fails do next?"),
        ],
        (HeroStage::ApproachToInmostCave, Phase::Development) => vec![
            obj(ch, p, 1, "Build the hardest activity in the experience"),
            obj(ch, p, 2, "Build the scaffold (hints, partial examples, worked models)"),
            obj(ch, p, 3, "Build the recovery path — failure is data, not a wall"),
        ],
        (HeroStage::ApproachToInmostCave, Phase::Implementation) => vec![
            obj(ch, p, 1, "Observe 3 learners attempting the hardest activity — do NOT help"),
            obj(ch, p, 2, "Log every scaffold used — which ones were invisible (best kind)?"),
            obj(ch, p, 3, "Track how many learners took the recovery path and whether it worked"),
        ],
        (HeroStage::ApproachToInmostCave, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Was the hardest moment hard enough? (Too easy = no growth, too hard = abandonment)"),
            obj(ch, p, 2, "Did the scaffold fade appropriately as learners gained confidence?"),
            obj(ch, p, 3, "Update PEARL: does the cave depth match the Vision you set in Ch 1?"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 8: THE ORDEAL — build the actual game
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::TheOrdeal, Phase::Analysis) => vec![
            obj(ch, p, 1, "Generate the Bevy project scaffold with the Engineer party member"),
            obj(ch, p, 2, "Build the core game loop (ECS systems: update → render → input)"),
            obj(ch, p, 3, "Create at least one interactive learning activity inside the game"),
        ],
        (HeroStage::TheOrdeal, Phase::Design) => vec![
            obj(ch, p, 1, "Design the game's win condition — how does the learner 'complete' the lesson?"),
            obj(ch, p, 2, "Design the loss condition — what does failure look like, and what does it teach?"),
            obj(ch, p, 3, "Map each game mechanic to a learning objective (no mechanic without pedagogy)"),
        ],
        (HeroStage::TheOrdeal, Phase::Development) => vec![
            obj(ch, p, 1, "Implement the win state with a clear success message that references the learning"),
            obj(ch, p, 2, "Implement the loss state with a redirect (not a punishment)"),
            obj(ch, p, 3, "Implement one adaptive element: the game responds to player behavior"),
        ],
        (HeroStage::TheOrdeal, Phase::Implementation) => vec![
            obj(ch, p, 1, "Run a 30-minute live playtest with a real learner — record it if possible"),
            obj(ch, p, 2, "Track completion rate and average time to win condition"),
            obj(ch, p, 3, "Find the one mechanic learners ignored — why did they skip it?"),
        ],
        (HeroStage::TheOrdeal, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Did the Ordeal (hardest game moment) produce the intended learning?"),
            obj(ch, p, 2, "Which mechanic created the most Bloom's-level thinking (Create/Evaluate)?"),
            obj(ch, p, 3, "Is this a game that teaches, or content disguised as a game? Be honest."),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 9: THE REWARD — it works!
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::TheReward, Phase::Analysis) => vec![
            obj(ch, p, 1, "Compile and run your complete experience from start to finish"),
            obj(ch, p, 2, "Play through the entire experience yourself — no skipping"),
            obj(ch, p, 3, "Document what you built: title, audience, objective, delivery format, time to complete"),
        ],
        (HeroStage::TheReward, Phase::Design) => vec![
            obj(ch, p, 1, "Design the celebration moment — how does the learner know they've earned the Reward?"),
            obj(ch, p, 2, "Design the reflection prompt: 'What did you build? What does it prove about you?'"),
            obj(ch, p, 3, "Design the 'call to teach' — how will this learner share what they learned?"),
        ],
        (HeroStage::TheReward, Phase::Development) => vec![
            obj(ch, p, 1, "Build the celebration/completion experience (badge, certificate, narrative climax)"),
            obj(ch, p, 2, "Build the reflection journal page or summary artifact"),
            obj(ch, p, 3, "Build the share mechanism: export, publish, or print"),
        ],
        (HeroStage::TheReward, Phase::Implementation) => vec![
            obj(ch, p, 1, "Let a learner experience the full end-to-end journey for the first time"),
            obj(ch, p, 2, "Capture their exact words when they reach the Reward — quote them"),
            obj(ch, p, 3, "Share the experience with one other educator — get their reaction"),
        ],
        (HeroStage::TheReward, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Did the Reward feel earned, or just appear? What's the difference you designed?"),
            obj(ch, p, 2, "Does the learner leave with a tangible artifact they can use in the real world?"),
            obj(ch, p, 3, "Measure transfer: one week later, can the learner apply the skill?"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 10: THE ROAD BACK — test, iterate, prepare to return
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::TheRoadBack, Phase::Analysis) => vec![
            obj(ch, p, 1, "Conduct a pilot test with 3 real learners (not colleagues — real target audience)"),
            obj(ch, p, 2, "Collect feedback using the Quality Matters rubric dimensions"),
            obj(ch, p, 3, "Identify the top 3 improvements needed — rank by learner impact"),
        ],
        (HeroStage::TheRoadBack, Phase::Design) => vec![
            obj(ch, p, 1, "Redesign the one moment with the lowest completion rate"),
            obj(ch, p, 2, "Design an A/B test: original vs. revised version of that moment"),
            obj(ch, p, 3, "Write the iteration brief: what hypothesis are you testing?"),
        ],
        (HeroStage::TheRoadBack, Phase::Development) => vec![
            obj(ch, p, 1, "Build the revised version based on pilot feedback"),
            obj(ch, p, 2, "Document every change made and why (your personal changelog)"),
            obj(ch, p, 3, "Run the A/B version with 2 learners — one sees each version"),
        ],
        (HeroStage::TheRoadBack, Phase::Implementation) => vec![
            obj(ch, p, 1, "Deploy the iteration to a broader group (5-10 learners if available)"),
            obj(ch, p, 2, "Track the delta: did completion rate improve? Did the aha moments move earlier?"),
            obj(ch, p, 3, "Identify the next iteration — the Road Back never truly ends"),
        ],
        (HeroStage::TheRoadBack, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Compare V1 to V2 with data: what changed and by how much?"),
            obj(ch, p, 2, "Did the iteration improve learning, or just improve satisfaction?"),
            obj(ch, p, 3, "Is this experience ready to publish — or does it need Ch 11's polish?"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 11: THE RESURRECTION — polish, accessibility, and presence
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::TheResurrection, Phase::Analysis) => vec![
            obj(ch, p, 1, "Apply WCAG 2.1 AA accessibility standards — run an automated check"),
            obj(ch, p, 2, "Pass the Quality Matters rubric review (8 standards, 41 specific review standards)"),
            obj(ch, p, 3, "Write the game's documentation and teacher implementation guide"),
        ],
        (HeroStage::TheResurrection, Phase::Design) => vec![
            obj(ch, p, 1, "Design the mobile/tablet experience — does it work without a mouse?"),
            obj(ch, p, 2, "Design for low-bandwidth: can a learner access this on a 3G connection?"),
            obj(ch, p, 3, "Design the educator version: what does a teacher see that learners don't?"),
        ],
        (HeroStage::TheResurrection, Phase::Development) => vec![
            obj(ch, p, 1, "Implement keyboard navigation for all interactive elements"),
            obj(ch, p, 2, "Add alt text / ARIA labels to all non-text content"),
            obj(ch, p, 3, "Build the educator dashboard: learner progress, time-on-task, completion status"),
        ],
        (HeroStage::TheResurrection, Phase::Implementation) => vec![
            obj(ch, p, 1, "Test with one learner who uses assistive technology (screen reader, voice control)"),
            obj(ch, p, 2, "Test the educator dashboard with one teacher — ask them to find a specific learner's data"),
            obj(ch, p, 3, "Run the complete Quality Matters self-review — score each standard honestly"),
        ],
        (HeroStage::TheResurrection, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Did accessibility improvements break anything for non-assistive-tech users?"),
            obj(ch, p, 2, "Quality Matters score: how many standards are fully met? Set a ship threshold (≥ 85%)"),
            obj(ch, p, 3, "Is this experience ready to represent your teaching practice publicly?"),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CH 12: RETURN WITH THE ELIXIR — publish, share, and transmit
        // ═══════════════════════════════════════════════════════════════════
        (HeroStage::ReturnWithElixir, Phase::Analysis) => vec![
            obj(ch, p, 1, "Package the experience as a distributable file (zip, crate, SCORM, or web link)"),
            obj(ch, p, 2, "Upload to consciousframework.com or your chosen platform"),
            obj(ch, p, 3, "Write your LitRPG dev novel chapter for greatrecycler.com — the story of this build"),
        ],
        (HeroStage::ReturnWithElixir, Phase::Design) => vec![
            obj(ch, p, 1, "Design the 'elixir': what reusable template, framework, or insight does this produce?"),
            obj(ch, p, 2, "Design the handoff: what does the next educator need to run this without you?"),
            obj(ch, p, 3, "Write the 1-page summary: problem → solution → evidence → replication instructions"),
        ],
        (HeroStage::ReturnWithElixir, Phase::Development) => vec![
            obj(ch, p, 1, "Build the replication package (all assets, source, data, and documentation)"),
            obj(ch, p, 2, "Record a 5-minute demo walkthrough video for the next Yardmaster"),
            obj(ch, p, 3, "Write the metadata: subject, audience, duration, prerequisites, Bloom's level"),
        ],
        (HeroStage::ReturnWithElixir, Phase::Implementation) => vec![
            obj(ch, p, 1, "Publish to at least one channel outside your own context (LMS, GitHub, community)"),
            obj(ch, p, 2, "Present the experience to one peer — watch them use your handoff documentation"),
            obj(ch, p, 3, "Submit to one external review: a colleague, a journal, a conference, or a community"),
        ],
        (HeroStage::ReturnWithElixir, Phase::Evaluation) => vec![
            obj(ch, p, 1, "Could another educator reproduce this experience from your documentation alone?"),
            obj(ch, p, 2, "Look at your original PEARL from Ch 1 — did you build what you envisioned?"),
            obj(ch, p, 3, "Close the circle: what is YOUR aha moment from building this? Write it as the Elixir."),
        ],

        // ═══════════════════════════════════════════════════════════════════
        // CRAP + EYE phases for chapters 2-12 — Socratic fallback
        // These display the specific phase and chapter so Pete can guide appropriately.
        // They will be replaced with specific content as each chapter is playtested.
        // ═══════════════════════════════════════════════════════════════════
        _ => vec![
            obj(ch, p, 1, &format!("Reflect on {}: what question must you answer before moving forward?", phase.label())),
            obj(ch, p, 2, &format!("Apply {} thinking to your PEARL subject — list 3 observations", phase.label())),
            obj(ch, p, 3, &format!("Ask Pete: 'How does {} apply to {}?' — log the insight", phase.label(), stage.title())),
        ],
    }
}

fn obj(ch: u8, phase: &str, n: u8, desc: &str) -> Objective {
    Objective {
        id: format!("ch{}_{}{}", ch, phase, n),
        description: desc.to_string(),
        completed: false,
    }
}

// ═══════════════════════════════════════════════════════════════════
// POSTGRESQL PERSISTENCE
// ═══════════════════════════════════════════════════════════════════

/// Ensure quest state tables exist
pub async fn ensure_quest_tables(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS quest_state (
            id SERIAL PRIMARY KEY,
            player_id TEXT NOT NULL DEFAULT 'default',
            chapter INT NOT NULL DEFAULT 1,
            phase TEXT NOT NULL DEFAULT 'analysis',
            xp INT NOT NULL DEFAULT 0,
            coal REAL NOT NULL DEFAULT 87.0,
            steam REAL NOT NULL DEFAULT 0.0,
            resonance INT NOT NULL DEFAULT 1,
            stats JSONB NOT NULL DEFAULT '{"traction":3,"velocity":2,"combustion":1,"coal_reserves":87.0,"resonance":1,"total_xp":0,"quests_completed":0}'::jsonb,
            inventory JSONB NOT NULL DEFAULT '["📐 ADDIE Framework","🌸 Bloom'\''s Taxonomy","🧠 Cognitive Load Theory"]'::jsonb,
            subject TEXT NOT NULL DEFAULT '',
            game_title TEXT NOT NULL DEFAULT '',
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            UNIQUE(player_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quest_state_player ON quest_state(player_id)")
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS quest_history (
            id SERIAL PRIMARY KEY,
            player_id TEXT NOT NULL DEFAULT 'default',
            quest_id TEXT NOT NULL,
            quest_title TEXT NOT NULL,
            status TEXT NOT NULL,
            xp_earned INT NOT NULL DEFAULT 0,
            duration_secs INT,
            completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            results JSONB
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quest_history_player ON quest_history(player_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quest_history_quest ON quest_history(quest_id)")
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO quest_state (player_id)
        VALUES ('default')
        ON CONFLICT (player_id) DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;

    info!("Quest state tables ensured");
    Ok(())
}

/// Save game state to PostgreSQL
#[allow(dead_code)] // Called from /api/quest/advance when game state changes
pub async fn save_game_state(
    pool: &SqlitePool,
    player_id: &str,
    state: &GameState,
) -> anyhow::Result<()> {
    let stats_json = serde_json::to_value(&state.stats)?;
    let inventory_json = serde_json::to_value(&state.inventory)?;
    let chapter = state.quest.hero_stage.chapter();

    sqlx::query(
        r#"
        INSERT INTO quest_state (player_id, chapter, phase, xp, coal, steam, resonance, stats, inventory, subject, game_title, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW())
        ON CONFLICT (player_id) DO UPDATE SET
            chapter = EXCLUDED.chapter,
            phase = EXCLUDED.phase,
            xp = EXCLUDED.xp,
            coal = EXCLUDED.coal,
            steam = EXCLUDED.steam,
            resonance = EXCLUDED.resonance,
            stats = EXCLUDED.stats,
            inventory = EXCLUDED.inventory,
            subject = EXCLUDED.subject,
            game_title = EXCLUDED.game_title,
            updated_at = NOW()
        "#
    )
    .bind(player_id)
    .bind(chapter as i32)
    .bind(state.quest.current_phase.label())
    .bind(state.quest.xp_earned as i32)
    .bind(state.stats.coal_reserves)
    .bind(state.quest.steam_generated)
    .bind(state.stats.resonance)
    .bind(stats_json)
    .bind(inventory_json)
    .bind(&state.quest.subject)
    .bind(&state.quest.game_title)
    .execute(pool)
    .await?;

    Ok(())
}

/// Load game state from PostgreSQL
#[allow(clippy::type_complexity)]
pub async fn load_game_state(pool: &SqlitePool, player_id: &str) -> anyhow::Result<GameState> {
    let row: Option<(
        i32,
        String,
        i32,
        f32,
        f32,
        i32,
        serde_json::Value,
        serde_json::Value,
        String,
        String,
    )> = sqlx::query_as(
        r#"
        SELECT chapter, phase, xp, coal, steam, resonance, stats, inventory, subject, game_title
        FROM quest_state
        WHERE player_id = $1
        "#,
    )
    .bind(player_id)
    .fetch_optional(pool)
    .await?;

    if let Some((
        chapter,
        phase_str,
        xp,
        coal,
        steam,
        _resonance,
        stats_json,
        inventory_json,
        subject,
        game_title,
    )) = row
    {
        let current_phase = match phase_str.as_str() {
            "Analysis" => Phase::Analysis,
            "Design" => Phase::Design,
            "Development" => Phase::Development,
            "Implementation" => Phase::Implementation,
            "Evaluation" => Phase::Evaluation,
            "Contrast" => Phase::Contrast,
            "Repetition" => Phase::Repetition,
            "Alignment" => Phase::Alignment,
            "Proximity" => Phase::Proximity,
            "Envision" => Phase::Envision,
            "Yoke" => Phase::Yoke,
            "Evolve" => Phase::Evolve,
            _ => Phase::Analysis,
        };

        let stats: PlayerStats = serde_json::from_value(stats_json).unwrap_or_default();
        let inventory: Vec<String> = serde_json::from_value(inventory_json).unwrap_or_default();

        let hero_stage = match chapter {
            1 => HeroStage::OrdinaryWorld,
            2 => HeroStage::CallToAdventure,
            3 => HeroStage::RefusalOfTheCall,
            4 => HeroStage::MeetingTheMentor,
            5 => HeroStage::CrossingTheThreshold,
            6 => HeroStage::TestsAlliesEnemies,
            7 => HeroStage::ApproachToInmostCave,
            8 => HeroStage::TheOrdeal,
            9 => HeroStage::TheReward,
            10 => HeroStage::TheRoadBack,
            11 => HeroStage::TheResurrection,
            12 => HeroStage::ReturnWithElixir,
            _ => HeroStage::OrdinaryWorld,
        };

        let quest_state = QuestState {
            quest_id: "journey".to_string(),
            quest_title: hero_stage.title().to_string(),
            hero_stage,
            current_phase,
            phase_objectives: objectives_for_chapter(hero_stage, current_phase),
            completed_phases: vec![],
            completed_chapters: vec![],
            xp_earned: xp as u32,
            coal_used: 100.0 - coal,
            steam_generated: steam,
            subject: subject.clone(),
            game_title,
            pearl: if subject.is_empty() {
                None
            } else {
                Some(trinity_protocol::Pearl::new(&subject))
            },
        };

        Ok(GameState {
            quest: quest_state,
            stats,
            party: default_party(),
            inventory,
        })
    } else {
        Ok(GameState::default())
    }
}
