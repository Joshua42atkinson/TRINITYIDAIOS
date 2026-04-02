# TRINITY ID AI OS User Manual

**Version**: 1.2
**Last Updated**: March 22, 2026

## Introduction

TRINITY ID AI OS is a gamified instructional design system designed to assist K-12 teachers in building educational games and content using AI. This manual provides a comprehensive guide to understanding, setting up, and using TRINITY to create engaging learning experiences. It covers essential user workflows for educators, detailed technical setup for IT support, and advanced API integration for developers looking to customize or extend the platform. Additionally, TRINITY envisions support for **parent and guardian engagement** to bridge home-school learning and tailored solutions for **special needs educators** to ensure accessibility and inclusivity in education.

TRINITY stands for **Instructional Design (ID)**, **Artificial Intelligence (AI)**, and **Operating System (OS)**, combining the ADDIE methodology, CRAP visual design principles, and EYE metacognitive reflection into a unified platform. As a forward-looking system, TRINITY is envisioned as a **classroom server** solution, powering **XR (Extended Reality) systems** in smart glasses for live training and machine learning (ML) scaling. The platform is poised for future scalability into **VR (Virtual Reality)**, **AR (Augmented Reality)**, and broader XR environments, with a focus on **offline and local development** to ensure accessibility and data privacy in educational settings. This vision aligns with the choice of **BEVY** as a foundational platform, offering robust support for 3D and immersive experiences critical for next-generation learning tools.

## System Overview

### Core Architecture
TRINITY operates on a **single-brain architecture** with multiple modes, utilizing a powerful large language model (LLM) like Mistral Small 4 119B (~68GB) to orchestrate various personas and functionalities. This design minimizes resource overhead by using one static RAM load with dynamic system prompts for different roles, rather than multiple models. The system is built with **Rust** for backend efficiency (using the Axum server framework on port 3000) and **React** for the frontend UI, running on high-spec hardware (128GB RAM recommended).

#### Key Technical Components
- **Inference Router**: Automatically detects and manages multiple inference backends (llama-server, vLLM, Ollama, LM Studio, SGLang) with health monitoring and failover capabilities. This ensures robust AI service availability.
- **Duality KV Cache**: Utilizes dual slots in the LLM (e.g., slot 0 for strategic Great Recycler persona, slot 1 for execution-focused Programmer Pete), enabling instant persona switching with up to 500K total context capacity.
- **Backend Server**: The Rust Axum server handles API orchestration, quest persistence, and real-time hardware telemetry (CPU/RAM/GPU/NPU usage), ensuring system stability and performance monitoring.
- **Frontend Interface**: A React-based UI with three primary modes:
  - **Iron Road Mode**: Full gamified experience with LitRPG narrative.
  - **Express Mode**: Streamlined wizard for quick content creation.
  - **Yardmaster Mode**: IDE-like environment for advanced orchestration and tool-calling.
- **Database**: PostgreSQL 15+ with pgvector extension for state persistence (sessions, messages, projects) and Retrieval Augmented Generation (RAG) for semantic search, auto-ingesting key documentation on startup.
- **Sidecar Services**: External services for specialized tasks, isolated for crash protection:
  - **ComfyUI (port 8188)**: SDXL Turbo for image generation.
  - **Voice Pipeline (port 8200)**: Whisper STT + Kokoro TTS for audio interactions.
  - **Qianfan-OCR (port 8081)**: Document intelligence and analysis sub-agent.

#### Autonomous Capabilities
TRINITY is designed for high autonomy, reducing user intervention through advanced self-management features:
- **Cow Catcher System**: Detects and recovers from obstacles like timeouts (e.g., 300s per LLM step), compilation errors, or quest failures. It logs issues, skips problematic steps, and can auto-restart critical components after multiple failures, ensuring continuous operation.
- **NPU Orchestration**: A classifier routes simple queries to a fast Neural Processing Unit model (e.g., Llama-3.2-1B ONNX) and complex tasks to sidecar models, optimizing performance (80% NPU, 20% sidecar).
- **State Persistence**: Quest progress and user data are automatically saved to PostgreSQL, surviving server restarts or crashes, with state loaded on startup for seamless continuation.
- **Self-Improvement Loop**: Analyzes Cow Catcher logs to adjust context sizes, refine prompts, and improve workflows based on detected failure patterns, ensuring long-term system enhancement.

### Key Components
- **Iron Road**: A gamified journey through 12 ADDIECRAPEYE stations for instructional design, mapping the Hero's Journey to educational content creation.
- **Pete**: Socratic AI mentor guiding users through questions rather than direct answers, with personas like Great Recycler (visionary) and Programmer Pete (executor).
- **ART**: Handles creative tasks like image generation and visual design, with modes for aesthetics, research, and tempo (code generation).
- **Yardmaster**: Manages system operations and autonomous tasks, providing an agentic dev console for multi-turn tool-calling and file operations.
- **VAAM (Vocabulary Acquisition Autonomy Mastery)**: Tracks vocabulary complexity and adapts responses to user language levels, enhancing accessibility.
- **Cow Catcher**: Autonomous error detection and recovery system, critical for maintaining workflow continuity.
- **PEARL (Perspective Engineering Aesthetic Research Layout)**: A focusing agent ensuring alignment of subject, medium, and vision across design phases.
- **Perspective Engine (Ring 6)**: Multi-perspective AI evaluation that annotates Pete's responses with 3 lenses — Bloom's Check (cognitive level verification), Practitioner (real-world applicability), and Devil's Advocate (constructive challenge). Perspectives appear as collapsible marginalia alongside the conversation.
- **Journal States**: Chapter milestone snapshots and weekly reflections that capture the complete learning state (quest progress, character sheet, skills) at a point in time. Entries can be exported as standalone HTML portfolio pages for sharing, standardization, or archival.
- **Quality Scorecard**: Pedagogical document evaluation across 5 dimensions — Bloom's Coverage, ADDIE Alignment, Accessibility, Student Engagement, and Assessment Clarity. Instantly scores uploaded documents and generates actionable recommendations. This is Trinity's competitive differentiator: *NotebookLM summarizes your syllabus; Trinity tells you what's missing.*

