use dba_trainer_domain::{
    ActivityStats, AnswerOptionId, AnswerResult, Lab, PracticeStats, PracticeTask,
    PracticeTaskProgress, Question, QuestionId, QuestionMistakeDetail, QuestionMistakeSummary,
    QuizSession, SessionConfig, SessionId, SessionProgress, StatisticsFilter, Topic, TopicId,
    TopicMasteryStats, TrainingStats,
};

use rand::{rng, seq::SliceRandom};

use crate::{
    ContentLocaleRepository, LabRepository, LearningHistoryRepository, PracticeRepository,
    RepositoryError, SessionRepository, TopicRepository,
};

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

    pub fn topic_mastery(&self, topic_id: TopicId) -> Result<TopicMasteryStats, RepositoryError> {
        self.repository.topic_mastery(topic_id)
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
        let mut question = self.repository.current_question(session_id)?;

        if let Some(question) = question.as_mut() {
            question.options.shuffle(&mut rng());
        }

        Ok(question)
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

    pub fn active_session(&self) -> Result<Option<SessionProgress>, RepositoryError> {
        self.repository.active_session()
    }

    pub fn cancel_session(&mut self, session_id: SessionId) -> Result<(), RepositoryError> {
        self.repository.cancel_session(session_id)
    }

    pub fn statistics(&self, filter: &StatisticsFilter) -> Result<TrainingStats, RepositoryError> {
        self.repository.statistics(filter)
    }

    pub fn clear_statistics(&mut self) -> Result<(), RepositoryError> {
        self.repository.clear_statistics()
    }
}

impl<R> TrainerService<R>
where
    R: LabRepository,
{
    pub fn labs(&self) -> Result<Vec<Lab>, RepositoryError> {
        self.repository.labs()
    }
}

impl<R> TrainerService<R>
where
    R: LearningHistoryRepository,
{
    pub fn mistake_summaries(&self) -> Result<Vec<QuestionMistakeSummary>, RepositoryError> {
        self.repository.mistake_summaries()
    }

    pub fn mistake_detail(
        &self,
        question_id: QuestionId,
    ) -> Result<QuestionMistakeDetail, RepositoryError> {
        self.repository.mistake_detail(question_id)
    }

    pub fn activity_stats(&self) -> Result<ActivityStats, RepositoryError> {
        self.repository.activity_stats()
    }

    pub fn record_app_open(&mut self) -> Result<(), RepositoryError> {
        self.repository.record_app_open()
    }
}

impl<R> TrainerService<R>
where
    R: PracticeRepository,
{
    pub fn practice_tasks(&self) -> Result<Vec<PracticeTask>, RepositoryError> {
        self.repository.practice_tasks()
    }

    pub fn practice_task(&self, key: &str) -> Result<PracticeTask, RepositoryError> {
        self.repository.practice_task(key)
    }

    pub fn practice_progress(&self) -> Result<Vec<PracticeTaskProgress>, RepositoryError> {
        self.repository.practice_progress()
    }

    pub fn save_practice_draft(&mut self, task_id: i64, sql: &str) -> Result<(), RepositoryError> {
        self.repository.save_practice_draft(task_id, sql)
    }

    pub fn record_practice_attempt(
        &mut self,
        task_id: i64,
        sql: &str,
        success: bool,
    ) -> Result<(), RepositoryError> {
        self.repository
            .record_practice_attempt(task_id, sql, success)
    }

    pub fn practice_stats(&self) -> Result<PracticeStats, RepositoryError> {
        self.repository.practice_stats()
    }
}

impl<R> TrainerService<R>
where
    R: ContentLocaleRepository,
{
    pub fn set_content_locale(&mut self, locale: &str) -> Result<(), RepositoryError> {
        self.repository.set_content_locale(locale)
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
                notes_part: 1,
                topic_number: 1,
                title: String::from("Architecture"),
                description: None,
            }])
        }

        fn topic_mastery(&self, topic_id: TopicId) -> Result<TopicMasteryStats, RepositoryError> {
            Ok(TopicMasteryStats {
                topic_id,
                completed_attempts: 0,
                best_correct_answers: 0,
                best_answered_questions: 0,
                attempts_to_pass: None,
            })
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
                correct_option_ids: answer_option_ids.to_vec(),
                is_correct: true,
            })
        }

        fn active_session(&self) -> Result<Option<SessionProgress>, RepositoryError> {
            Ok(None)
        }

        fn cancel_session(&mut self, _session_id: SessionId) -> Result<(), RepositoryError> {
            Ok(())
        }

        fn statistics(&self, _filter: &StatisticsFilter) -> Result<TrainingStats, RepositoryError> {
            Ok(TrainingStats::default())
        }

        fn clear_statistics(&mut self) -> Result<(), RepositoryError> {
            Ok(())
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
