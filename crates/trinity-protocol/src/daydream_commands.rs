// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — DAYDREAM Command Protocol
// ═══════════════════════════════════════════════════════════════════════════════
//
// PHILOSOPHY:
//   DAYDREAM is NOT something WE build — it's a canvas that PETE builds
//   for each user, driven by their PEARL and ADDIECRAPEYE progression.
//   This module defines the command language Pete speaks to shape the 3D world.
//
//   Pete (Mistral MS4) receives:
//     1. The user's PEARL (subject + medium + vision)
//     2. The current ADDIECRAPEYE station (1-12)
//     3. The current PearlPhase (Extracting → Placing → Refining → Polished)
//
//   Pete emits DaydreamCommands as JSON to the Bevy world:
//     Station 1 (Analyze):   Spawn terrain themed by subject
//     Station 3 (Develop):   Spawn entities representing lesson concepts
//     Station 7 (Repetition): Add game loop mechanics
//     Station 12 (Evolve):   The world is alive — user explores freely
//
// ARCHITECTURE:
//   Agent Tool `daydream_command` → JSON → DaydreamCommandQueue → ECS system
//
//   This protocol is ISOLATED — it depends only on trinity-protocol types
//   so Pete can work on DAYDREAM without touching the rest of the Bevy code.
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use crate::pearl::{PearlMedium, PearlPhase};

/// Unique identifier for DAYDREAM entities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DaydreamEntityId(pub String);

// ═══════════════════════════════════════════════════════════════════════════════
// DAYDREAM BLUEPRINT — What Pete sends to shape the world
// ═══════════════════════════════════════════════════════════════════════════════

/// A DAYDREAM Blueprint is what Pete creates at each ADDIECRAPEYE phase transition.
/// It represents a batch of world-shaping intentions, not individual commands.
///
/// This is the HIGH-LEVEL interface Pete uses — the "what", not the "how".
/// The DaydreamPlugin translates blueprints into ECS mutations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaydreamBlueprint {
    /// Which PEARL phase this blueprint was created for
    pub pearl_phase: PearlPhase,

    /// Which ADDIECRAPEYE station triggered this blueprint
    pub station: u8,

    /// The user's subject (from PEARL) — drives theming
    pub subject: String,

    /// The user's medium (from PEARL) — drives interaction style
    pub medium: PearlMedium,

    /// The user's vision (from PEARL) — the emotional target
    pub vision: String,

    /// Commands to execute in order
    pub commands: Vec<DaydreamCommand>,
}

impl DaydreamBlueprint {
    /// Create a blueprint for a given PEARL state and station.
    pub fn new(subject: &str, medium: PearlMedium, vision: &str, station: u8) -> Self {
        Self {
            pearl_phase: PearlPhase::from_station(station),
            station,
            subject: subject.to_string(),
            medium,
            vision: vision.to_string(),
            commands: Vec::new(),
        }
    }

    /// Add a command to this blueprint.
    pub fn add(&mut self, cmd: DaydreamCommand) {
        self.commands.push(cmd);
    }

