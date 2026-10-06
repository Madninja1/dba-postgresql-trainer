use crate::{AnswerOptionId, QuestionId, SessionId, TopicId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuizScope {
    Topic(TopicId),
    AllTopics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestionLimit {
    Twenty,
    Fifty,
    All,
}

impl QuestionLimit {
    pub fn as_limit(self) -> Option<usize> {
        match self {
            Self::Twenty => Some(20),
            Self::Fifty => Some(50),
            Self::All => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionConfig {
    pub scope: QuizScope,
    pub limit: QuestionLimit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuizSession {
    pub id: SessionId,
    pub question_ids: Vec<QuestionId>,
    pub current_index: usize,
}

impl QuizSession {
    pub fn total_questions(&self) -> usize {
        self.question_ids.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswerResult {
    pub question_id: QuestionId,
    pub selected_option_ids: Vec<AnswerOptionId>,
    pub correct_option_ids: Vec<AnswerOptionId>,
    pub is_correct: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionProgress {
    pub id: SessionId,

    pub scope: QuizScope,

    pub current_index: usize,

    pub total_questions: usize,

    pub answered_questions: usize,

    pub correct_answers: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TrainingStats {
    pub completed_sessions: usize,

    pub cancelled_sessions: usize,

    pub answered_questions: usize,

    pub correct_answers: usize,
}

impl TrainingStats {
    pub fn incorrect_answers(&self) -> usize {
        self.answered_questions.saturating_sub(self.correct_answers)
    }

    pub fn accuracy_percent(&self) -> f64 {
        if self.answered_questions == 0 {
            return 0.0;
        }

        (self.correct_answers as f64 / self.answered_questions as f64) * 100.0
    }
}
