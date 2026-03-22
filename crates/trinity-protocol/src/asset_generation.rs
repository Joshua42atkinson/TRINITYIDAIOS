// Trinity ID AI OS - Behavior to Content Mapper
// Copyright (c) Joshua
//
// ═══════════════════════════════════════════════════════════════════════════════
// 🎯 ZONE: PROTOCOL | Module: Asset Generation
// ═══════════════════════════════════════════════════════════════════════════════
// Maps observable behaviors to content generation requirements.
// Transforms Action Mapping outputs into structured Yardmaster specifications.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::id_contract::{ActionMap, LearningObjective, QuestMilestone};

/// A Yardmaster (Learning Object) specification generated from behavioral analysis.
/// Yardmaster are the atomic units of learning content in Trinity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Yardmaster {
    /// Unique identifier for this learning object
    pub id: Uuid,
    /// Title of the learning content
    pub title: String,
    /// Type of learning content (practice, assessment, demonstration, etc.)
    pub content_type: YardmasterType,
    /// The specific behavior this Yardmaster addresses
    pub target_behavior: String,
    /// Bloom's taxonomy level this content targets
    pub bloom_level: crate::character_sheet::BloomLevel,
    /// Content generation template to use
    pub template: ContentTemplate,
    /// Required resources for this Yardmaster
    pub resources: Vec<ContentResource>,
    /// Assessment criteria for this learning object
    pub assessment_criteria: Vec<AssessmentCriterion>,
    /// Estimated cognitive load (coal units)
    pub cognitive_load: f32,
    /// Prerequisite Yardmaster (if any)
    pub prerequisites: Vec<Uuid>,
}

/// Types of Yardmaster (Learning Objects)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum YardmasterType {
    /// Practice exercises and activities
    Practice,
    /// Knowledge assessment items
    Assessment,
    /// Demonstration or example content
    Demonstration,
    /// Interactive simulation
    Simulation,
    /// Reading or reference material
    Reference,
    /// Collaborative learning activity
    Collaborative,
}

/// Content generation templates for different learning objectives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTemplate {
    /// Template identifier
    pub name: String,
    /// Template category (practice, assessment, etc.)
    pub category: String,
    /// Template structure with placeholders
    pub structure: TemplateStructure,
    /// Required input parameters
    pub parameters: Vec<TemplateParameter>,
    /// Generation instructions
    pub instructions: String,
}

/// Template structure definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStructure {
    /// Sections of the content
    pub sections: Vec<TemplateSection>,
    /// Interactive elements
    pub interactions: Vec<InteractionPattern>,
    /// Assessment integration points
    pub assessment_points: Vec<AssessmentPoint>,
}

/// Template section definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    /// Section identifier
    pub id: String,
    /// Section type (introduction, practice, assessment, etc.)
    pub section_type: SectionType,
    /// Content template with placeholders
    pub content_template: String,
    /// Order in the template
    pub order: u32,
}

/// Section types in learning content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SectionType {
    Introduction,
    Objective,
    Theory,
    Example,
    Practice,
    Assessment,
    Feedback,
    Summary,
}

/// Interactive element patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionPattern {
    /// Pattern identifier
    pub id: String,
    /// Type of interaction
    pub interaction_type: InteractionType,
    /// Configuration for the interaction
    pub config: InteractionConfig,
}

/// Types of interactions in learning content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InteractionType {
    /// Multiple choice question
    MultipleChoice,
    /// Text input field
    TextInput,
    /// Drag and drop exercise
    DragDrop,
    /// Simulation interaction
    Simulation,
    /// Code editor
    CodeEditor,
    /// File upload
    FileUpload,
}

/// Interaction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConfig {
    /// Configuration parameters
    pub parameters: std::collections::HashMap<String, String>,
    /// Validation rules
    pub validation: Vec<ValidationRule>,
}

/// Validation rule for interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule type
    pub rule_type: String,
    /// Rule parameters
    pub parameters: std::collections::HashMap<String, String>,
    /// Error message
    pub error_message: String,
}

/// Assessment point in template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentPoint {
    /// Point identifier
    pub id: String,
    /// Assessment type
    pub assessment_type: AssessmentType,
    /// Scoring criteria
    pub scoring_criteria: ScoringCriteria,
}

