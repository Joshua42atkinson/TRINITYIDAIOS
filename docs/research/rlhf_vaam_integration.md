# RLHF & VAAM Integration Strategy (Trinity OS)

## Concept Overview
We are integrating **Reinforcement Learning from Human Feedback (RLHF)** (based on Nathan Lambert's frameworks) directly into the **VAAM (Vocabulary Acquisition Autonomy Mastery)** and **Semantic Search** engines of Trinity. 

This creates a self-optimizing feedback loop where the AI doesn't just passively track what words the user knows, but actively *adjusts its generation* based on explicit and implicit human feedback regarding cognitive load, instructional clarity, and narrative resonance.

## The Three Pillars of Trinity's RLHF

### 1. The Reward Model: VAAM Mastery & Cognitive Load
In traditional RLHF, a reward model is trained on human preference data to score generated text. In Trinity, our "Reward Model" is inherently linked to the user's **Character Sheet (VAAM Tier)**.

- **Positive Reward (+1):** The Yardmaster or Pete generates text that aligns perfectly with the user's current Flesch-Kincaid tier and successfully incorporates a recently "Discovered" VAAM word, prompting the user to successfully use it back (Mastery).
- **Negative Reward (-1):** The user explicitly requests "simpler terms," "too complex," or the Semantic Search returns a graph relationship that the user rejects or ignores.

### 2. Semantic Search as State Observation
When the AI queries the **SurrealDB Graph RAG**, it isn't just looking for facts; it's looking for the *state* of the user's knowledge.
- If a user asks about "ECS Systems," the Graph RAG returns the entity.
- The RLHF system evaluates: "Has this user mastered the VAAM words related to this entity?"
- The Policy (Mistral Small 4 / Ming) adjusts its response to maximize the probability of the user successfully understanding the concept based on past feedback.

### 3. Direct Preference Optimization (DPO) via Iron Road
Instead of full PPO (Proximal Policy Optimization) which is computationally expensive on consumer hardware like the Strix Halo, we will implement a lightweight **DPO (Direct Preference Optimization)** loop through the UI.

**Mechanic: The "Resonance" Rating**
When Pete or the Yardmaster provides an explanation or code snippet, the user can give it a "Resonance Rating" (Thumbs Up / Thumbs Down / "Re-roll with different syntax"). 
- We store this prompt/chosen/rejected triplet in a local SQLite/Postgres table.
- Overnight, during the "Great Recycler" (CROW Continuity) phase, the system runs a lightweight LoRA tuning pass on the NPU (if using a small adapter model) or logs the preferences to adjust the base RAG system prompts dynamically.

## Implementation Path (trinity-iron-road & trinity-data)

1. **Preference Logging Schema:**
   Add a table to our Postgres database to store RLHF triplets: `(prompt, chosen_response, rejected_response, vaam_tier)`.

2. **UI Feedback Hooks:**
   Implement implicit feedback mechanisms in the Bevy UI and Web UI. If a user copies a code snippet, that's an implicit `chosen`. If they immediately regenerate, the previous was `rejected`.

3. **Prompt injection:**
   Update `crates/trinity/src/agent.rs` to dynamically inject the top 3 highest-rated interaction styles for the specific user into the system prompt before generating the next response.
