use dba_trainer_domain::{
    AnswerOptionId, AnswerResult, Lab, Question, QuestionId, QuizSession, SessionConfig, SessionId,
    SessionProgress, StatisticsFilter, Topic, TopicId, TopicMasteryStats, TrainingStats,
};

use crate::RepositoryError;

pub trait TopicRepository {
    fn topics(&self) -> Result<Vec<Topic>, RepositoryError>;

    fn topic_mastery(&self, topic_id: TopicId) -> Result<TopicMasteryStats, RepositoryError>;
}

pub trait LabRepository {
    fn labs(&self) -> Result<Vec<Lab>, RepositoryError>;
}

pub trait ContentLocaleRepository {
    fn set_content_locale(&mut self, locale: &str) -> Result<(), RepositoryError>;
}

pub trait SessionRepository {
    fn start_session(&mut self, config: &SessionConfig) -> Result<QuizSession, RepositoryError>;

    fn active_session(&self) -> Result<Option<SessionProgress>, RepositoryError>;

    fn cancel_session(&mut self, session_id: SessionId) -> Result<(), RepositoryError>;

    fn statistics(&self, filter: &StatisticsFilter) -> Result<TrainingStats, RepositoryError>;

    fn current_question(&self, session_id: SessionId) -> Result<Option<Question>, RepositoryError>;

    fn submit_answer(
        &mut self,
        session_id: SessionId,
        question_id: QuestionId,
        answer_option_ids: &[AnswerOptionId],
    ) -> Result<AnswerResult, RepositoryError>;

    fn clear_statistics(&mut self) -> Result<(), RepositoryError>;
}
