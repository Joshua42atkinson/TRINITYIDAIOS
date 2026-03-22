// Trinity AI Agent System - Quality Matters Rubric Evaluator
// Copyright (c) Joshua
//
// ═══════════════════════════════════════════════════════════════════════════════
// 📊 ZONE: PROTOCOL | Module: QM Rubric
// ═══════════════════════════════════════════════════════════════════════════════
// Quality Matters Rubric evaluation for instructional design contracts.
// Provides objective scoring for learning effectiveness and design quality.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::id_contract::{ActionMap, IdContract, LearningObjective};

/// Quality Matters Rubric evaluation results for an ID Contract.
/// This provides objective validation of instructional design quality.
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmEvaluation {
    /// Unique ID for this evaluation
    pub id: Uuid,
    /// Contract being evaluated
    pub contract_id: Uuid,
    /// Overall QM score (0-100)
    pub overall_score: f32,
    /// Individual criterion scores
    pub criteria: Vec<QmCriterion>,
    /// Specific feedback for improvement
    pub feedback: Vec<String>,
    /// Whether the contract meets minimum quality standards
    pub meets_standards: bool,
    /// Evaluation timestamp
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
}

/// Individual QM Rubric criterion evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmCriterion {
    /// Criterion name/number
    pub name: String,
    /// Score for this criterion (0-3 scale, converted to 0-100)
    pub score: f32,
    /// Specific feedback for this criterion
    pub feedback: String,
    /// Whether this criterion is met
    pub met: bool,
}

/// QM Rubric evaluator implementation
pub struct QmRubricEvaluator;

impl QmRubricEvaluator {
    /// Evaluate an ID Contract against the QM Rubric
    pub fn evaluate(contract: &IdContract) -> QmEvaluation {
        let mut criteria = Vec::new();
        let mut feedback = Vec::new();

        // Criterion 1: Learning Objectives (QM 2.1-2.4)
        let objectives_score = Self::evaluate_learning_objectives(&contract.learning_objectives);
        criteria.push(QmCriterion {
            name: "Learning Objectives".to_string(),
            score: objectives_score,
            feedback: Self::objectives_feedback(&contract.learning_objectives),
            met: objectives_score >= 70.0,
        });

        // Criterion 2: Action Mapping Quality (QM 3.1-3.3)
        let action_score = Self::evaluate_action_mapping(&contract.action_map);
        criteria.push(QmCriterion {
            name: "Action Mapping".to_string(),
            score: action_score,
            feedback: Self::action_feedback(&contract.action_map),
            met: action_score >= 70.0,
        });

        // Criterion 3: Assessment Alignment (QM 4.1-4.3)
        let assessment_score = Self::evaluate_assessment_alignment(&contract.milestones);
        criteria.push(QmCriterion {
            name: "Assessment Alignment".to_string(),
            score: assessment_score,
            feedback: Self::assessment_feedback(&contract.milestones),
            met: assessment_score >= 70.0,
        });

        // Criterion 4: Cognitive Load (QM 5.1-5.3)
        let cognitive_score = Self::evaluate_cognitive_load(contract);
        criteria.push(QmCriterion {
            name: "Cognitive Load".to_string(),
            score: cognitive_score,
            feedback: Self::cognitive_feedback(contract),
            met: cognitive_score >= 70.0,
        });

        // Calculate overall score
        let overall_score = criteria.iter().map(|c| c.score).sum::<f32>() / criteria.len() as f32;
        let meets_standards = overall_score >= 70.0 && criteria.iter().all(|c| c.met);

        // Generate overall feedback
        if overall_score >= 90.0 {
            feedback.push(
                "🌟 Excellent instructional design! This contract exceeds QM standards."
                    .to_string(),
            );
        } else if overall_score >= 70.0 {
            feedback.push(
                "✅ Good instructional design that meets QM standards. Consider minor refinements."
                    .to_string(),
            );
        } else {
            feedback.push(
                "⚠️ This contract needs significant revision to meet QM standards.".to_string(),
            );
        }

        QmEvaluation {
            id: Uuid::new_v4(),
            contract_id: contract.id,
            overall_score,
            criteria,
            feedback,
            meets_standards,
            evaluated_at: chrono::Utc::now(),
        }
    }

