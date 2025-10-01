// Confidence calculation
use crate::models::{ConfidenceBreakdown, SpecificationAnalysis, ArchitecturePlan, Requirement};

/// Calculate overall confidence score using weighted formula
pub fn calculate_confidence(
    analysis: &SpecificationAnalysis,
    plan: &ArchitecturePlan,
) -> ConfidenceBreakdown {
    let spec_clarity = calculate_spec_clarity(analysis);
    let technical_feasibility = calculate_technical_feasibility(plan);
    let architecture_soundness = calculate_architecture_soundness(plan);
    let completeness = calculate_completeness(analysis, plan);
    let risk_assessment = calculate_risk_assessment(analysis);

    ConfidenceBreakdown::calculate(
        spec_clarity,
        technical_feasibility,
        architecture_soundness,
        completeness,
        risk_assessment,
    )
}

/// Calculate spec clarity score (0.0-1.0)
pub fn calculate_spec_clarity(analysis: &SpecificationAnalysis) -> f32 {
    let mut score = 0.5; // Base score

    // Bonus for having explicit requirements
    if !analysis.explicit_requirements.is_empty() {
        score += 0.2;
        score += (analysis.explicit_requirements.len() as f32 * 0.02).min(0.2);
    }

    // Penalty for ambiguities
    let ambiguity_penalty = (analysis.ambiguities.len() as f32 * 0.1).min(0.3);
    score -= ambiguity_penalty;

    // Penalty for missing critical info
    let missing_penalty = (analysis.missing_critical_info.len() as f32 * 0.05).min(0.2);
    score -= missing_penalty;

    // Bonus for technical keywords (shows specificity)
    if analysis.technical_keywords.len() >= 3 {
        score += 0.1;
    }

    score.max(0.0).min(1.0)
}

/// Calculate technical feasibility score (0.0-1.0)
pub fn calculate_technical_feasibility(plan: &ArchitecturePlan) -> f32 {
    let mut score: f32 = 0.5;

    // Check if tech stack is defined
    if !plan.tech_stack.framework.is_empty() && plan.tech_stack.framework != "Unknown" {
        score += 0.2;
    }
    if !plan.tech_stack.language.is_empty() && plan.tech_stack.language != "Unknown" {
        score += 0.2;
    }

    // Check if components are defined
    if !plan.components.is_empty() {
        score += 0.1;
    }

    // Check if decisions are made
    if !plan.decisions.is_empty() {
        score += 0.1;
    }

    score.max(0.0).min(1.0)
}

/// Calculate architecture soundness score (0.0-1.0)
pub fn calculate_architecture_soundness(plan: &ArchitecturePlan) -> f32 {
    let mut score = 0.5;

    // Bonus for having multiple architecture decisions
    let decision_bonus = (plan.decisions.len() as f32 * 0.05).min(0.3);
    score += decision_bonus;

    // Bonus for having components
    if plan.components.len() >= 5 {
        score += 0.2;
    } else if plan.components.len() >= 3 {
        score += 0.1;
    }

    // Average confidence of decisions
    if !plan.decisions.is_empty() {
        let avg_decision_confidence: f32 = plan.decisions.iter()
            .map(|d| d.confidence)
            .sum::<f32>() / plan.decisions.len() as f32;
        score = (score + avg_decision_confidence) / 2.0;
    }

    score.max(0.0).min(1.0)
}

/// Calculate completeness score (0.0-1.0)
pub fn calculate_completeness(
    analysis: &SpecificationAnalysis,
    plan: &ArchitecturePlan,
) -> f32 {
    let mut score = 0.0;

    // Check if all requirement categories are addressed
    let req_categories = count_requirement_categories(&analysis.explicit_requirements);
    let category_coverage = (req_categories as f32 * 0.1).min(0.4);
    score += category_coverage;

    // Check plan completeness
    if !plan.project_name.is_empty() {
        score += 0.1;
    }
    if !plan.components.is_empty() {
        score += 0.2;
    }
    if !plan.decisions.is_empty() {
        score += 0.2;
    }
    if plan.tech_stack.runtime.is_some() {
        score += 0.05;
    }
    if plan.tech_stack.bundler.is_some() {
        score += 0.05;
    }

    score.max(0.0).min(1.0)
}

/// Calculate risk assessment score (0.0-1.0)
pub fn calculate_risk_assessment(analysis: &SpecificationAnalysis) -> f32 {
    let mut score = 1.0; // Start high, reduce for risks

    // Penalty for high-impact ambiguities
    let high_impact_ambiguities = analysis.ambiguities.iter()
        .filter(|a| matches!(a.impact, crate::models::Impact::High))
        .count();
    score -= (high_impact_ambiguities as f32 * 0.15).min(0.5);

    // Penalty for missing critical info
    score -= (analysis.missing_critical_info.len() as f32 * 0.1).min(0.3);

    // Penalty for unknown project intent
    if matches!(analysis.intent, crate::models::ProjectIntent::Unknown) {
        score -= 0.2;
    }

    score.max(0.0).min(1.0)
}

// Helper functions

fn count_requirement_categories(requirements: &[Requirement]) -> usize {
    use std::collections::HashSet;
    let mut categories = HashSet::new();
    for req in requirements {
        categories.insert(format!("{:?}", req.category));
    }
    categories.len()
}
