# Iron Road Demo Script
## TRINITY ID AI OS - Complete Demonstration Workflow

### Purpose
This script provides a step-by-step demonstration of the TRINITY ID AI OS "Iron Road" workflow for Purdue University faculty and institutional adoption. The demo showcases the gamified instructional design process through the 12 ADDIECRAPEYE stations.

### Prerequisites
- TRINITY ID AI OS compiled and running (`cargo run --bin trinity`)
- LLM server accessible (Mistral Small 4 119B via llama-server)
- PostgreSQL database running
- Browser pointed to `http://localhost:3000`

---

## Demo Script - Phase 1: System Introduction (2 minutes)

### Step 1: Welcome & Overview
**Speaker**: "Welcome to TRINITY ID AI OS. I'm demonstrating our 'Iron Road' workflow - a gamified approach to instructional design that makes course creation engaging and systematic."

**Actions**:
1. Show the main Trinity interface
2. Highlight the three core components:
   - **Pete** (Instructional Design Mentor)
   - **ART** (AI Creative Assistant) 
   - **Yardmaster** (System Orchestrator)

**Screen**: Main Trinity dashboard showing all three agents

### Step 2: The Iron Road Concept
**Speaker**: "The Iron Road is a 12-station journey based on ADDIECRAPEYE methodology. Each station represents a phase in instructional design, mapped to Bloom's Taxonomy with increasing difficulty levels."

**Actions**:
1. Navigate to the Iron Road interface
2. Show the 12 stations with their Bloom's levels
3. Highlight the current station (Analysis)

**Screen**: Iron Road station map with progress indicators

---

## Demo Script - Phase 2: Beginning the Journey (3 minutes)

### Step 3: Starting at Analysis Station
**Speaker**: "We begin at the Analysis station where Pete helps us understand our instructional goals. Let's design a course on 'Introduction to Machine Learning'."

**Actions**:
1. Click on Analysis station
2. Type course topic: "Introduction to Machine Learning"
3. Submit to Pete for Socratic questioning

**Expected Response**: Pete asks clarifying questions about audience, prerequisites, learning objectives

**Screen**: Chat interface with Pete's Socratic questions

### Step 4: VAAM System in Action
**Speaker**: "Notice the VAAM (Vocabulary Acquisition And Mastery) system tracking our vocabulary complexity. We're currently at Tier 1 - using accessible language."

**Actions**:
1. Point to VAAM tier indicator
2. Show vocabulary complexity meter
3. Demonstrate how Pete adjusts responses based on VAAM

**Screen**: VAAM interface showing Tier 1 status and vocabulary analysis

---

## Demo Script - Phase 3: The Gamified Mechanics (4 minutes)

### Step 5: Scope Creep Battle
**Speaker**: "Here's where it gets interesting - Scope Creep monsters appear when we try to add too much complexity. We need to battle them with skill checks."

**Actions**:
1. Intentionally suggest a complex feature
2. Watch Scope Creep monster appear
3. Demonstrate skill check mechanic (roll d20 + skill bonus)

**Expected Result**: Show success/failure and coal resource management

**Screen**: Scope Creep battle interface with dice rolls and resources

### Step 6: Resource Management
**Speaker**: "We manage three resources: Coal (compute budget), Steam (creative energy), and Iron (structural integrity). These affect our ability to tackle complex tasks."

**Actions**:
1. Show resource meters
2. Explain how successful battles increase resources
3. Demonstrate resource allocation decisions

**Screen**: Resource management panel with current values

---

## Demo Script - Phase 4: Progressive Stations (5 minutes)

### Step 7: Design Station with ART
**Speaker**: "Moving to Design station, ART helps us create visual metaphors and learning activities. ART is our creative AI assistant."

**Actions**:
1. Navigate to Design station
2. Request visual metaphor for ML concepts
3. Show ART's creative suggestions

**Expected Response**: ART suggests visual analogies and learning activities

**Screen**: ART interface with creative suggestions and mood boards

### Step 8: Development Station
**Speaker**: "At Development station, Yardmaster helps structure the actual content. This is where we build the learning materials."

**Actions**:
1. Move to Development station
2. Request lesson structure for "Neural Networks Basics"
3. Show Yardmaster's content organization

**Screen**: Yardmaster interface with content templates and structure

---

## Demo Script - Phase 5: Advanced Features (4 minutes)