    /// Evaluate learning objectives quality (QM 2.1-2.4)
    fn evaluate_learning_objectives(objectives: &[LearningObjective]) -> f32 {
        if objectives.is_empty() {
            return 0.0;
        }

        let mut total_score = 0.0;

        for obj in objectives {
            let mut obj_score: f32 = 33.33; // Base score

            // Check for measurable verbs
            if Self::has_measurable_verb(&obj.verb) {
                obj_score += 16.67;
            }

            // Check for conditions
            if !obj.condition.trim().is_empty() {
                obj_score += 16.67;
            }

            // Check for criteria
            if !obj.criterion.trim().is_empty() {
                obj_score += 16.67;
            }

            // Check for specific content
            if obj.content.len() > 10 {
                obj_score += 16.66;
            }

            total_score += obj_score.min(100.0);
        }

        (total_score / objectives.len() as f32).min(100.0)
    }

    /// Evaluate action mapping quality (QM 3.1-3.3)
    fn evaluate_action_mapping(action_map: &Option<ActionMap>) -> f32 {
        let Some(map) = action_map else {
            return 0.0;
        };

        let mut score: f32 = 33.33; // Base score

        // Measurable goal quality
        if map.measurable_goal.len() > 20 {
            score += 20.0;
        }
        if map.measurable_goal.to_lowercase().contains("%")
            || map.measurable_goal.to_lowercase().contains("reduce")
            || map.measurable_goal.to_lowercase().contains("increase")
        {
            score += 20.0;
        }

        // Observable behaviors quality
        if map.observable_behaviors.len() > 30 {
            score += 20.0;
        }
        if map.observable_behaviors.to_lowercase().contains("will")
            || map.observable_behaviors.to_lowercase().contains("can")
        {
            score += 6.67;
        }

        score.min(100.0)
    }

    /// Evaluate assessment alignment (QM 4.1-4.3)
    fn evaluate_assessment_alignment(milestones: &[crate::id_contract::QuestMilestone]) -> f32 {
        if milestones.is_empty() {
            return 0.0;
        }

        let mut total_score = 0.0;

        for milestone in milestones {
            let mut milestone_score: f32 = 50.0; // Base score

            // Check for clear deliverable
            if milestone.deliverable.len() > 15 {
                milestone_score += 25.0;
            }

            // Check for reasonable coal cost
            if milestone.coal_cost > 0.0 && milestone.coal_cost <= 100.0 {
                milestone_score += 25.0;
            }

            total_score += milestone_score.min(100.0);
        }

        (total_score / milestones.len() as f32).min(100.0)
    }

    /// Evaluate cognitive load management (QM 5.1-5.3)
    fn evaluate_cognitive_load(contract: &IdContract) -> f32 {
        let mut score: f32 = 50.0; // Base score

        // Check Bloom's level appropriateness
        match contract.bloom_level {
            crate::character_sheet::BloomLevel::Remember => score += 10.0,
            crate::character_sheet::BloomLevel::Understand => score += 20.0,
            crate::character_sheet::BloomLevel::Apply => score += 25.0,
            crate::character_sheet::BloomLevel::Analyze => score += 20.0,
            crate::character_sheet::BloomLevel::Evaluate => score += 15.0,
            crate::character_sheet::BloomLevel::Create => score += 10.0,
        }

        // Check total coal cost (cognitive load)
        if contract.estimated_coal_cost <= 200.0 {
            score += 20.0;
        } else if contract.estimated_coal_cost <= 500.0 {
            score += 10.0;
        }

        // Check number of milestones (cognitive chunking)
        if contract.milestones.len() >= 3 && contract.milestones.len() <= 7 {
            score += 20.0;
        }

        score.min(100.0)
    }

