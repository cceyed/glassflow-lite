// T028: ConfidenceBreakdown struct for Engineer Agent
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceBreakdown {
    pub overall: f32,           // 0.0-100.0
    pub quality_score: f32,     // 0.0-100.0
    pub plan_adherence: f32,    // 0.0-100.0
    pub issue_penalty: f32,     // 0.0-100.0
}

impl ConfidenceBreakdown {
    pub fn new(quality_score: f32, plan_adherence: f32, issue_penalty: f32) -> Self {
        let overall = Self::calculate_overall(quality_score, plan_adherence, issue_penalty);
        
        Self {
            overall,
            quality_score,
            plan_adherence,
            issue_penalty,
        }
    }

    fn calculate_overall(quality_score: f32, plan_adherence: f32, issue_penalty: f32) -> f32 {
        // Formula: overall = (quality * 0.50) + (adherence * 0.30) + ((100 - penalty) * 0.20)
        let overall = (quality_score * 0.50)
            + (plan_adherence * 0.30)
            + ((100.0 - issue_penalty) * 0.20);
        
        overall.clamp(0.0, 100.0)
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=100.0).contains(&self.overall) {
            return Err("Overall confidence must be between 0 and 100".to_string());
        }
        
        if !(0.0..=100.0).contains(&self.quality_score) {
            return Err("Quality score must be between 0 and 100".to_string());
        }
        
        if !(0.0..=100.0).contains(&self.plan_adherence) {
            return Err("Plan adherence must be between 0 and 100".to_string());
        }
        
        if !(0.0..=100.0).contains(&self.issue_penalty) {
            return Err("Issue penalty must be between 0 and 100".to_string());
        }
        
        // Verify calculation
        let expected_overall = Self::calculate_overall(
            self.quality_score,
            self.plan_adherence,
            self.issue_penalty,
        );
        
        if (self.overall - expected_overall).abs() > 0.1 {
            return Err(format!(
                "Overall confidence mismatch: expected {}, got {}",
                expected_overall, self.overall
            ));
        }
        
        Ok(())
    }

    pub fn is_high_confidence(&self) -> bool {
        self.overall >= 85.0
    }

    pub fn is_medium_confidence(&self) -> bool {
        self.overall >= 70.0 && self.overall < 85.0
    }

    pub fn is_low_confidence(&self) -> bool {
        self.overall < 70.0
    }
}

impl Default for ConfidenceBreakdown {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_score() {
        let confidence = ConfidenceBreakdown::new(100.0, 100.0, 0.0);
        assert_eq!(confidence.overall, 100.0);
        assert!(confidence.is_high_confidence());
    }

    #[test]
    fn test_medium_score() {
        let confidence = ConfidenceBreakdown::new(80.0, 75.0, 15.0);
        assert!(confidence.overall >= 70.0 && confidence.overall < 85.0);
        assert!(confidence.is_medium_confidence());
    }

    #[test]
    fn test_low_score() {
        let confidence = ConfidenceBreakdown::new(50.0, 60.0, 40.0);
        assert!(confidence.overall < 70.0);
        assert!(confidence.is_low_confidence());
    }

    #[test]
    fn test_validation() {
        let confidence = ConfidenceBreakdown::new(90.0, 85.0, 10.0);
        assert!(confidence.validate().is_ok());
    }
}