### Step 9: Semantic Creep Taming
**Speaker**: "As we progress, we encounter Semantic Creeps - words that can be tamed and used as power-ups. Each word has elemental properties based on its part of speech."

**Actions**:
1. Show Semantic Creep encounter
2. Demonstrate word taming mechanic
3. Explain elemental system (Earth, Air, Fire, Water)

**Screen**: Semantic Creep battle with elemental word analysis

### Step 10: Sacred Circuitry System
**Speaker**: "The Sacred Circuitry represents different learning modalities. We rotate through circuits to maintain engagement and address different learning styles."

**Actions**:
1. Show current circuit (e.g., Analytical)
2. Demonstrate circuit rotation
3. Explain how circuits affect interaction style

**Screen**: Sacred Circuitry display with current active circuit

---

## Demo Script - Phase 6: Completion & Assessment (3 minutes)

### Step 11: Final Stations
**Speaker**: "As we approach the final stations (Planning, Extension, Yield, Execution), the difficulty increases but so does our capability."

**Actions**:
1. Show progression through higher stations
2. Demonstrate advanced Pete interactions
3. Show final course structure generation

**Screen**: Final course output with complete lesson plans

### Step 12: Character Sheet & Progress
**Speaker**: "Throughout the journey, we've been building our character sheet - tracking skills, accomplishments, and learning design capabilities."

**Actions**:
1. Display final character sheet
2. Show skill progression in different ID areas
3. Highlight achievements and badges earned

**Screen**: Character sheet with comprehensive skill tracking

---

## Demo Script - Phase 7: Technical Features (2 minutes)

### Step 13: System Architecture
**Speaker**: "Behind the scenes, TRINITY uses a 'True Trinity' architecture - single LLM with specialized personas, not multiple models."

**Actions**:
1. Show system status screen
2. Explain the single-brain approach
3. Highlight offline-first capability

**Screen**: System status showing all components connected

### Step 14: Export & Integration
**Speaker**: "Finally, we can export our course design in various formats for LMS integration or further refinement."

**Actions**:
1. Demonstrate export options
2. Show LMS compatibility formats
3. Explain institutional integration possibilities

**Screen**: Export interface with format options

---

## Demo Script - Conclusion (1 minute)

### Step 15: Summary & Call to Action
**Speaker**: "TRINITY ID AI OS transforms instructional design from a chore into an engaging journey. The Iron Road provides structure while maintaining creativity and pedagogical rigor."

**Key Points to Emphasize**:
- Gamification increases engagement
- VAAM system ensures accessibility
- Single LLM architecture reduces complexity
- Offline-first protects institutional data
- Built on proven instructional design frameworks

**Final Screen**: Complete Iron Road journey with all 12 stations completed

---

## Technical Notes for Demo

### System Requirements
- Rust toolchain installed
- PostgreSQL running locally
- llama-server running with Mistral Small 4 119B
- Modern web browser

### Common Demo Issues & Solutions
1. **LLM Connection**: Ensure llama-server is running before starting
2. **Database Issues**: Check PostgreSQL connection string in .env
3. **Resource Display**: Refresh browser if resource meters don't update
4. **Voice Features**: Note that voice is handled by Python sidecar (optional for demo)

### Backup Plans
- Have screenshots ready for each major step
- Prepare sample responses if live LLM is slow
- Keep a simplified demo version available (skip advanced features)

### Timing Tips
- Total demo time: ~24 minutes
- Allow 5-10 minutes for questions
- Have extended version ready (45 minutes) for interested faculty
- Prepare 5-minute "elevator pitch" version for quick demonstrations

---

## Post-Demo Follow-up

### For Faculty Interested in Adoption
1. Schedule one-on-one consultation
2. Provide sandbox environment access
3. Share technical documentation
4. Discuss pilot course opportunities

### Technical Support Contacts
- Primary: [Your email]
- Technical: [Dev team contact]
- Documentation: Link to TRINITY_TECHNICAL_BIBLE.md

### Next Steps for Interested Institutions
1. Technical requirements assessment
2. Pilot program setup
3. Faculty training sessions
4. LMS integration planning
5. Student onboarding strategy

---

This demo script provides a comprehensive showcase of TRINITY ID AI OS capabilities while maintaining engagement and demonstrating practical value for educational institutions.
