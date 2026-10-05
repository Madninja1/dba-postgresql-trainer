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
    pub is_correct: bool,
}
