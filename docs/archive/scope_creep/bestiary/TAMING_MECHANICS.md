# ADDIECRAPEYE Taming Mechanics & Pet Integration

In the LitRPG context of Trinity, **"Scope Creeps"** are not just technical debt—they are wild monsters living in the **Dumpster Universe**. They represent ambitious ideas, complex integrations, or side quests that were abandoned because they threatened the stability of the core **Iron Road**.

To safely bring a Scope Creep back into the fold, a developer must "tame" it using the **ADDIECRAPEYE** instructional design workflow, tracked via Git.

---

## The Taming Cycle (ADDIECRAPEYE)

To integrate a Scope Creep, you must progress it through the following phases:

### 1. Analysis (The Encounter)
* **Action:** You discover a Scope Creep in the Dumpster Universe (`docs/scope_creep/dumpster_universe/`).
* **Objective:** Read its lore. Analyze its HP (Tech Debt) and Mana Cost (Hardware footprint). Determine if your current character level (system stability) can handle it.
* **Git Action:** Open an issue or discussion about the Scope Creep.

### 2. Design (Containment)
* **Action:** You decide to capture it.
* **Objective:** Architect a safe bounding box around the feature so it cannot crash the Iron Road.
* **Git Action:** Create a dedicated branch: `git checkout -b scope-creep/<monster-name>`. This is the "Summoning Circle."

### 3. Development (Training)
* **Action:** You write the code to adapt the Scope Creep to the current Trinity API.
* **Objective:** Replace outdated mocks, update dependencies, and hook it into the `ConductorLeader` or UI cleanly.
* **Git Action:** Commit progress on the `scope-creep/<monster-name>` branch.

### 4. Implementation (Summoning)
* **Action:** You activate the feature in a local test run.
* **Objective:** The monster is summoned into the active environment. Does it break the UI? Does it panic?

### 5. Evaluation (Sparring)
* **Action:** You measure the friction.
* **Objective:** Watch the `system_reaper` telemetry. If the feature causes the UMA trap, OOM panics, or disrupts the Socratic flow, the pet is disobedient.
* **Failure:** Return to Design/Development.
* **Success:** Proceed to Yield.

### 6. Yield (Tamed)
* **Action:** The monster is officially a tamed pet.
* **Objective:** The feature is stable, performant, and adds value to the LitRPG or ID experience.
* **Git Action:** Merge the branch into `main`. Move the Bestiary entry from "Wild" to "Tamed" (or update its status). The user's Character Sheet gains a new equipped ability.

---

## The Git-Backed Summoning Workflow

1. **Banishment (Sending to the Dumpster):**
   When you encounter Scope Creep during normal development, do not delete it!
   ```bash
   # Move the file to the dumpster
   mv src/cool_but_complex.rs docs/scope_creep/dumpster_universe/cool_but_complex.rs.bak
   ```
   Then create a Bestiary entry for it in `docs/scope_creep/bestiary/` detailing why it was banished.

2. **Summoning (Checking Out a Pet):**
   When you are ready to tame a pet:
   ```bash
   git checkout -b scope-creep/diffusion-golem
   # Restore the file
   mv docs/scope_creep/dumpster_universe/diffusion_asset.rs.bak src/diffusion_asset.rs
   ```

3. **Releasing (Giving Up):**
   If the pet is too strong (too much tech debt), simply discard the branch or put the files back in the dumpster and return to `main`. The pet goes back to sleep.