    /// Generate feedback for learning objectives
    fn objectives_feedback(objectives: &[LearningObjective]) -> String {
        if objectives.is_empty() {
            return "❌ No learning objectives defined. Add clear, measurable objectives."
                .to_string();
        }

        let mut issues = Vec::new();

        for (i, obj) in objectives.iter().enumerate() {
            if !Self::has_measurable_verb(&obj.verb) {
                issues.push(format!(
                    "Objective {}: '{}' needs a measurable verb",
                    i + 1,
                    obj.verb
                ));
            }
            if obj.condition.trim().is_empty() {
                issues.push(format!("Objective {}: missing conditions", i + 1));
            }
            if obj.criterion.trim().is_empty() {
                issues.push(format!("Objective {}: missing performance criteria", i + 1));
            }
        }

        if issues.is_empty() {
            "✅ Learning objectives are well-formed and measurable.".to_string()
        } else {
            format!("⚠️ Issues: {}", issues.join("; "))
        }
    }

    /// Generate feedback for action mapping
    fn action_feedback(action_map: &Option<ActionMap>) -> String {
        let Some(map) = action_map else {
            return "❌ No action mapping defined. Define measurable goals and observable behaviors.".to_string();
        };

        let mut issues = Vec::new();

        if map.measurable_goal.len() < 20 {
            issues.push("Measurable goal is too brief or vague".to_string());
        }
        if !map.measurable_goal.to_lowercase().contains("%")
            && !map.measurable_goal.to_lowercase().contains("reduce")
            && !map.measurable_goal.to_lowercase().contains("increase")
        {
            issues.push("Measurable goal lacks specific metrics".to_string());
        }
        if map.observable_behaviors.len() < 30 {
            issues.push("Observable behaviors need more detail".to_string());
        }

        if issues.is_empty() {
            "✅ Action mapping clearly defines goals and behaviors.".to_string()
        } else {
            format!("⚠️ Issues: {}", issues.join("; "))
        }
    }

    /// Generate feedback for assessment alignment
    fn assessment_feedback(milestones: &[crate::id_contract::QuestMilestone]) -> String {
        if milestones.is_empty() {
            return "❌ No milestones defined. Add clear assessment checkpoints.".to_string();
        }

        let mut issues = Vec::new();

        for (i, milestone) in milestones.iter().enumerate() {
            if milestone.deliverable.len() < 15 {
                issues.push(format!("Milestone {}: deliverable unclear", i + 1));
            }
            if milestone.coal_cost <= 0.0 || milestone.coal_cost > 100.0 {
                issues.push(format!("Milestone {}: unreasonable coal cost", i + 1));
            }
        }

        if issues.is_empty() {
            "✅ Milestones provide clear assessment checkpoints.".to_string()
        } else {
            format!("⚠️ Issues: {}", issues.join("; "))
        }
    }

    /// Generate feedback for cognitive load
    fn cognitive_feedback(contract: &IdContract) -> String {
        let mut issues = Vec::new();

        if contract.estimated_coal_cost > 500.0 {
            issues.push("Total cognitive load may be too high".to_string());
        }
        if contract.milestones.len() < 3 {
            issues.push("Consider breaking content into smaller chunks".to_string());
        }
        if contract.milestones.len() > 7 {
            issues.push("Too many milestones may overwhelm learners".to_string());
        }

        if issues.is_empty() {
            "✅ Cognitive load is well-managed and appropriate.".to_string()
        } else {
            format!("⚠️ Issues: {}", issues.join("; "))
        }
    }

    /// Check if a verb is measurable
    fn has_measurable_verb(verb: &str) -> bool {
        let measurable_verbs = [
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
        ];

        measurable_verbs.contains(&verb.to_lowercase().as_str())
    }
}

impl Default for QmEvaluation {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            contract_id: Uuid::new_v4(),
            overall_score: 0.0,
            criteria: Vec::new(),
            feedback: vec!["No evaluation performed".to_string()],
            meets_standards: false,
            evaluated_at: chrono::Utc::now(),
        }
    }
}
