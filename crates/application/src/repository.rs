use dba_trainer_domain::{
    AnswerOptionId, AnswerResult, Question, QuestionId, QuizSession, SessionConfig, SessionId,
    Topic,
};

use crate::RepositoryError;

pub trait TopicRepository {
    fn topics(&self) -> Result<Vec<Topic>, RepositoryError>;
}

pub trait SessionRepository {
    fn start_session(&self, config: &SessionConfig) -> Result<QuizSession, RepositoryError>;

    fn current_question(&self, session_id: SessionId) -> Result<Option<Question>, RepositoryError>;

    fn submit_answer(
        &self,
        session_id: SessionId,
        question_id: QuestionId,
        answer_option_id: AnswerOptionId,
    ) -> Result<AnswerResult, RepositoryError>;
}