    /// How the world should FEEL based on the current PEARL phase.
    /// This injects into Pete's system prompt so the AI knows the vibe.
    pub fn phase_atmosphere(&self) -> &'static str {
        match self.pearl_phase {
            PearlPhase::Extracting => {
                // ADDIE (1-5): Discovering wisdom — world is misty, forming, potential
                "The world is partially formed — fog and possibility. Terrain emerges \
                 as knowledge crystallizes. The landscape reflects the subject: gentle \
                 hills for humanities, geometric forms for math, organic growth for science."
            }
            PearlPhase::Placing => {
                // CRAP (6-9): Designing the experience — world gains structure
                "The world has structure now — paths, landmarks, clear visual hierarchy. \
                 Contrast highlights key concepts. Repetition creates a core game loop. \
                 Alignment ensures everything serves the PEARL vision."
            }
            PearlPhase::Refining => {
                // EYE (10-12): Reflecting and shipping — world is alive
                "The world is vibrant and alive — NPCs move with purpose, the environment \
                 responds to the user's voice, lighting matches the emotional vision. \
                 This is the DAYDREAM as the user imagined it."
            }
            PearlPhase::Polished => {
                // Complete — the pearl is manifest
                "The world IS the PEARL made manifest. Every element serves the vision. \
                 The user can explore freely. This is their DAYDREAM — they built it."
            }
        }
    }

    /// Suggested terrain style for the user's subject domain.
    pub fn subject_terrain(&self) -> TerrainTheme {
        let subject_lower = self.subject.to_lowercase();
        if subject_lower.contains("math") || subject_lower.contains("geometry") || subject_lower.contains("algebra") {
            TerrainTheme::Geometric
        } else if subject_lower.contains("science") || subject_lower.contains("biology") || subject_lower.contains("chemistry") {
            TerrainTheme::Crystal
        } else if subject_lower.contains("history") || subject_lower.contains("social") {
            TerrainTheme::Ruins
        } else if subject_lower.contains("english") || subject_lower.contains("language") || subject_lower.contains("writing") {
            TerrainTheme::Garden
        } else if subject_lower.contains("art") || subject_lower.contains("music") {
            TerrainTheme::Ethereal
        } else {
            TerrainTheme::Meadow
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DAYDREAM COMMANDS — Individual world mutations
// ═══════════════════════════════════════════════════════════════════════════════

/// All commands that can mutate the DAYDREAM world.
/// Pete emits these via the `daydream_command` agent tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", content = "params")]
pub enum DaydreamCommand {
    // ── World Foundation (ADDIE — Extracting Phase) ──────────────
    /// Set the terrain based on the user's subject domain.
    /// Pete calls this when the PEARL subject is identified (Station 1-2).
    SetTerrain {
        theme: TerrainTheme,
        #[serde(default = "default_seed")]
        seed: u32,
    },

    /// Set world atmosphere — lighting, fog, time of day.
    /// Changes as the PEARL phase advances.
    SetAtmosphere {
        time_of_day: f32,    // 0.0 = midnight, 0.5 = noon
        fog_density: f32,    // 0.0 = clear, 1.0 = thick fog
        mood: WorldMood,
    },

    // ── Entity Management (ADDIE → CRAP transition) ─────────────
    /// Spawn a concept entity — represents a learning objective in 3D space.
    /// Pete maps PEARL subject vocabulary to physical objects.
    SpawnConcept {
        id: DaydreamEntityId,
        label: String,
        position: [f32; 3],
        #[serde(default)]
        mesh_type: MeshType,
        /// Which ADDIECRAPEYE station this concept belongs to
        #[serde(default)]
        station: Option<u8>,
        /// Optional PyO3 script for complex interaction logic
        #[serde(default)]
        python_script: Option<String>,
    },

    /// Remove an entity from the world.
    DespawnEntity { id: DaydreamEntityId },

    /// Move an entity smoothly to a new position.
    MoveEntity {
        id: DaydreamEntityId,
        target: [f32; 3],
        #[serde(default = "default_speed")]
        speed: f32,
    },

    // ── Avatar (Voice-Driven Character) ─────────────────────────
    /// Spawn the Yardmaster's avatar.
    /// Called once when the user first enters DAYDREAM.
    SpawnAvatar {
        position: [f32; 3],
    },

    // ── NPC Layer (CRAP → EYE transition) ────────────────────────
    /// Spawn a Pete NPC — the AI guide in 3D space.
    /// Pete manifests as a character the user can talk to.
    SpawnPeteNpc {
        position: [f32; 3],
        /// Pete's current persona: "Great Recycler" or "Programmer Pete"
        persona: String,
    },

    /// Spawn a subject-specific NPC (e.g., "Newton" for physics).
    /// These NPCs speak using VAAM vocabulary from the user's domain.
    SpawnSubjectNpc {
        id: DaydreamEntityId,
        name: String,
        role: String,
        position: [f32; 3],
        /// Vocabulary domain this NPC draws from
        vocabulary_domain: Option<String>,
    },

    /// NPC speaks — triggers TTS and floating text.
    NpcSpeak {
        id: DaydreamEntityId,
        text: String,
        /// VAAM emotion for TTS prosody
        #[serde(default)]
        emotion: Option<String>,
    },

    // ── Game Loop (CRAP Phase — Repetition Station) ──────────────
    /// Place a quest waypoint in the world.
    /// Maps to ADDIECRAPEYE objectives — completing the waypoint
    /// advances the quest and burns coal.
    PlaceWaypoint {
        id: DaydreamEntityId,
        label: String,
        position: [f32; 3],
        /// Which quest objective this waypoint completes
        objective_index: Option<u32>,
    },

    /// Mark a waypoint as completed (visual + physics change).
    CompleteWaypoint {
        id: DaydreamEntityId,
    },

    // ── Camera Control ───────────────────────────────────────────
    /// Focus camera on an entity or position.
    FocusCamera {
        target: CameraTarget,
    },

    // ── Tier 3 Sensories & UI ────────────────────────────────────
    /// Play a sound effect or ambient track
    PlaySound {
        src: String,
        #[serde(default)]
        spatial_entity: Option<DaydreamEntityId>,
    },

    /// Trigger an animation on a spawned entity
    AnimateEntity {
        id: DaydreamEntityId,
        clip: String,
        #[serde(default)]
        loop_anim: bool,
    },

    /// Spawn a UI context button (e.g. Next Quest)
    /// Emits a daydream_event when clicked by the user
    SpawnUiButton {
        id: DaydreamEntityId,
        text: String,
        position: [f32; 2],
        event_trigger: String,
    },

    /// Spawn a branching Socratic Dialogue Tree
    /// Emits a daydream_event when the user selects a dialogue node
    SpawnDialogueTree {
        id: DaydreamEntityId,
        npc_id: DaydreamEntityId,
        nodes: Vec<DialogueNode>,
    },

    // ── Interaction ──────────────────────────────────
    /// Log a message back to the developer console
    LogMessage {
        msg: String,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
// SUPPORTING TYPES — All driven by PEARL and ADDIECRAPEYE
// ═══════════════════════════════════════════════════════════════════════════════

/// Terrain themes mapped to VAAM vocabulary domains.
/// Pete selects based on the user's PEARL subject.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TerrainTheme {
    /// Default — rolling green hills (general subjects)
    #[default]
    Meadow,
    /// Science/STEM → crystalline landscape
    Crystal,
    /// History/Social Studies → ancient structures
    Ruins,
    /// Math → procedural geometric terrain
    Geometric,
    /// Language Arts → organic garden landscape
    Garden,
    /// Art/Music → ethereal floating islands
    Ethereal,
    /// User-defined via PEARL vision
    Custom { name: String, seed: u32 },
}

/// World mood — shifts as the PEARL phase advances.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WorldMood {
    /// Extracting phase — mysterious, potential-filled
    #[default]
    Emergence,
    /// Placing phase — structured, clear
    Structured,
    /// Refining phase — vibrant, alive
    Vibrant,
    /// Polished — transcendent, the PEARL manifest
    Transcendent,
}

impl From<PearlPhase> for WorldMood {
    fn from(phase: PearlPhase) -> Self {
        match phase {
            PearlPhase::Extracting => WorldMood::Emergence,
            PearlPhase::Placing => WorldMood::Structured,
            PearlPhase::Refining => WorldMood::Vibrant,
            PearlPhase::Polished => WorldMood::Transcendent,
        }
    }
}

/// Types of meshes that can be spawned in DAYDREAM.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum MeshType {
    #[default]
    Cube,
    Sphere,
    Capsule,
    Cylinder,
    Plane,
    /// Load a glTF model by path
    Gltf { path: String },
}

/// Camera target for focus commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CameraTarget {
    /// Focus on a named entity
    Entity(DaydreamEntityId),
    /// Focus on a world position
    Position([f32; 3]),
    /// Return to default orbit position
    Default,
}

