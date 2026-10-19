use crate::TopicId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topic {
    pub id: TopicId,

    pub course_code: String,

    pub slug: String,
    pub notes_part: i64,
    pub topic_number: i64,
    pub title: String,
    pub description: Option<String>,
}

/// A topic is considered passed when a completed topic quiz reaches this accuracy.
pub const TOPIC_PASS_THRESHOLD_PERCENT: f64 = 85.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicMasteryStats {
    pub topic_id: TopicId,
    pub completed_attempts: usize,
    pub best_correct_answers: usize,
    pub best_answered_questions: usize,
    pub attempts_to_pass: Option<usize>,
}

impl TopicMasteryStats {
    pub fn best_accuracy_percent(&self) -> f64 {
        if self.best_answered_questions == 0 {
            return 0.0;
        }

        (self.best_correct_answers as f64 / self.best_answered_questions as f64) * 100.0
    }

    pub fn passed(&self) -> bool {
        self.attempts_to_pass.is_some()
    }
}
