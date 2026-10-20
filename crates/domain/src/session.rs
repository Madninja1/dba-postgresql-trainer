use crate::{AnswerOptionId, QuestionId, SessionId, TopicId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuizScope {
    Topic(TopicId),
    Course(String),
    AllTopics,
    Mistakes,
    MistakesCourse(String),
    MistakesTopic(TopicId),
}

impl QuizScope {
    pub fn is_review(&self) -> bool {
        matches!(
            self,
            Self::Mistakes | Self::MistakesCourse(_) | Self::MistakesTopic(_)
        )
    }
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

    pub const fn pass_threshold_percent(self) -> f64 {
        match self {
            Self::Twenty => 85.0,
            Self::Fifty => 70.0,
            Self::All => 60.0,
        }
    }

    pub fn required_correct_answers(self, total_questions: usize) -> usize {
        if total_questions == 0 {
            return 0;
        }

        ((total_questions as f64 * self.pass_threshold_percent()) / 100.0).ceil() as usize
    }

    pub fn is_passed(self, correct_answers: usize, answered_questions: usize) -> bool {
        answered_questions > 0
            && correct_answers >= self.required_correct_answers(answered_questions)
    }
}

pub const fn pass_threshold_percent_for_requested_count(requested_count: Option<usize>) -> f64 {
    match requested_count {
        Some(20) => QuestionLimit::Twenty.pass_threshold_percent(),
        Some(50) => QuestionLimit::Fifty.pass_threshold_percent(),
        None => QuestionLimit::All.pass_threshold_percent(),
        Some(_) => QuestionLimit::Twenty.pass_threshold_percent(),
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

    pub limit: QuestionLimit,

    pub current_index: usize,

    pub total_questions: usize,

    pub answered_questions: usize,

    pub correct_answers: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatisticsLimit {
    Any,
    Twenty,
    Fifty,
    AllQuestions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatisticsScope {
    All,
    Course(String),
    Topic(TopicId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatisticsFilter {
    pub scope: StatisticsScope,
    pub limit: StatisticsLimit,
}

impl StatisticsFilter {
    pub fn all() -> Self {
        Self {
            scope: StatisticsScope::All,
            limit: StatisticsLimit::Any,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_thresholds_depend_on_question_limit() {
        assert_eq!(QuestionLimit::Twenty.pass_threshold_percent(), 85.0);
        assert_eq!(QuestionLimit::Fifty.pass_threshold_percent(), 70.0);
        assert_eq!(QuestionLimit::All.pass_threshold_percent(), 60.0);
    }

    #[test]
    fn required_correct_answers_rounds_up() {
        assert_eq!(QuestionLimit::Twenty.required_correct_answers(20), 17);
        assert_eq!(QuestionLimit::Fifty.required_correct_answers(50), 35);
        assert_eq!(QuestionLimit::All.required_correct_answers(73), 44);
    }

    #[test]
    fn pass_check_uses_mode_threshold() {
        assert!(QuestionLimit::Twenty.is_passed(17, 20));
        assert!(!QuestionLimit::Twenty.is_passed(16, 20));
        assert!(QuestionLimit::Fifty.is_passed(35, 50));
        assert!(!QuestionLimit::Fifty.is_passed(34, 50));
        assert!(QuestionLimit::All.is_passed(44, 73));
        assert!(!QuestionLimit::All.is_passed(43, 73));
    }

    #[test]
    fn review_scope_recognizes_all_mistake_variants() {
        assert!(QuizScope::Mistakes.is_review());
        assert!(QuizScope::MistakesCourse(String::from("dba-1")).is_review());
        assert!(QuizScope::MistakesTopic(TopicId(1)).is_review());
        assert!(!QuizScope::AllTopics.is_review());
    }
}
