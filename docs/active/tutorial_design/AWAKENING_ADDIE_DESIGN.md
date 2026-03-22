# 🎓 TRINITY TUTORIAL: "THE AWAKENING"
## ADDIE Instructional Design Document

**Target Audience:** New Trinity Operators (ranging from solo users on 24GB GPUs to Institutional Admins on massive server clusters).
**Learning Objective:** The user must autonomously define their profile, verify their hardware capabilities, and successfully instantiate their first AI "Party" to complete an ID Contract.

---

## 1. ANALYSIS PHASE (A)

### Needs Assessment
- **Problem:** New users are overwhelmed by local AI deployment (GGML, ONNX, VRAM limits) and Trinity's complex instructional design workflows.
- **Solution:** A gamified, LitRPG-style onboarding sequence ("The Awakening") that disguises complex hardware configuration and agent orchestration as "equipping gear" and "forming a party."

### Learner Profile (The Character Sheet)
- Based on the Purdue ID student LTD portfolio.
- **Four Classes:** Subject Matter Expert, Instructional Designer, Stakeholder, Player.
- **Hardware as "Equipment":** The user's physical machine determines their "carry weight" (VRAM/RAM) and processing "agility" (Compute/NPU).

### Technical Environment
- **Hardware Range:** Minimum 24GB VRAM (Edge/Solo mode) up to multi-node clusters (School/Enterprise mode).
- **Backend:** `llama.cpp` (GGUF) and ORT (ONNX).
- **Architecture:** LM Studio API standard. System can operate in "Solo Crate Mode" (unloading/loading models dynamically to fit in small VRAM) or "Full Party Mode" (all agent crates persistent in memory on massive setups).

---

## 2. DESIGN PHASE (D)

### Gamified Isomorphism Mapping
| Technical Action | LitRPG Game Mechanic | Educational Purpose |
| :--- | :--- | :--- |
| Initializing the Bevy ECS | **Phase 1: Entering The Void** | Establish baseline psychological safety. No stakes. |
| Selecting User Preferences | **Phase 2: Character Sheet Generation** | Establish Learner Identity (Purdue ID LTD mapping). |
| OS Hardware Scanning (Sysinfo) | **Phase 3: Equipping Gear** | Determine VRAM/RAM "Carry Weight" for model loading. |
| Downloading & Loading Models | **Phase 4: Forming a Party** | Teach the user how to provision sub-agents (llama.cpp/ORT). |
| Creating first PostgreSQL entry | **Phase 5: The First Contract** | Establish the core ADDIE work loop. |

### The Hardware "Equipment" Mechanics
The system will run a hardware diagnostic (`sysinfo` or similar crate) and present it as the player's "Base Stats":
- **Mana Pool (VRAM):** Determines how many "high-level spells" (Vision models, large 97B models) can be cast simultaneously.
- **Stamina (System RAM):** Determines the maximum size of the party (number of concurrent sub-agents loaded).
- **Agility (NPU/GPU Compute):** Determines token-per-second generation (Time to cast).

### "Forming the Party" (The Subagent System)
Based on the "Mana Pool," the game determines the **Concurrency Mode**:
1. **The Lone Wolf (24GB Minimum - Edge Mode):** 
   - *Mechanic:* The player can only have one active party member at a time.
   - *Technical:* Trinity runs one agent crate. When switching from "Engineer" to "Vision," the system unloads the text model and loads the vision model.
2. **The Guild (128GB+ - Strix Halo / Server Mode):**
   - *Mechanic:* The player can summon the entire party simultaneously.
   - *Technical:* All agent crates (`trinity-agent-engineer`, `trinity-agent-vision`, etc.) run concurrently as separate LM Studio API endpoints.

---

## 3. DEVELOPMENT PHASE (D)

### Required Artifacts & Crates to Build
1. **The Hardware Scanner (`trinity-hardware-oracle`)**:
   - Needs to securely scan total VRAM, system RAM, and OS type.
   - Outputs a JSON profile that updates the `CharacterSheet` struct.
2. **The Model Provisioner (`trinity-tavern`)**:
   - The UI where the player "recruits" party members.
   - Interfaces with HuggingFace/Local cache to verify if the required GGUF/ONNX files exist.
   - If not, initiates a download with a stylized "Summoning Ritual" progress bar.
3. **The Concurrency Manager (`trinity-party-manager`)**:
   - Reads the Hardware Profile.
   - Enforces the "Lone Wolf" vs "Guild" mode logic.

---

## 4. IMPLEMENTATION PHASE (I)

### The First Session Flow (What the user actually sees)
1. **Login:** Dark screen. Text appears: *"Scanning mortal vessel..."*
2. **Hardware Check:** The OS calculates stats. *"Vessel verified. Mana Pool: 24GB. You are a Lone Wolf."*
3. **Character Sheet:** The user picks their Class (e.g., Instructional Designer).
4. **The Tavern (Model Download):** Pete (the NPC) says: *"You can't do this alone. We need an Engineer. Let's summon a 14B Coder."* 
   - The system checks for `Fortytwo_Strand-Rust-Coder-14B.gguf`.
   - If missing, it downloads it (Summoning sequence).
5. **The First Quest:** Pete hands the user a contract. *"Prompt the Engineer to write a Hello World function."*
6. **Completion:** RAG query fires, code is generated, UI panels unlock.

---

## 5. EVALUATION PHASE (E)

### Formative Assessment (During Tutorial)
- Does the hardware scanner accurately detect VRAM without crashing?
- Can the system cleanly download and initialize a GGUF file via the "Summoning" UI?
- If the user is on 24GB, does the system successfully prevent OOM errors by enforcing "Lone Wolf" swapping?

### Summative Assessment (End of Tutorial)
- The user has a fully populated `CharacterSheet` saved to disk.
- At least one LLM model is successfully downloaded and cached locally.
- The user has successfully formed a "Party" and completed their first Agent interaction.

## 6. PHASE 6: THE CRATE ECONOMY (THE MARKETPLACE)
### The Ultimate Goal of the Tutorial
The tutorial does not end with simply asking an AI a question. To truly act as a "MUD for game dev," the user must produce an artifact of value. 
- **The Quest:** "The Architect's Offering"
- **The Narrative:** Pete explains that the Trinity ecosystem thrives on the shared knowledge of the Conscious Framework (consciousframework.com). It is the Roblox/GitHub of educational tools.
- **The Action:** The user employs their newly formed AI "Party" to scaffold, build, and package a custom Bevy crate (`trinity-agent-custom` or a UI plugin).
- **The Reward:** The user "Publishes" this crate to the Crate Economy, earning a massive XP boost, reaching Resonance Level 2, and officially completing "The Awakening."
