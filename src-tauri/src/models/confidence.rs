use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceBreakdown {
    pub overall: f32,
    pub spec_clarity: f32,
    pub technical_feasibility: f32,
    pub architecture_soundness: f32,
    pub completeness: f32,
    pub risk_assessment: f32,
}

impl Default for ConfidenceBreakdown {
    fn default() -> Self {
        Self {
            overall: 0.0,
            spec_clarity: 0.0,
            technical_feasibility: 0.0,
            architecture_soundness: 0.0,
            completeness: 0.0,
            risk_assessment: 0.0,
        }
    }
}

impl ConfidenceBreakdown {
    pub fn calculate(
        spec_clarity: f32,
        feasibility: f32,
        soundness: f32,
        completeness: f32,
        risk: f32,
    ) -> Self {
        let overall = (spec_clarity * 0.25
            + feasibility * 0.25
            + soundness * 0.20
            + completeness * 0.20
            + risk * 0.10)
            * 100.0;

        Self {
            overall: overall.clamp(0.0, 100.0),
            spec_clarity: spec_clarity * 100.0,
            technical_feasibility: feasibility * 100.0,
            architecture_soundness: soundness * 100.0,
            completeness: completeness * 100.0,
            risk_assessment: risk * 100.0,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let fields = [
            ("overall", self.overall),
            ("spec_clarity", self.spec_clarity),
            ("technical_feasibility", self.technical_feasibility),
            ("architecture_soundness", self.architecture_soundness),
            ("completeness", self.completeness),
            ("risk_assessment", self.risk_assessment),
        ];

        for (name, value) in &fields {
            if *value < 0.0 || *value > 100.0 {
                return Err(format!("{} must be between 0 and 100", name));
            }
        }

        Ok(())
    }
}
