use serde::{Deserialize, Serialize};
use super::ambiguity::Impact;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    YesNo,
    FreeText,
    ConfirmationWithDefault,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub text: String,
    pub question_type: QuestionType,
    pub options: Option<Vec<QuestionOption>>,
    pub recommended_answer: Option<String>,
    pub reasoning: Option<String>,
    pub impact: Impact,
}

impl Question {
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Question ID cannot be empty".to_string());
        }
        if self.text.is_empty() {
            return Err("Question text cannot be empty".to_string());
        }
        
        match self.question_type {
            QuestionType::SingleChoice | QuestionType::MultipleChoice => {
                if self.options.is_none() {
                    return Err("Choice questions must have options".to_string());
                }
            }
            QuestionType::YesNo => {
                if self.options.is_some() {
                    return Err("YesNo questions should not have options".to_string());
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}
