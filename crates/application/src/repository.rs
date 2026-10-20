use dba_trainer_domain::{
    ActivityStats, AnswerOptionId, AnswerResult, Lab, PracticeStats, PracticeTask,
    PracticeTaskProgress, Question, QuestionId, QuestionMistakeDetail, QuestionMistakeSummary,
    QuizSession, SessionConfig, SessionId, SessionProgress, StatisticsFilter, Topic, TopicId,
    TopicMasteryStats, TrainingStats,
};

use crate::RepositoryError;

pub trait TopicRepository {
    fn topics(&self) -> Result<Vec<Topic>, RepositoryError>;

    fn topic_mastery(&self, topic_id: TopicId) -> Result<TopicMasteryStats, RepositoryError>;
}

pub trait LabRepository {
    fn labs(&self) -> Result<Vec<Lab>, RepositoryError>;
}

pub trait LearningHistoryRepository {
    fn mistake_summaries(&self) -> Result<Vec<QuestionMistakeSummary>, RepositoryError>;

    fn mistake_detail(
        &self,
        question_id: QuestionId,
    ) -> Result<QuestionMistakeDetail, RepositoryError>;

    fn activity_stats(&self) -> Result<ActivityStats, RepositoryError>;

    fn record_app_open(&mut self) -> Result<(), RepositoryError>;
}

pub trait PracticeRepository {
    fn practice_tasks(&self) -> Result<Vec<PracticeTask>, RepositoryError>;

    fn practice_task(&self, key: &str) -> Result<PracticeTask, RepositoryError>;

    fn practice_progress(&self) -> Result<Vec<PracticeTaskProgress>, RepositoryError>;

    fn save_practice_draft(&mut self, task_id: i64, sql: &str) -> Result<(), RepositoryError>;

    fn record_practice_attempt(
        &mut self,
        task_id: i64,
        sql: &str,
        success: bool,
    ) -> Result<(), RepositoryError>;

    fn practice_stats(&self) -> Result<PracticeStats, RepositoryError>;
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