/// Assessment types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssessmentType {
    /// Formative assessment (learning check)
    Formative,
    /// Summative assessment (evaluation)
    Summative,
    /// Competency check
    Competency,
    /// Performance assessment
    Performance,
}

/// Scoring criteria for assessments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringCriteria {
    /// Maximum score
    pub max_score: f32,
    /// Passing threshold
    pub passing_threshold: f32,
    /// Scoring rubric
    pub rubric: Vec<ScoringRubricItem>,
}

/// Scoring rubric item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringRubricItem {
    /// Score value
    pub score: f32,
    /// Description of performance at this level
    pub description: String,
    /// Performance indicators
    pub indicators: Vec<String>,
}

/// Template parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: ParameterType,
    /// Whether this parameter is required
    pub required: bool,
    /// Default value (if any)
    pub default_value: Option<String>,
    /// Parameter description
    pub description: String,
}

/// Parameter types for templates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterType {
    /// Text string
    Text,
    /// Numeric value
    Number,
    /// Boolean flag
    Boolean,
    /// List of values
    List,
    /// Complex object
    Object,
}

/// Content resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentResource {
    /// Resource identifier
    pub id: String,
    /// Resource type
    pub resource_type: ResourceType,
    /// Resource description
    pub description: String,
    /// Whether this resource is required
    pub required: bool,
}

/// Resource types for content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    /// Image file
    Image,
    /// Video file
    Video,
    /// Audio file
    Audio,
    /// Document file
    Document,
    /// Interactive component
    Interactive,
    /// External link
    Link,
}

/// Assessment criterion for Yardmaster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentCriterion {
    /// Criterion identifier
    pub id: String,
    /// Criterion description
    pub description: String,
    /// Measurement method
    pub measurement_method: MeasurementMethod,
    /// Success criteria
    pub success_criteria: String,
}

/// Methods for measuring learning outcomes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeasurementMethod {
    /// Direct observation
    Observation,
    /// Automated scoring
    Automated,
    /// Peer evaluation
    Peer,
    /// Self assessment
    SelfAssessment,
    /// Instructor evaluation
    Instructor,
}

/// Behavior analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAnalysis {
    /// Analyzed behaviors
    pub behaviors: Vec<AnalyzedBehavior>,
    /// Content requirements derived from behaviors
    pub content_requirements: Vec<ContentRequirement>,
    /// Recommended Yardmaster
    pub recommended_yardmaster: Vec<Yardmaster>,
}

/// Individual behavior analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzedBehavior {
    /// The behavior text
    pub behavior: String,
    /// Action verb identified
    pub action_verb: String,
    /// Cognitive level required
    pub cognitive_level: crate::character_sheet::BloomLevel,
    /// Content complexity
    pub complexity: ComplexityLevel,
    /// Required practice types
    pub practice_types: Vec<YardmasterType>,
    /// Assessment methods recommended
    pub assessment_methods: Vec<AssessmentType>,
}

/// Complexity levels for content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplexityLevel {
    /// Simple, single-step tasks
    Simple,
    /// Moderate complexity, multiple steps
    Moderate,
    /// Complex, multi-component tasks
    Complex,
    /// Expert-level, sophisticated tasks
    Expert,
}

/// Content requirement derived from behavior analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRequirement {
    /// Requirement identifier
    pub id: String,
    /// Requirement type
    pub requirement_type: RequirementType,
    /// Requirement description
    pub description: String,
    /// Priority level
    pub priority: Priority,
}

/// Types of content requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RequirementType {
    /// Knowledge content needed
    Knowledge,
    /// Practice activities needed
    Practice,
    /// Assessment items needed
    Assessment,
    /// Demonstration content needed
    Demonstration,
    /// Resource materials needed
    Resource,
}

/// Priority levels for requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    /// Critical - must include
    Critical,
    /// High - should include
    High,
    /// Medium - nice to include
    Medium,
    /// Low - optional
    Low,
}

/// Behavior to Content Mapper - Main engine
pub struct BehaviorToContentMapper;