/// Dialogue tree node used for branching Socratic conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: String,
    pub text: String,
    pub choices: Vec<DialogueChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub text: String,
    pub event_trigger: String,
}



// ── Default helpers ──────────────────────────────────────────────────────────

fn default_speed() -> f32 { 5.0 }
fn default_seed() -> u32 { 42 }

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS — Verify serialization round-trips and PEARL integration
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blueprint_creation_from_pearl() {
        let bp = DaydreamBlueprint::new(
            "Physics",
            PearlMedium::Game,
            "Students discover gravity through play",
            3, // Station 3 = Develop (ADDIE)
        );
        assert_eq!(bp.pearl_phase, PearlPhase::Extracting);
        assert_eq!(bp.station, 3);
        assert_eq!(bp.subject, "Physics");
    }

    #[test]
    fn test_subject_terrain_mapping() {
        let bp = DaydreamBlueprint::new("Mathematics", PearlMedium::Game, "", 1);
        assert!(matches!(bp.subject_terrain(), TerrainTheme::Geometric));

        let bp2 = DaydreamBlueprint::new("Biology 101", PearlMedium::Simulation, "", 1);
        assert!(matches!(bp2.subject_terrain(), TerrainTheme::Crystal));

        let bp3 = DaydreamBlueprint::new("American History", PearlMedium::Storyboard, "", 1);
        assert!(matches!(bp3.subject_terrain(), TerrainTheme::Ruins));

        let bp4 = DaydreamBlueprint::new("Creative Writing", PearlMedium::Book, "", 1);
        assert!(matches!(bp4.subject_terrain(), TerrainTheme::Garden));
    }

    #[test]
    fn test_pearl_phase_to_world_mood() {
        assert!(matches!(WorldMood::from(PearlPhase::Extracting), WorldMood::Emergence));
        assert!(matches!(WorldMood::from(PearlPhase::Placing), WorldMood::Structured));
        assert!(matches!(WorldMood::from(PearlPhase::Refining), WorldMood::Vibrant));
        assert!(matches!(WorldMood::from(PearlPhase::Polished), WorldMood::Transcendent));
    }

    #[test]
    fn test_command_serialization() {
        let cmd = DaydreamCommand::SpawnConcept {
            id: DaydreamEntityId("gravity-1".into()),
            label: "Newton's First Law".into(),
            position: [5.0, 1.0, 0.0],
            mesh_type: MeshType::Sphere,
            station: Some(3),
        };
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("SpawnConcept"));
        assert!(json.contains("Newton"));

        let parsed: DaydreamCommand = serde_json::from_str(&json).unwrap();
        match parsed {
            DaydreamCommand::SpawnConcept { id, station, .. } => {
                assert_eq!(id.0, "gravity-1");
                assert_eq!(station, Some(3));
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_blueprint_serialization_roundtrip() {
        let mut bp = DaydreamBlueprint::new(
            "Chemistry",
            PearlMedium::Simulation,
            "Students feel like alchemists",
            7,
        );
        bp.add(DaydreamCommand::SetTerrain {
            theme: TerrainTheme::Crystal,
            seed: 42,
        });
        bp.add(DaydreamCommand::SpawnConcept {
            id: DaydreamEntityId("element-h".into()),
            label: "Hydrogen".into(),
            position: [0.0, 2.0, 0.0],
            mesh_type: MeshType::Sphere,
            station: Some(7),
        });

        let json = serde_json::to_string(&bp).unwrap();
        let restored: DaydreamBlueprint = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.subject, "Chemistry");
        assert_eq!(restored.commands.len(), 2);
        assert_eq!(restored.pearl_phase, PearlPhase::Placing); // Station 7 = CRAP
    }

    #[test]
    fn test_place_waypoint_for_quest() {
        let cmd = DaydreamCommand::PlaceWaypoint {
            id: DaydreamEntityId("wp-1".into()),
            label: "Define Learning Objectives".into(),
            position: [10.0, 1.0, 5.0],
            objective_index: Some(0),
        };
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("PlaceWaypoint"));
        assert!(json.contains("objective_index"));
    }

    #[test]
    fn test_phase_atmosphere_text() {
        let bp = DaydreamBlueprint::new("Art", PearlMedium::Game, "", 1);
        assert!(bp.phase_atmosphere().contains("fog"));

        let bp2 = DaydreamBlueprint::new("Art", PearlMedium::Game, "", 7);
        assert!(bp2.phase_atmosphere().contains("structure"));

        let bp3 = DaydreamBlueprint::new("Art", PearlMedium::Game, "", 11);
        assert!(bp3.phase_atmosphere().contains("vibrant"));
    }
}
