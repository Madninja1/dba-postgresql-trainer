use dba_trainer_domain::{
    AnswerOptionId, AnswerResult, Question, QuestionId, QuizSession, SessionConfig, SessionId,
    Topic,
};

use crate::{RepositoryError, SessionRepository, TopicRepository};

pub struct TrainerService<R> {
    repository: R,
}

impl<R> TrainerService<R>
where
    R: TopicRepository + SessionRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn topics(&self) -> Result<Vec<Topic>, RepositoryError> {
        self.repository.topics()
    }

    pub fn start_session(
        &mut self,
        config: &SessionConfig,
    ) -> Result<QuizSession, RepositoryError> {
        self.repository.start_session(config)
    }

    pub fn current_question(
        &self,
        session_id: SessionId,
    ) -> Result<Option<Question>, RepositoryError> {
        self.repository.current_question(session_id)
    }

    pub fn submit_answer(
        &mut self,
        session_id: SessionId,
        question_id: QuestionId,
        answer_option_ids: &[AnswerOptionId],
    ) -> Result<AnswerResult, RepositoryError> {
        self.repository
            .submit_answer(session_id, question_id, answer_option_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use dba_trainer_domain::{AnswerOptionId, QuestionLimit, QuizScope, SessionId, TopicId};

    struct FakeRepository;

    impl TopicRepository for FakeRepository {
        fn topics(&self) -> Result<Vec<Topic>, RepositoryError> {
            Ok(vec![Topic {
                id: TopicId(1),
                course_code: String::from("dba-1"),
                slug: String::from("architecture"),
                title: String::from("Architecture"),
                description: None,
            }])
        }
    }

    impl SessionRepository for FakeRepository {
        fn start_session(
            &mut self,
            _config: &SessionConfig,
        ) -> Result<QuizSession, RepositoryError> {
            Ok(QuizSession {
                id: SessionId(1),
                question_ids: vec![QuestionId(10), QuestionId(20)],
                current_index: 0,
            })
        }

        fn current_question(
            &self,
            _session_id: SessionId,
        ) -> Result<Option<Question>, RepositoryError> {
            Ok(None)
        }

        fn submit_answer(
            &mut self,
            _session_id: SessionId,
            question_id: QuestionId,
            answer_option_ids: &[AnswerOptionId],
        ) -> Result<AnswerResult, RepositoryError> {
            Ok(AnswerResult {
                question_id,
                selected_option_ids: answer_option_ids.to_vec(),
                is_correct: true,
            })
        }
    }

    #[test]
    fn service_returns_topics() {
        let service = TrainerService::new(FakeRepository);

        let topics = service
            .topics()
            .expect("fake repository should return topics");

        assert_eq!(topics.len(), 1);
        assert_eq!(topics[0].id, TopicId(1));
    }

    #[test]
    fn service_starts_session() {
        let mut service = TrainerService::new(FakeRepository);

        let config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),
            limit: QuestionLimit::Twenty,
        };

        let session = service
            .start_session(&config)
            .expect("fake repository should start session");

        assert_eq!(session.id, SessionId(1));
        assert_eq!(session.total_questions(), 2);
    }
}