This architecture and set of autonomous features make TRINITY a robust, self-sustaining platform for educational content creation, capable of handling complex workflows with minimal user intervention while providing a user-friendly, gamified interface for educators.

## ADDIECRAPEYE Framework

The ADDIECRAPEYE framework is the core instructional design methodology of TRINITY ID AI OS, guiding users through a structured 12-phase process to create pedagogically sound educational content. It combines three key methodologies:
- **ADDIE**: Analysis, Design, Development, Implementation, Evaluation - the foundational instructional design model.
- **CRAP**: Contrast, Repetition, Alignment, Proximity - visual design principles to enhance content presentation.
- **EYE**: Envision, Yoke, Evolve - metacognitive reflection to ensure long-term impact and adaptation.

Each phase, or 'station,' on the Iron Road corresponds to a stage in the Hero's Journey narrative structure and is mapped to Bloom's Taxonomy for increasing cognitive complexity. Below is an overview of the 12 stations, including specific examples of objectives or outputs at each stage to illustrate the user journey:

1. **Analysis (ADDIE)** - Understand the instructional goals and learner needs. Users define their subject, audience, and objectives with Socratic guidance from Pete. (Bloom's: Remember/Understand)
   - *Example*: For a course on "Introduction to Fractions," Pete might ask, "What prior knowledge do your 4th-grade students have about division?" The output is a defined learner profile and learning goals, such as "Students will understand basic fraction concepts."
2. **Design (ADDIE)** - Create a blueprint for the learning experience, including structure and activities. (Bloom's: Understand/Apply)
   - *Example*: Users outline a lesson plan with Pete's guidance, deciding to use a pizza-slicing game to teach fractions. The output is a storyboard or lesson structure with activities like "Interactive pizza division game."
3. **Development (ADDIE)** - Build the content and materials based on the design blueprint. (Bloom's: Apply/Create)
   - *Example*: ART assists in generating visual assets (pizza slice images), and users script quiz questions like "How many slices make a whole pizza?" The output is a set of digital assets and content drafts.
4. **Implementation (ADDIE)** - Deploy the content, testing it in a real or simulated environment. (Bloom's: Apply)
   - *Example*: Users test the pizza game in a mock classroom setting within TRINITY, observing student interaction simulations provided by Pete. The output is a tested prototype with initial feedback notes.
5. **Evaluation (ADDIE)** - Assess the effectiveness of the content against objectives and refine as needed. (Bloom's: Analyze/Evaluate)
   - *Example*: Pete guides users to evaluate quiz results (e.g., "80% answered correctly"), prompting refinement of unclear questions. The output is a revised lesson with improved clarity.
6. **Contrast (CRAP)** - Focus on visual hierarchy and boundary design to make content engaging. (Bloom's: Analyze)
   - *Example*: ART suggests using bold colors for fraction parts versus the whole pizza to highlight differences. The output is a visually distinct game interface that draws attention to key concepts.
7. **Repetition (CRAP)** - Solidify core concepts through repeated exposure in varied contexts. (Bloom's: Evaluate)
   - *Example*: Users design multiple game levels (pizza, pie, cake slicing) to reinforce fraction division. The output is a series of activities repeating the core mechanic in diverse scenarios.
8. **Alignment (CRAP)** - Ensure all elements align with learning objectives and pedagogical standards. (Bloom's: Evaluate)
   - *Example*: Pete questions if game visuals align with curriculum standards, leading to adjustments like adding numerical labels. The output is a cohesive design meeting educational goals.
9. **Proximity (CRAP)** - Optimize user experience by grouping related content and minimizing cognitive load. (Bloom's: Create)
   - *Example*: Users organize game controls and fraction visuals close together on-screen to reduce confusion. The output is an intuitive UI layout for student interaction.
10. **Envision (EYE)** - Reflect on the broader vision and emotional impact of the learning experience. (Bloom's: Evaluate)
    - *Example*: Pete asks, "How will this game inspire curiosity about math?" leading to added narrative elements like a chef character. The output is a motivational storyline enhancing engagement.
11. **Yoke (EYE)** - Connect frontend design with backend functionality for a cohesive product. (Bloom's: Create)
    - *Example*: Users integrate game logic (correct fraction answers advance levels) with visuals, ensuring seamless operation. The output is a fully functional game prototype.
12. **Evolve (EYE)** - Finalize deployment and plan for future adaptations based on feedback and outcomes. (Bloom's: Create)
    - *Example*: After testing, users plan a follow-up module on decimals, exporting the game as HTML5 for LMS integration. The output is a deployed game with a roadmap for expansion.

Throughout these stations, users interact with Pete, who employs a Socratic approach by asking questions rather than providing direct answers, fostering discovery and critical thinking. The Iron Road interface gamifies this process with resources like Coal (compute budget), Steam (creative energy), and Iron (structural integrity), alongside mechanics like Scope Creep battles and Semantic Creep taming to maintain focus and engagement.

This framework ensures that educational content is not only well-designed but also visually appealing and reflective of long-term learning goals, making TRINITY a powerful tool for educators.

## User Interaction with Pete

Pete is the primary AI mentor within TRINITY ID AI OS, designed to guide users through the instructional design process using a Socratic approach. Unlike traditional AI assistants that provide direct answers, Pete engages users with questions to foster critical thinking, discovery, and ownership of the learning design process.

### Socratic Protocol
Pete adheres to a strict Socratic Protocol, which is integrated into the narrative of the Iron Road. This protocol includes the following principles:
- **Ask Before Telling**: Pete always leads with questions to encourage users to think deeply about their goals and challenges.
- **Present Options**: Instead of giving a single solution, Pete offers 2-3 narrative paths or options for users to choose from, ensuring they remain in control of the design process.
- **Reflect Back**: Pete summarizes and reflects the user's input to confirm understanding and alignment before proceeding to the next step.
- **Reward Discovery**: When users demonstrate understanding or use vocabulary correctly, Pete acknowledges their progress narratively, often as earning 'Coal' or forging a new skill.
- **Guard the PEARL**: If a user's response drifts from their defined subject or vision (as captured in the PEARL focusing agent), Pete flags it as a 'Scope Creep' sighting, gently guiding them back to focus.

### Interaction Flow
1. **Set the Scene**: Pete describes the current station on the Iron Road, providing context about what the user is working on and the challenges ahead.
2. **Engage with Questions**: Based on the current ADDIECRAPEYE phase, Pete asks targeted questions to help define objectives, brainstorm activities, or evaluate outcomes.
3. **Narrative Integration**: Responses are framed within the gamified Iron Road story, making the interaction feel like a journey. For example, Pete might narrate, "You step onto the platform at the Analysis Station. What do you see as the biggest challenge your students face?"
4. **Progress Tracking**: As users complete objectives, Pete narrates their advancement, celebrating milestones like clearing a station and preparing for the next phase.

### Modes of Interaction
- **Iron Road Mode**: Full gamification with LitRPG narrative, where Pete acts as a Dungeon Master, guiding users through a story-driven quest.
- **Express Mode**: A streamlined wizard for quick content creation, where Pete's guidance is more direct but still question-based.
- **Yardmaster Mode**: An IDE-like environment for advanced users, where Pete assists with technical orchestration while maintaining the Socratic method.

### Gamification Mechanics
TRINITY integrates several gamified elements into interactions with Pete to enhance engagement and learning:
- **Resource Management**: Users manage three key resources influenced by their interactions with Pete:
  - **Coal (Compute Budget)**: Represents the energy or attention available for tasks. Answering Pete's questions thoughtfully can earn Coal, while Scope Creep battles may consume it. For example, successfully defining a clear objective might grant 10 Coal units.
  - **Steam (Creative Energy)**: Reflects momentum and inspiration. Completing a station or taming Semantic Creeps increases Steam, which can be spent on creative tasks like generating assets with ART. A typical gain might be 5 Steam per completed objective.
  - **Iron (Structural Integrity)**: Represents the robustness of the design. Strong answers to Pete's questions bolster Iron, which protects against design flaws or Scope Creep damage. Losing a battle might deduct 3 Iron points.
- **Scope Creep Battles**: When ideas drift from core objectives, Pete triggers a battle mechanic. Users must make a skill check (virtual d20 roll + relevant skill bonus, e.g., Curriculum Design +2) to refocus. Success preserves resources; failure may cost Coal or Iron. For instance, suggesting an unrelated game feature might spawn a "Scope Creep Monster" with a difficulty of 15 to overcome.
- **Semantic Creep Taming**: Pete identifies vocabulary usage as creatures to tame, each with elemental properties (Earth for nouns, Air for verbs, Fire for adjectives, Water for adverbs). Correct usage narrated by Pete as "taming" grants bonuses like extra Coal or skill boosts. For example, using "partition" correctly in a math context might tame an "Earth Creep," adding +1 to Assessment Design skill.
- **Character Sheet Progression**: Interactions with Pete build the user's character sheet, tracking skills (e.g., Gamification, Narrative Design) and achievements (e.g., "Station Master" badge for completing a phase). Each station cleared with Pete's guidance might increase a skill by +1, reflecting professional growth.

### Practical Scenario
Imagine a teacher designing a biology lesson on photosynthesis at the Design Station:
1. **Pete's Prompt**: "You've arrived at the Design Station with a lush forest as your backdrop. How might you structure a game to teach photosynthesis to 7th graders? Consider a journey, a puzzle, or a competition."
2. **User Response**: "I think a journey where students guide a plant through growth stages by collecting sunlight and water would work."
3. **Pete's Reflection**: "A journey through growth stages is a fine path. I see you've earned some Steam for that creative spark! How many stages will this journey have, and what challenges will the plant face at each?"
4. **Outcome**: If the user's stages align with photosynthesis steps, Pete narrates, "Your design has forged 5 Iron for its strong structure." If the idea drifts (e.g., adding unrelated fantasy elements), Pete warns, "A Scope Creep shadow looms—does this magic potion fit your science lesson? Roll for a skill check to refocus." Success might preserve resources; failure could cost 3 Coal as the distraction lingers.
5. **Progress**: Completing the Design Station with Pete's guidance updates the character sheet (e.g., +1 to Gamification skill) and advances the Iron Road narrative to Development.

By interacting with Pete, users not only design educational content but also develop their own metacognitive skills, making TRINITY a transformative tool for professional growth in instructional design.

## Advanced Usage and API Integration

This section is intended for technical users, developers, or IT administrators who wish to customize, extend, or integrate TRINITY ID AI OS with other systems. TRINITY provides a robust set of APIs and advanced tools for orchestration, allowing for tailored educational solutions and automated workflows. With a vision towards future scalability, TRINITY's architecture is designed to support **classroom server deployments**, integration with **XR systems** in smart glasses for live training, and **ML scaling** for adaptive learning content. The platform's roadmap includes expansion into **VR, AR, and XR environments** with **offline and local development** capabilities, ensuring privacy and accessibility. The choice of **BEVY** as a foundational platform enhances this vision, providing a powerful game engine for 3D and immersive content critical for next-generation educational tools.

### Agentic Dev Console
The Agentic Dev Console, accessible at `http://localhost:3000/dev.html`, offers a powerful interface for multi-turn tool-calling and autonomous operations. It enables users to interact directly with TRINITY's underlying AI capabilities beyond the standard UI.

- **Capabilities**:
  - **File Operations**: Read, write, and modify files within the workspace using tools like `read_file` and `write_file`.
  - **Shell Commands**: Execute bash commands via the `shell` tool for system-level operations (subject to security restrictions).
  - **Code Search**: Search across the codebase with `search_files` to locate specific functionalities or troubleshoot issues.
  - **Sidecar Management**: Check and manage AI model status with `sidecar_status` and `sidecar_start`.
- **Usage Example**: A developer might use the console to automate the creation of multiple lesson plans by scripting a sequence of API calls, leveraging Pete's Socratic guidance programmatically.
- **Security Note**: Destructive tools are gated by persona clearance (e.g., only Programmer Pete can execute shell commands), enforced by TRINITY's Ring 2 security system.

### Key API Endpoints
TRINITY's backend server (default port 3000) exposes a comprehensive API for interacting with core functionalities. Below are key endpoints for advanced usage, derived from system documentation:

- **Unified Chat Endpoint**:
  - **POST /api/v1/trinity**: A single endpoint to interact with TRINITY as a person, routing requests based on mode (Iron Road, Express, Yardmaster).
  - *Example*: Send a JSON payload with `{"mode": "IronRoad", "message": "Design a math game"}` to initiate a gamified design session.
- **Chat and Agentic Interaction**:
  - **POST /api/chat**: Direct chat with the LLM.
  - **POST /api/chat/stream**: Streaming chat responses via Server-Sent Events (SSE).
  - **POST /api/chat/yardmaster**: Agentic chat with tool-calling capabilities for complex tasks.
  - *Example*: Use `/api/chat/yardmaster` with a payload like `{"query": "Generate a Bevy game scaffold", "tools": ["scaffold_bevy_game"]}` to trigger automated game creation.
- **Quest and Progression**:
  - **GET /api/quest**: Retrieve full game state and ADDIECRAPEYE phases.
  - **POST /api/quest/advance**: Advance to the next phase.
  - **POST /api/quest/complete**: Mark a quest objective as complete, updating resources (Coal, Steam, XP).
  - *Example*: After completing a design objective, call `/api/quest/complete` with `{"objective_id": "design-outline"}` to progress.
- **Creative Tools**:
  - **POST /api/creative/image**: Generate images via ComfyUI.
  - **POST /api/creative/music**: Generate audio via MusicGPT (if configured).
  - **GET /api/creative/status**: Check sidecar health.
  - *Example*: Request an image with `{"prompt": "A diagram of photosynthesis", "workflow": "SDXL Turbo"}` to `/api/creative/image` for lesson materials.
- **Inference Management**:
  - **GET /api/inference/status**: View status of all inference backends (health, capabilities).
  - **POST /api/inference/switch**: Switch active backend (e.g., to a lighter model for testing).
  - *Example*: Switch to a smaller model with `{"backend": "Crow-9B"}` to `/api/inference/switch` on lower-spec hardware.
- **Export and Compilation**:
  - **POST /api/eye/compile**: Compile quest data into an exportable EYE container.
  - **GET /api/eye/export**: Export content as HTML5 quiz, adventure, or JSON (specify `?format=`).
  - *Example*: Export a completed game with `/api/eye/export?format=html5-quiz` for LMS integration.
- **Perspective Engine (Ring 6)**:
  - Perspectives are delivered via Server-Sent Events (SSE) on the `/api/book/stream` endpoint with event type `perspective`.
  - Lenses fire automatically after Pete's responses — no API call needed.
  - Frontend displays results in a collapsible sidebar with relevance scoring and feedback buttons.
- **Journal States**:
  - **GET /api/journal**: List all journal entries (newest first).
  - **POST /api/journal**: Create a new journal entry from current game state. Body: `{"entry_type": "weekly_reflection", "reflection": "What I learned...", "tags": ["week-3"]}`.
  - **GET /api/journal/export/:id**: Export a journal entry as a standalone HTML page for portfolio sharing.
  - Entry types: `phase_complete`, `chapter_complete`, `weekly_reflection`, `checkpoint`, `demo_bookmark`.
  - *Example*: Create a weekly reflection with `POST /api/journal {"entry_type": "weekly_reflection", "reflection": "Students struggled with fractions this week."}` to capture the moment for review.
- **Quality Scorecard**:
  - **POST /api/yard/score**: Score a document for pedagogical quality. Body: `{"text": "<document content>"}` or `{"document_id": "<title to look up in RAG>"}`.
  - Returns 5-dimension scores (0.0-1.0), letter grade (A-F), summary, and actionable recommendations.
  - *Example*: Score a lesson plan with `POST /api/yard/score {"text": "# Ecosystems Lesson Plan..."}`.

### API Authentication and Security
- All API calls are subject to TRINITY's Ring Security System:
  - **Ring 2**: Blocks destructive tools unless the persona has appropriate clearance.
  - **Ring 3**: Summarizes old context to prevent token overload, maintaining only recent messages verbatim.
  - **Ring 5**: Enforces rate limiting (60 calls/min globally, 5 destructive/min) and sandboxing for shell commands.
  - **Ring 6**: Multi-perspective evaluation of AI responses (Bloom's Check, Practitioner, Devil's Advocate) — annotation only, never modifies Pete's responses.
- Ensure API requests include proper session authentication if required (check UI for session ID or token under "Settings" or "Developer" tabs).

### Customizing Workflows
Technical users can create custom workflows using the Agentic Dev Console or direct API calls:
- **Automated Content Generation**: Script a sequence to generate lesson plans across multiple subjects by iterating through ADDIECRAPEYE phases programmatically.
- **Integration with LMS**: Use `/api/eye/export` to generate content in formats compatible with Learning Management Systems like Canvas or Moodle, automating deployment with scripts.
- **Custom Sidecar Development**: Extend TRINITY by developing new sidecars for specific tasks (e.g., video generation), integrating them via the `/api/creative/` endpoints.

### Practical Scenario: Building a Custom Tool
A developer wants to automate rubric generation for multiple courses:
1. Access the Agentic Dev Console at `http://localhost:3000/dev.html`.
2. Use the `shell` tool to list existing rubric templates: `shell: ls templates/`.
3. Call `/api/chat/yardmaster` with a custom query: `{"query": "Generate 5 rubrics for biology units", "tools": ["generate_rubric"]}`.
4. Script a loop to repeat for other subjects, saving outputs via `write_file`.
5. Export results using `/api/eye/export?format=json` for external processing.

### Documentation and Further Reading
For full API documentation, refer to `CONTEXT.md` in the repository (section on Key API Endpoints) or check the `/api/health` endpoint for a live overview of available services. Developers can also explore `crates/trinity/src/trinity_api.rs` for implementation details of the unified API.

This advanced usage guide empowers technical users to leverage TRINITY's full potential, from automating educational content creation to integrating with external systems, ensuring flexibility for diverse educational environments.

## Installation and Setup

This section provides step-by-step instructions for installing and setting up TRINITY ID AI OS on compatible hardware. Given the system's requirements, it is primarily designed for high-spec environments, but alternative configurations and simplified guidance are provided to assist a broader range of users, including educators with limited technical experience.

### Prerequisites
To run TRINITY ID AI OS, ensure you have the following:
- **Operating System**: Linux system (developed on AMD Strix Halo hardware).
- **Hardware**: 128GB+ unified RAM recommended for full functionality.
- **Inference Engine**: [llama.cpp](https://github.com/ggml-org/llama.cpp) built with Vulkan support for GPU acceleration.
- **AI Model**: [Mistral Small 4 119B](https://huggingface.co/mistralai/Mistral-Small-4-119B-2503) GGUF (Q4_K_M) for the primary LLM brain (~68GB).
- **Database**: PostgreSQL 15+ with pgvector extension for state persistence and RAG.
- **Development Tools**: Node.js 18+ and Rust 1.80+ for building and running the system.

### Simplified Setup for Non-Technical Users
TRINITY ID AI OS is a complex system requiring technical setup, which may be challenging for educators without a background in software development. If you're not comfortable with command-line operations, consider the following options before proceeding with the detailed steps:
- **Institutional Support**: If you're part of a school or university (e.g., participating in a Purdue University pilot program), check with your IT department or program coordinator. They may provide a pre-configured server or setup assistance.
- **Community Resources**: Future updates to this manual will include links to video tutorials and community forums for visual step-by-step guidance (check the Regular Updates section for announcements).
- **Pre-Built Environments**: A roadmap item for TRINITY includes cloud-hosted versions or pre-installed virtual machine images to bypass local setup. Monitor system announcements for availability.

If you must set up TRINITY locally and lack technical support, follow the detailed steps below with the simplified explanations provided. Note that a basic understanding of terminal commands is still necessary, and you may need assistance from a technical colleague.

### Installation Steps
1. **Clone the Repository**:
   Clone the TRINITY ID AI OS repository to your local machine. If not already done, use the following command:
   ```bash
   git clone <repository-url>
   cd trinity-genesis
   ```
   *Simplified Explanation*: This step downloads the TRINITY software to your computer. Think of it as downloading a large file. Replace `<repository-url>` with the actual web address provided by your support contact or documentation. You'll type this in a terminal window (a text-based command tool on Linux).
2. **Set Up the LLM Server**:
   Start the LLM server using llama.cpp to serve the Mistral Small 4 model. TRINITY can auto-detect and launch this if not running, but for manual setup:
   ```bash
   llama-server -m ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
     --host 127.0.0.1 --port 8080 -ngl 99 --ctx-size 262144 --flash-attn on --jinja --parallel 2
   ```
   Ensure the model path matches your local setup. The port can be configured (common values: 8080, 1234). Set the `LLM_URL` environment variable if using a non-default port.
   *Simplified Explanation*: This starts the AI brain that powers TRINITY. It's like turning on the engine of a car. You need to ensure the path (e.g., `~/trinity-models/...`) points to where the AI model file is stored on your computer. If unsure, ask your IT support for help with file paths.
3. **Build and Run TRINITY**:
   Compile and start the TRINITY server using Cargo:
   ```bash
   cargo build --release
   cargo run --release
   ```
   The server will run on `http://localhost:3000` by default.
   *Simplified Explanation*: These commands build and launch the TRINITY application, similar to installing and opening a program. It may take a few minutes as it prepares everything. You'll see text output in the terminal indicating progress; wait until it stabilizes.
4. **Access the Interface**:
   Open a modern web browser and navigate to `http://localhost:3000` to access the TRINITY UI.
   *Simplified Explanation*: Once the server is running, open your web browser (like Chrome or Firefox) and type `http://localhost:3000` into the address bar. This is like visiting a website, but it's running on your own computer. You should see the TRINITY dashboard appear.

### Optional Sidecar Setup
For additional functionality like image generation or document intelligence, set up the following sidecars. These are optional and can be skipped if not needed:
- **Image Generation (ComfyUI)**:
  ```bash
  cd ~/ComfyUI && python main.py --port 8188 --listen 127.0.0.1
  ```
  *Simplified Explanation*: This starts a tool for creating images within TRINITY. It's like adding a graphics plugin to your system.
- **Document Intelligence (Qianfan-OCR Researcher)**:
  ```bash
  llama-server -m ~/trinity-models/gguf/Qianfan-OCR-Q4_K_M.gguf --port 8081 --ctx-size 32768
  ```
  *Simplified Explanation*: This enables TRINITY to analyze documents, useful for incorporating existing materials into lessons.
- **Voice Pipeline**:
  ```bash
  python scripts/voice_sidecar.py  # Runs on port 8200
  ```
  *Simplified Explanation*: This allows voice interaction with TRINITY, like speaking to Pete instead of typing.

### Configuration
- **Environment Variables**: Optionally set `LLM_URL` to specify the inference server if not on the default port (`http://127.0.0.1:8080`).
  *Simplified Explanation*: This is a setting to tell TRINITY where to find the AI brain if it's not in the usual place. Most users won't need to change this.
- **Database**: Ensure PostgreSQL is configured with the correct connection string in `.env` (default: `postgres://trinity:trinity@127.0.0.1:5432/trinity`). TRINITY will start without a database but quest saving will be disabled.
  *Simplified Explanation*: This sets up a storage system to save your progress. If not set up, you can still use TRINITY, but your work won't be saved between sessions.

### Alternative Configurations
For users without high-spec hardware:
- **Cloud Deployment**: Future updates plan for cloud-based LLM hosting to reduce local hardware demands (roadmap item noted in documentation). Check the Regular Updates section for announcements on availability.
- **Smaller Models**: Use lighter models like Crow 9B (~6GB) for testing on lower-spec systems, though functionality may be limited. Adjust the `LLM_URL` to point to the appropriate server. Consult with technical support to set this up if needed.

### Verification
After setup, verify the system is operational:
1. Check if the server is running at `http://localhost:3000`.
2. Confirm Pete responds to initial queries in the Iron Road interface.
3. Check system status in the UI to ensure LLM, database, and sidecars are connected.

*Simplified Explanation*: Once everything is running, go to `http://localhost:3000` in your browser. If you see the TRINITY dashboard and can chat with Pete (a welcome message or question appears), it's working. Look for a status area in the interface to confirm everything is connected (like a green light indicator).

If issues occur, refer to the Troubleshooting section for common problems and solutions. For non-technical users, don't hesitate to seek help from a technical colleague or IT support if the steps feel overwhelming.

## Troubleshooting and Known Issues

This section addresses common issues users may encounter while setting up or using TRINITY ID AI OS, along with solutions and workarounds based on the current system documentation and identified gaps. Specific error messages and UI indicators are included to aid in diagnostics.

### Common Setup Issues
- **LLM Connection Failure**:
  - **Symptoms**: TRINITY UI shows "No LLM detected" or Pete does not respond to queries. In the system status panel (accessible via the UI dashboard), the LLM indicator will be red or show "Disconnected."
  - **Cause**: The llama-server or alternative inference backend is not running or not accessible on the configured port (default: 8080).
  - **Error Log Example**: Server logs may show `Error: Failed to connect to LLM at http://127.0.0.1:8080 - Connection refused.`
  - **Solution**: Ensure the LLM server is running before starting TRINITY. Manually start it with the command provided in the Installation section. Verify the `LLM_URL` environment variable matches the server address (e.g., `http://127.0.0.1:8080`). TRINITY will auto-launch llama-server if none is detected, but check logs for errors if this fails (look for `Auto-launch failed: ...` in terminal output).
- **Database Connection Errors**:
  - **Symptoms**: Error messages about PostgreSQL connection or inability to save quest progress. UI may display a warning banner like "Database offline - Progress saving disabled."
  - **Cause**: PostgreSQL is not running, or the connection string in `.env` is incorrect.
  - **Error Log Example**: Logs might include `Database connection error: failed to connect to postgres://trinity:trinity@127.0.0.1:5432/trinity - Connection refused.`
  - **Solution**: Start PostgreSQL and verify the connection string (default: `postgres://trinity:trinity@127.0.0.1:5432/trinity`). Note that TRINITY can start without a database, but features like quest saving will be disabled. Check server logs for specific connection errors to pinpoint the issue.
- **Hardware Requirements Not Met**:
  - **Symptoms**: System crashes, slow performance, or inability to load models due to insufficient RAM or GPU capabilities. UI system status may show "Hardware Insufficient" or high usage metrics (e.g., RAM at 95%).
  - **Cause**: TRINITY's full functionality requires high-end hardware (128GB RAM, Vulkan GPU).
  - **Error Log Example**: Logs could report `Model load failed: Insufficient memory - 68GB required, 32GB available.`
  - **Solution**: For testing, use a smaller model like Crow 9B (~6GB) and adjust `LLM_URL` to point to its server. Consider future cloud deployment options (roadmap item) for lower-spec hardware. Check system status in the UI for hardware telemetry (under "Hardware" tab, look for red warnings on RAM or GPU).

### Runtime Issues
- **UI Not Updating (Resource Meters, Quest Progress)**:
  - **Symptoms**: Resource meters (Coal, Steam, Iron) or quest progress do not update after actions. The UI might show stale data or a spinning loader icon indefinitely.
  - **Cause**: Browser cache or connection issues with server-sent events (SSE).
  - **Error Log Example**: Browser console (accessible via F12 or right-click "Inspect") may show `SSE connection failed: disconnected from http://localhost:3000.`
  - **Solution**: Refresh the browser page (`Ctrl+R` or `Cmd+R`). Ensure no network interruptions between the browser and `http://localhost:3000`. Check browser console for errors (F12 or right-click "Inspect" to view JavaScript errors). If persistent, restart the TRINITY server to reset connections.
- **Sidecar Services Not Responding**:
  - **Symptoms**: Image generation (ComfyUI), voice features, or document intelligence fail with "service unavailable" errors. UI status indicators for sidecars will show red or "Offline" (e.g., "ComfyUI: Disconnected").
  - **Cause**: Sidecar services are not running or ports are misconfigured (ComfyUI: 8188, Voice: 8200, Qianfan-OCR: 8081).
  - **Error Log Example**: Server logs may include `Sidecar error: Failed to connect to ComfyUI at http://127.0.0.1:8188 - Timeout.`
  - **Solution**: Start the respective sidecar as outlined in the Installation section. Verify port availability and check TRINITY logs for connection attempts (search for `Sidecar health check failed` in terminal output). The system will auto-launch some sidecars if configured correctly; confirm this in the UI status panel.
- **Timeout Errors During Quest Execution**:
  - **Symptoms**: Quest steps fail with timeout messages in the UI (e.g., "Operation timed out after 300 seconds - Step skipped").
  - **Cause**: Complex queries or large context sizes exceed configured timeouts.
  - **Error Log Example**: Logs might show `Cow Catcher: LLM timeout: generate_code:src/main.rs took 300s (max 300s) on model Mistral.`
  - **Solution**: The Cow Catcher system will log the timeout and skip the step, allowing the quest to continue. Users can monitor logs for patterns (look for `Cow Catcher` entries in server output) and consider simplifying prompts or reducing context size in future interactions. No manual intervention is typically required; the UI will notify you of skipped steps and suggest next actions.

### Known Limitations
- **Hardware Dependency**: TRINITY's full functionality requires high-end hardware (128GB RAM, Vulkan GPU), which may exclude some users. Workarounds include using smaller models or awaiting cloud deployment options. UI indicators will warn if hardware is insufficient (check "Hardware" tab for red metrics).
- **Incomplete Features**: Some features like 3D Yard (Bevy integration) and advanced voice pipelines are in progress or deferred as of March 2026. These will be updated in future versions of the manual. The UI may show "Coming Soon" or grayed-out options for these features.
- **Technical User Base**: Current documentation assumes a technical user base familiar with command-line operations. Less technical users (e.g., teachers) may require additional onboarding support, which will be addressed in future updates. Look for "Help" or "Tutorial" sections in the UI for basic guidance if available.

### Getting Support
- **System Logs**: Check TRINITY server logs for detailed error messages (accessible via terminal where `cargo run` was executed). Search for keywords like `Error`, `Failed`, or `Timeout` to locate issues.
- **Documentation**: Refer to additional resources like `TRINITY_FANCY_BIBLE.md` and `CONTEXT.md` in the repository for deeper system insights. These can be found in the project folder or accessed via links in the UI if provided.
- **Community**: Future updates will include links to community forums or support channels for institutional adoption (e.g., Purdue University pilot programs). Check the Regular Updates section for announcements on support resources.

If an issue persists, note the error message and context, then consult the system status page in the TRINITY UI (`http://localhost:3000`, typically under a "Status" or "Settings" tab) for real-time health checks of subsystems like LLM, database, and sidecars. Copy any specific error codes or messages from logs or UI notifications to share with technical support for faster resolution.

## Validation of Claims and Resources

This section validates the visionary claims made about TRINITY ID AI OS's potential as a classroom server, its integration with XR/VR/AR systems, and its scalability for educational environments. It also provides relevant resources and maps the system's potential integration with Purdue University's global campus initiatives as an agnostic student and teacher tool, while addressing expanded use cases for parent engagement and special needs education.

### Validation of XR/VR/AR Scalability with BEVY
TRINITY's choice of **BEVY** as a foundational platform for 3D and immersive content is supported by ongoing developments in the BEVY community for XR (Extended Reality) capabilities:
- **OpenXR Support**: BEVY has active discussions and proof-of-concept projects for integrating OpenXR, a standard for VR and AR rendering. For instance, the GitHub issue [OpenXR: Virtual Reality Rendering #115](https://github.com/bevyengine/bevy/issues/115) highlights early interest in XR support, noting the need for render target integration with BEVY's rendering module. This aligns with TRINITY's roadmap for smart glasses and live training applications.
- **xrbevy Project**: The [xrbevy](https://github.com/blaind/xrbevy) repository by blaind demonstrates a proof-of-concept for OpenXR rendering in BEVY using gfx-rs abstractions, though it notes a more current implementation at [bevy_openxr](https://github.com/awtterpip/bevy_openxr). This validates BEVY's potential for XR, supporting TRINITY's vision for immersive educational tools.

These resources confirm that BEVY is a suitable choice for TRINITY's future development into VR, AR, and XR environments, ensuring the platform can deliver cutting-edge, interactive learning experiences as outlined in the system overview.

### Legal Considerations for AI in Classroom Servers
Integrating AI tools like TRINITY into classroom servers raises important legal considerations, particularly around data privacy and compliance with educational regulations. The following insights are derived from current literature and align with TRINITY's design for offline and local deployment to prioritize privacy:
- **Data Privacy Laws**: Key U.S. federal laws such as the **Family Educational Rights and Privacy Act (FERPA)** protect student data, requiring schools to ensure AI tools comply when sharing information with third parties. The **Children's Online Privacy Protection Act (COPPA)** mandates parental consent for data collection from children under 13. TRINITY's offline-first approach, as noted in the System Overview, helps mitigate these concerns by keeping data local (source: [Edutopia - AI and the Law](https://www.edutopia.org/article/laws-ai-education/)).
- **Algorithmic Accountability**: AI systems must avoid bias and discrimination under laws like **Title VI of the Civil Rights Act of 1964**. TRINITY's Socratic Protocol and PEARL focusing agent aim to ensure equitable content creation, though continuous monitoring is necessary (source: [TSH Anywhere - Legal Ramifications of AI in Education](https://www.tshanywhere.org/post/legal-issues-ai-education-key-insights)).
- **Accessibility Compliance**: For special needs education, laws like the **Individuals with Disabilities Education Act (IDEA)** and **Section 504 of the Rehabilitation Act** mandate accessible educational tools. TRINITY's potential customization for Individualized Education Programs (IEPs) and assistive technology integration aligns with these requirements, ensuring inclusivity (source: [TSH Anywhere - Legal Ramifications of AI in Education](https://www.tshanywhere.org/post/legal-issues-ai-education-key-insights)).
- **Compliance Practices**: Educators should set clear AI usage guidelines and vet tools for compliance. TRINITY supports this through its UI status indicators and health checks, allowing IT administrators to confirm system integrity and data handling (refer to the Troubleshooting section).

TRINITY's design as a local classroom server aligns with these legal requirements by minimizing external data sharing, and future updates will include detailed compliance guides as cloud options emerge.

### Mapping to Purdue University's Global Campus
TRINITY ID AI OS has significant potential for integration with Purdue University's global campus initiatives as an agnostic tool for students and teachers. Purdue's commitment to international education and technology integration provides a fertile ground for TRINITY's adoption:
- **Global Partnerships and Programs**: Purdue's Office of Global Partnerships and Programs fosters international collaboration and study abroad opportunities, as detailed on [Purdue GPP](https://www.purdue.edu/gpp/). TRINITY can serve as a unified platform for creating culturally adaptive educational content across global campuses, supporting Purdue's mission for a vibrant international community.
- **Technology Integration Programs**: Purdue offers programs like the [Technology Integration for Learning Graduate Certificate](https://education.purdue.edu/program/technology-integration-for-learning-online-graduate-certificate/), aimed at educators and administrators to design technology-enhanced learning. TRINITY's gamified instructional design and API extensibility align directly with this curriculum, offering a practical tool for certificate participants to apply learning in real-world scenarios.
- **Agnostic Tool Potential**: As an offline-first, customizable platform, TRINITY can plug into Purdue's global campus infrastructure as a neutral tool, supporting diverse student and teacher needs across disciplines. Its API allows integration with existing Learning Management Systems (LMS) at Purdue, while local deployment ensures compliance with data privacy for international students.
- **Institutional Recognition**: TRINITY's alignment with Purdue's innovation focus (noted in initial documentation for pilot programs) could position it as a cornerstone for edtech research and deployment, potentially funded through initiatives like Purdue's Global Impact Fund for international education projects.

### Expanded Use Cases and Audiences
TRINITY's vision extends beyond traditional classroom educators to include additional critical stakeholders in the educational ecosystem:
- **Parent and Guardian Engagement**: TRINITY can support at-home learning by providing parents and guardians with access to progress tracking, simplified interactions with Pete for homework guidance, and family-oriented mini-quests. This bridges home-school learning, enhancing student reinforcement and family buy-in for educational tools. Future development could include a dedicated parent portal or app mode with a simplified UI, leveraging existing APIs like `/api/quest` for progress data.
- **Special Needs Educators and Accessibility**: TRINITY's framework and AI personalization (VAAM, Pete's adaptive responses) can be tailored for special needs educators to create content aligned with Individualized Education Programs (IEPs) and accessibility standards (e.g., IDEA, Section 504). This includes prompts for accessible design (e.g., audio descriptions via voice pipeline) and integration with assistive technologies, positioning TRINITY as a leader in inclusive edtech.

### Additional Resources and Recognitions
To further validate TRINITY's claims and potential, the following resources and repositories are relevant:
- **BEVY Engine**: Official repository at [https://github.com/bevyengine/bevy](https://github.com/bevyengine/bevy) - A Rust-based game engine with growing support for XR, underpinning TRINITY's immersive content vision.
- **OpenXR for BEVY**: Community efforts like [bevy_openxr](https://github.com/awtterpip/bevy_openxr) demonstrate active development towards VR/AR integration, supporting TRINITY's scalability claims.
- **Purdue University Global**: Overview at [https://www.purdueglobal.edu/](https://www.purdueglobal.edu/) - Highlights Purdue's online and global reach, a potential deployment avenue for TRINITY as an educational tool.
- **Legal Frameworks for AI in Education**: Comprehensive guides from [Edutopia](https://www.edutopia.org/article/laws-ai-education/) and [TSH Anywhere](https://www.tshanywhere.org/post/legal-issues-ai-education-key-insights) provide frameworks for ensuring TRINITY's compliance with FERPA, COPPA, and other regulations.

These validations and resources underscore TRINITY's potential as a transformative educational tool, particularly within Purdue University's global campus network, ensuring its vision for classroom servers, XR integration, and inclusive use cases is grounded in feasible technology and legal compliance.

## Regular Updates

To ensure the TRINITY ID AI OS User Manual remains relevant and accurate, it will be updated regularly to reflect system changes, new features, and user feedback. This section outlines the plan for maintaining and updating the manual.

### Update Frequency
- **Monthly Reviews**: The manual will be reviewed monthly to incorporate minor updates, bug fixes, or user-reported issues.
- **Major Releases**: Significant updates will align with major system releases or milestones (e.g., new feature rollouts, architectural changes), expected quarterly or as announced.
- **Ad-Hoc Updates**: Critical issues or urgent feedback may prompt immediate updates to specific sections, ensuring users have access to the latest solutions.

### Sources of Updates
- **System Documentation**: Changes in core documents like `TRINITY_FANCY_BIBLE.md`, `CONTEXT.md`, and release notes will be monitored to update technical details and feature descriptions.
- **User Feedback**: Input from educators, technical users, and institutional partners (e.g., Purdue University pilot programs) will be collected to address usability concerns and improve clarity.
- **Development Roadmap**: Upcoming features (e.g., cloud deployment, 3D Yard integration) will be added as placeholders or detailed sections once implemented.

### Update Process
1. **Change Identification**: Monitor system logs, GitHub commits, and user forums for changes or issues requiring documentation updates.
2. **Content Revision**: Update relevant sections of the manual with accurate, verified information, ensuring alignment with the current system state.
3. **Version Control**: Increment the manual's version number (e.g., from 1.0 to 1.1 for minor updates, 2.0 for major revisions) and log the update date.
4. **User Notification**: Announce updates through the TRINITY UI or associated communication channels, highlighting key changes.

### Contributing to Updates
Users are encouraged to contribute to the manual by:
- Reporting errors or outdated information via the TRINITY interface or repository issues.
- Suggesting additional content (e.g., specific use cases, troubleshooting tips) for inclusion in future revisions.
- Participating in pilot programs or community forums to provide detailed feedback.

This proactive update strategy ensures the manual evolves alongside TRINITY ID AI OS, maintaining its utility as a comprehensive resource for all users.