impl BehaviorToContentMapper {
    /// Analyze observable behaviors and generate Yardmaster specifications
    pub fn analyze_behaviors(
        action_map: &ActionMap,
        learning_objectives: &[LearningObjective],
        milestones: &[QuestMilestone],
    ) -> BehaviorAnalysis {
        let mut analyzed_behaviors = Vec::new();
        let mut content_requirements = Vec::new();
        let mut recommended_yardmaster = Vec::new();

        // Parse observable behaviors
        let behaviors = Self::parse_behaviors(&action_map.observable_behaviors);

        for behavior in behaviors {
            let analyzed = Self::analyze_single_behavior(&behavior, learning_objectives);
            analyzed_behaviors.push(analyzed.clone());

            // Generate content requirements
            let requirements = Self::generate_content_requirements(&analyzed);
            content_requirements.extend(requirements);

            // Generate recommended Yardmaster
            let yardmaster_items = Self::generate_yardmaster(&analyzed, milestones);
            recommended_yardmaster.extend(yardmaster_items);
        }

        BehaviorAnalysis {
            behaviors: analyzed_behaviors,
            content_requirements,
            recommended_yardmaster,
        }
    }

    /// Parse observable behaviors into individual behavior statements
    fn parse_behaviors(behaviors_text: &str) -> Vec<String> {
        behaviors_text
            .split('.')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    /// Analyze a single behavior statement
    fn analyze_single_behavior(
        behavior: &str,
        learning_objectives: &[LearningObjective],
    ) -> AnalyzedBehavior {
        let action_verb = Self::extract_action_verb(behavior);
        let cognitive_level = Self::determine_cognitive_level(&action_verb, learning_objectives);
        let complexity = Self::assess_complexity(behavior, &cognitive_level);
        let practice_types = Self::recommend_practice_types(&action_verb, &cognitive_level);
        let assessment_methods = Self::recommend_assessment_methods(&cognitive_level);

        AnalyzedBehavior {
            behavior: behavior.to_string(),
            action_verb,
            cognitive_level,
            complexity,
            practice_types,
            assessment_methods,
        }
    }

    /// Extract action verb from behavior statement
    fn extract_action_verb(behavior: &str) -> String {
        // Common action verbs in instructional design
        let action_verbs = vec![
            "identify",
            "list",
            "define",
            "describe",
            "explain",
            "summarize",
            "compare",
            "contrast",
            "analyze",
            "evaluate",
            "create",
            "apply",
            "demonstrate",
            "implement",
            "solve",
            "calculate",
            "measure",
            "classify",
            "arrange",
            "construct",
            "design",
            "formulate",
            "judge",
            "critique",
            "assess",
            "recommend",
            "justify",
            "recognize",
            "recall",
            "interpret",
            "predict",
            "differentiate",
            "organize",
            "synthesize",
            "compose",
            "produce",
            "perform",
        ];

        let behavior_lower = behavior.to_lowercase();
        let words: Vec<&str> = behavior_lower.split_whitespace().collect();

        for word in words {
            if action_verbs.contains(&word) {
                return word.to_string();
            }
        }

        // Default if no action verb found
        "perform".to_string()
    }

    /// Determine cognitive level based on action verb and objectives
    fn determine_cognitive_level(
        action_verb: &str,
        learning_objectives: &[LearningObjective],
    ) -> crate::character_sheet::BloomLevel {
        // Map action verbs to Bloom's levels
        let verb_to_level = std::collections::HashMap::from([
            // Remember
            ("identify", crate::character_sheet::BloomLevel::Remember),
            ("list", crate::character_sheet::BloomLevel::Remember),
            ("recall", crate::character_sheet::BloomLevel::Remember),
            ("recognize", crate::character_sheet::BloomLevel::Remember),
            ("define", crate::character_sheet::BloomLevel::Remember),
            // Understand
            ("describe", crate::character_sheet::BloomLevel::Understand),
            ("explain", crate::character_sheet::BloomLevel::Understand),
            ("summarize", crate::character_sheet::BloomLevel::Understand),
            ("interpret", crate::character_sheet::BloomLevel::Understand),
            ("predict", crate::character_sheet::BloomLevel::Understand),
            // Apply
            ("apply", crate::character_sheet::BloomLevel::Apply),
            ("implement", crate::character_sheet::BloomLevel::Apply),
            ("perform", crate::character_sheet::BloomLevel::Apply),
            ("demonstrate", crate::character_sheet::BloomLevel::Apply),
            ("use", crate::character_sheet::BloomLevel::Apply),
            // Analyze
            ("analyze", crate::character_sheet::BloomLevel::Analyze),
            ("compare", crate::character_sheet::BloomLevel::Analyze),
            ("contrast", crate::character_sheet::BloomLevel::Analyze),
            ("differentiate", crate::character_sheet::BloomLevel::Analyze),
            ("organize", crate::character_sheet::BloomLevel::Analyze),
            // Evaluate
            ("evaluate", crate::character_sheet::BloomLevel::Evaluate),
            ("judge", crate::character_sheet::BloomLevel::Evaluate),
            ("critique", crate::character_sheet::BloomLevel::Evaluate),
            ("assess", crate::character_sheet::BloomLevel::Evaluate),
            ("recommend", crate::character_sheet::BloomLevel::Evaluate),
            // Create
            ("create", crate::character_sheet::BloomLevel::Create),
            ("design", crate::character_sheet::BloomLevel::Create),
            ("construct", crate::character_sheet::BloomLevel::Create),
            ("compose", crate::character_sheet::BloomLevel::Create),
            ("produce", crate::character_sheet::BloomLevel::Create),
            ("synthesize", crate::character_sheet::BloomLevel::Create),
        ]);

        // Default to verb mapping
        if let Some(level) = verb_to_level.get(action_verb) {
            return *level;
        }

        // Fall back to highest level from learning objectives
        learning_objectives
            .iter()
            .map(|obj| &obj.verb)
            .filter_map(|verb| verb_to_level.get(verb.as_str()))
            .max_by_key(|level| match level {
                crate::character_sheet::BloomLevel::Remember => 1,
                crate::character_sheet::BloomLevel::Understand => 2,
                crate::character_sheet::BloomLevel::Apply => 3,
                crate::character_sheet::BloomLevel::Analyze => 4,
                crate::character_sheet::BloomLevel::Evaluate => 5,
                crate::character_sheet::BloomLevel::Create => 6,
            })
            .cloned()
            .unwrap_or(crate::character_sheet::BloomLevel::Understand)
    }

    /// Assess complexity of behavior
    fn assess_complexity(
        behavior: &str,
        cognitive_level: &crate::character_sheet::BloomLevel,
    ) -> ComplexityLevel {
        let word_count = behavior.split_whitespace().count();
        let has_conjunctions = behavior.to_lowercase().contains(" and ")
            || behavior.to_lowercase().contains(" or ")
            || behavior.to_lowercase().contains(" but ");

        match cognitive_level {
            crate::character_sheet::BloomLevel::Remember
            | crate::character_sheet::BloomLevel::Understand => {
                if word_count <= 8 && !has_conjunctions {
                    ComplexityLevel::Simple
                } else {
                    ComplexityLevel::Moderate
                }
            }
            crate::character_sheet::BloomLevel::Apply => {
                if word_count <= 10 && !has_conjunctions {
                    ComplexityLevel::Moderate
                } else {
                    ComplexityLevel::Complex
                }
            }
            crate::character_sheet::BloomLevel::Analyze => ComplexityLevel::Complex,
            crate::character_sheet::BloomLevel::Evaluate
            | crate::character_sheet::BloomLevel::Create => {
                if word_count > 12 || has_conjunctions {
                    ComplexityLevel::Expert
                } else {
                    ComplexityLevel::Complex
                }
            }
        }
    }

    /// Recommend practice types based on action verb and cognitive level
    fn recommend_practice_types(
        _action_verb: &str,
        cognitive_level: &crate::character_sheet::BloomLevel,
    ) -> Vec<YardmasterType> {
        match cognitive_level {
            crate::character_sheet::BloomLevel::Remember => {
                vec![YardmasterType::Practice, YardmasterType::Reference]
            }
            crate::character_sheet::BloomLevel::Understand => {
                vec![
                    YardmasterType::Practice,
                    YardmasterType::Demonstration,
                    YardmasterType::Assessment,
                ]
            }
            crate::character_sheet::BloomLevel::Apply => {
                vec![
                    YardmasterType::Practice,
                    YardmasterType::Simulation,
                    YardmasterType::Assessment,
                ]
            }
            crate::character_sheet::BloomLevel::Analyze => {
                vec![
                    YardmasterType::Practice,
                    YardmasterType::Collaborative,
                    YardmasterType::Assessment,
                ]
            }
            crate::character_sheet::BloomLevel::Evaluate => {
                vec![
                    YardmasterType::Collaborative,
                    YardmasterType::Assessment,
                    YardmasterType::Reference,
                ]
            }
            crate::character_sheet::BloomLevel::Create => {
                vec![
                    YardmasterType::Practice,
                    YardmasterType::Simulation,
                    YardmasterType::Collaborative,
                ]
            }
        }
    }

    /// Recommend assessment methods based on cognitive level
    fn recommend_assessment_methods(
        cognitive_level: &crate::character_sheet::BloomLevel,
    ) -> Vec<AssessmentType> {
        match cognitive_level {
            crate::character_sheet::BloomLevel::Remember => {
                vec![AssessmentType::Formative, AssessmentType::Competency]
            }
            crate::character_sheet::BloomLevel::Understand => {
                vec![AssessmentType::Formative, AssessmentType::Competency]
            }
            crate::character_sheet::BloomLevel::Apply => {
                vec![AssessmentType::Performance, AssessmentType::Formative]
            }
            crate::character_sheet::BloomLevel::Analyze => {
                vec![AssessmentType::Formative, AssessmentType::Summative]
            }
            crate::character_sheet::BloomLevel::Evaluate => {
                vec![AssessmentType::Summative, AssessmentType::Performance]
            }
            crate::character_sheet::BloomLevel::Create => {
                vec![AssessmentType::Performance, AssessmentType::Summative]
            }
        }
    }

    /// Generate content requirements from analyzed behavior
    fn generate_content_requirements(behavior: &AnalyzedBehavior) -> Vec<ContentRequirement> {
        let mut requirements = Vec::new();

        // Knowledge requirement
        requirements.push(ContentRequirement {
            id: Uuid::new_v4().to_string(),
            requirement_type: RequirementType::Knowledge,
            description: format!("Knowledge foundation for: {}", behavior.behavior),
            priority: Priority::Critical,
        });

        // Practice requirements based on recommended types
        for practice_type in &behavior.practice_types {
            requirements.push(ContentRequirement {
                id: Uuid::new_v4().to_string(),
                requirement_type: RequirementType::Practice,
                description: format!("{:?} practice for: {}", practice_type, behavior.action_verb),
                priority: match practice_type {
                    YardmasterType::Practice => Priority::Critical,
                    YardmasterType::Simulation => Priority::High,
                    YardmasterType::Demonstration => Priority::Medium,
                    _ => Priority::Low,
                },
            });
        }

        // Assessment requirements
        for assessment_type in &behavior.assessment_methods {
            requirements.push(ContentRequirement {
                id: Uuid::new_v4().to_string(),
                requirement_type: RequirementType::Assessment,
                description: format!(
                    "{:?} assessment for: {}",
                    assessment_type, behavior.action_verb
                ),
                priority: match assessment_type {
                    AssessmentType::Performance => Priority::Critical,
                    AssessmentType::Summative => Priority::High,
                    AssessmentType::Formative => Priority::Medium,
                    _ => Priority::Low,
                },
            });
        }

        requirements
    }

    /// Generate recommended Yardmaster from behavior analysis
    fn generate_yardmaster(
        behavior: &AnalyzedBehavior,
        _milestones: &[QuestMilestone],
    ) -> Vec<Yardmaster> {
        let mut yardmaster_items = Vec::new();

        // Generate Yardmaster for each recommended practice type
        for practice_type in behavior.practice_types.iter() {
            let template = Self::select_template(practice_type, &behavior.cognitive_level);
            let cognitive_load =
                Self::calculate_cognitive_load(&behavior.complexity, practice_type);

            yardmaster_items.push(Yardmaster {
                id: Uuid::new_v4(),
                title: format!("{:?}: {}", practice_type, behavior.action_verb),
                content_type: practice_type.clone(),
                target_behavior: behavior.behavior.clone(),
                bloom_level: behavior.cognitive_level,
                template,
                resources: Self::generate_resources(practice_type, &behavior.complexity),
                assessment_criteria: Self::generate_assessment_criteria(
                    &behavior.assessment_methods,
                ),
                cognitive_load,
                prerequisites: Vec::new(), // Will be calculated later
            });
        }

        yardmaster_items
    }

    /// Select appropriate template for practice type and cognitive level
    fn select_template(
        practice_type: &YardmasterType,
        cognitive_level: &crate::character_sheet::BloomLevel,
    ) -> ContentTemplate {
        // Template selection logic - simplified for now
        ContentTemplate {
            name: format!("{:?}_{:?}_template", practice_type, cognitive_level),
            category: format!("{:?}", practice_type),
            structure: TemplateStructure {
                sections: vec![
                    TemplateSection {
                        id: "introduction".to_string(),
                        section_type: SectionType::Introduction,
                        content_template: "Introduction to {topic}".to_string(),
                        order: 1,
                    },
                    TemplateSection {
                        id: "practice".to_string(),
                        section_type: SectionType::Practice,
                        content_template: "Practice activity for {topic}".to_string(),
                        order: 2,
                    },
                    TemplateSection {
                        id: "assessment".to_string(),
                        section_type: SectionType::Assessment,
                        content_template: "Assessment for {topic}".to_string(),
                        order: 3,
                    },
                ],
                interactions: vec![],
                assessment_points: vec![],
            },
            parameters: vec![TemplateParameter {
                name: "topic".to_string(),
                param_type: ParameterType::Text,
                required: true,
                default_value: None,
                description: "Main topic for the content".to_string(),
            }],
            instructions: format!(
                "Generate {:?} content at {:?} level",
                practice_type, cognitive_level
            ),
        }
    }

    /// Calculate cognitive load based on complexity and practice type
    fn calculate_cognitive_load(
        complexity: &ComplexityLevel,
        practice_type: &YardmasterType,
    ) -> f32 {
        let base_load = match complexity {
            ComplexityLevel::Simple => 10.0,
            ComplexityLevel::Moderate => 25.0,
            ComplexityLevel::Complex => 50.0,
            ComplexityLevel::Expert => 80.0,
        };

        let type_modifier = match practice_type {
            YardmasterType::Practice => 1.0,
            YardmasterType::Simulation => 1.5,
            YardmasterType::Collaborative => 1.3,
            YardmasterType::Assessment => 1.2,
            YardmasterType::Demonstration => 0.8,
            YardmasterType::Reference => 0.5,
        };

        base_load * type_modifier
    }

    /// Generate resource requirements for Yardmaster
    fn generate_resources(
        practice_type: &YardmasterType,
        _complexity: &ComplexityLevel,
    ) -> Vec<ContentResource> {
        match practice_type {
            YardmasterType::Simulation => vec![ContentResource {
                id: "simulation_engine".to_string(),
                resource_type: ResourceType::Interactive,
                description: "Interactive simulation environment".to_string(),
                required: true,
            }],
            YardmasterType::Practice => vec![ContentResource {
                id: "practice_materials".to_string(),
                resource_type: ResourceType::Document,
                description: "Practice exercises and worksheets".to_string(),
                required: true,
            }],
            YardmasterType::Demonstration => vec![ContentResource {
                id: "demo_video".to_string(),
                resource_type: ResourceType::Video,
                description: "Demonstration video".to_string(),
                required: true,
            }],
            _ => Vec::new(),
        }
    }

    /// Generate assessment criteria for Yardmaster
    fn generate_assessment_criteria(
        assessment_methods: &[AssessmentType],
    ) -> Vec<AssessmentCriterion> {
        assessment_methods
            .iter()
            .enumerate()
            .map(|(i, method)| AssessmentCriterion {
                id: format!("criterion_{}", i),
                description: format!("Assessment using {:?}", method),
                measurement_method: match method {
                    AssessmentType::Formative => MeasurementMethod::Automated,
                    AssessmentType::Summative => MeasurementMethod::Instructor,
                    AssessmentType::Competency => MeasurementMethod::Observation,
                    AssessmentType::Performance => MeasurementMethod::Observation,
                },
                success_criteria: "Meets performance standards".to_string(),
            })
            .collect()
    }
}
