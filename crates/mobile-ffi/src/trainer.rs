use std::sync::{Arc, Mutex, MutexGuard};

use dba_trainer_application::{PracticeEngine, TrainerService};
use dba_trainer_content::{
    DEFAULT_CONTENT_LOCALE, load_builtin_bundles, load_builtin_labs_for_locale,
    load_builtin_practice_for_locale,
};
use dba_trainer_domain::{
    AnswerOptionId, PracticeTask, QuestionId, QuizScope, SessionConfig, SessionId,
    StatisticsFilter, StatisticsScope, TopicId,
};
use dba_trainer_storage_sqlite::{SqlitePracticeEngine, SqliteRepository};

use crate::{
    MobileActivityStats, MobileAnswerResult, MobileError, MobileLab, MobilePracticeCheckResult,
    MobilePracticeExecutionResult, MobilePracticeSchema, MobilePracticeStats,
    MobilePracticeTableData, MobilePracticeTask, MobilePracticeTaskProgress, MobileQuestion,
    MobileQuestionLimit, MobileQuestionMistakeDetail, MobileQuestionMistakeSummary,
    MobileQuizSession, MobileSessionProgress, MobileStatisticsLimit, MobileTopic,
    MobileTopicMasteryStats, MobileTrainingStats, error::to_mobile_error,
};

struct PracticeState {
    engine: SqlitePracticeEngine,
    task_id: Option<i64>,
}

impl Default for PracticeState {
    fn default() -> Self {
        Self {
            engine: SqlitePracticeEngine::new(),
            task_id: None,
        }
    }
}

#[derive(uniffi::Object)]
pub struct MobileTrainer {
    service: Mutex<TrainerService<SqliteRepository>>,
    practice: Mutex<PracticeState>,
}

#[uniffi::export]
impl MobileTrainer {
    #[uniffi::constructor]
    pub fn new(database_path: String) -> Result<Arc<Self>, MobileError> {
        let mut repository = SqliteRepository::open(database_path).map_err(to_mobile_error)?;

        let bundles = load_builtin_bundles().map_err(to_mobile_error)?;

        for bundle in &bundles {
            repository.sync_bundle(bundle).map_err(to_mobile_error)?;
        }

        let labs = load_builtin_labs_for_locale(DEFAULT_CONTENT_LOCALE).map_err(to_mobile_error)?;
        for lab in &labs {
            repository.sync_lab(lab).map_err(to_mobile_error)?;
        }

        let practice =
            load_builtin_practice_for_locale(DEFAULT_CONTENT_LOCALE).map_err(to_mobile_error)?;
        for task in &practice {
            repository.sync_practice(task).map_err(to_mobile_error)?;
        }

        let mut service = TrainerService::new(repository);
        service.record_app_open().map_err(to_mobile_error)?;

        Ok(Arc::new(Self {
            service: Mutex::new(service),
            practice: Mutex::new(PracticeState::default()),
        }))
    }

    pub fn set_content_locale(&self, locale: String) -> Result<(), MobileError> {
        {
            let mut service = self.service()?;
            service
                .set_content_locale(&locale)
                .map_err(to_mobile_error)?;
        }

        let mut practice = self.practice()?;
        *practice = PracticeState::default();
        Ok(())
    }

    pub fn topics(&self) -> Result<Vec<MobileTopic>, MobileError> {
        let service = self.service()?;

        service
            .topics()
            .map(|topics| topics.into_iter().map(MobileTopic::from).collect())
            .map_err(to_mobile_error)
    }

    pub fn topic_mastery(&self, topic_id: i64) -> Result<MobileTopicMasteryStats, MobileError> {
        let service = self.service()?;

        service
            .topic_mastery(TopicId(topic_id))
            .map(MobileTopicMasteryStats::from)
            .map_err(to_mobile_error)
    }

    pub fn labs(&self) -> Result<Vec<MobileLab>, MobileError> {
        let service = self.service()?;

        service
            .labs()
            .map(|labs| labs.into_iter().map(MobileLab::from).collect())
            .map_err(to_mobile_error)
    }

    pub fn practice_tasks(&self) -> Result<Vec<MobilePracticeTask>, MobileError> {
        let service = self.service()?;

        service
            .practice_tasks()
            .map(|tasks| tasks.into_iter().map(MobilePracticeTask::from).collect())
            .map_err(to_mobile_error)
    }

    pub fn practice_progress(&self) -> Result<Vec<MobilePracticeTaskProgress>, MobileError> {
        let service = self.service()?;

        service
            .practice_progress()
            .map(|progress| {
                progress
                    .into_iter()
                    .map(MobilePracticeTaskProgress::from)
                    .collect()
            })
            .map_err(to_mobile_error)
    }

    pub fn practice_stats(&self) -> Result<MobilePracticeStats, MobileError> {
        let service = self.service()?;

        service
            .practice_stats()
            .map(MobilePracticeStats::from)
            .map_err(to_mobile_error)
    }

    pub fn practice_load_task(&self, task_id: i64) -> Result<MobilePracticeSchema, MobileError> {
        let task = self.practice_task(task_id)?;
        let mut practice = self.practice()?;
        practice.engine.load_task(&task).map_err(to_mobile_error)?;
        practice.task_id = Some(task_id);

        practice
            .engine
            .schema()
            .map(MobilePracticeSchema::from)
            .map_err(to_mobile_error)
    }

    pub fn practice_save_draft(&self, task_id: i64, sql: String) -> Result<(), MobileError> {
        let mut service = self.service()?;
        service
            .save_practice_draft(task_id, &sql)
            .map_err(to_mobile_error)
    }

    pub fn practice_execute(
        &self,
        task_id: i64,
        sql: String,
    ) -> Result<MobilePracticeExecutionResult, MobileError> {
        self.practice_save_draft(task_id, sql.clone())?;
        let task = self.practice_task(task_id)?;
        let mut practice = self.practice()?;
        Self::ensure_practice_loaded(&mut practice, &task)?;

        practice
            .engine
            .execute(&sql)
            .map(MobilePracticeExecutionResult::from)
            .map_err(to_mobile_error)
    }

    pub fn practice_check(
        &self,
        task_id: i64,
        sql: String,
    ) -> Result<MobilePracticeCheckResult, MobileError> {
        let task = self.practice_task(task_id)?;
        let result = {
            let practice = self.practice()?;
            practice
                .engine
                .check(&task, &sql)
                .map_err(to_mobile_error)?
        };

        let mut service = self.service()?;
        service
            .save_practice_draft(task_id, &sql)
            .map_err(to_mobile_error)?;
        service
            .record_practice_attempt(task_id, &sql, result.is_correct)
            .map_err(to_mobile_error)?;

        Ok(result.into())
    }

    pub fn practice_reset(&self, task_id: i64) -> Result<MobilePracticeSchema, MobileError> {
        let task = self.practice_task(task_id)?;
        {
            let mut service = self.service()?;
            service
                .save_practice_draft(task_id, &task.starter_sql)
                .map_err(to_mobile_error)?;
        }

        let mut practice = self.practice()?;
        practice.engine.reset(&task).map_err(to_mobile_error)?;
        practice.task_id = Some(task_id);
        practice
            .engine
            .schema()
            .map(MobilePracticeSchema::from)
            .map_err(to_mobile_error)
    }

    pub fn practice_schema(&self, task_id: i64) -> Result<MobilePracticeSchema, MobileError> {
        let task = self.practice_task(task_id)?;
        let mut practice = self.practice()?;
        Self::ensure_practice_loaded(&mut practice, &task)?;

        practice
            .engine
            .schema()
            .map(MobilePracticeSchema::from)
            .map_err(to_mobile_error)
    }

    pub fn practice_table_data(
        &self,
        task_id: i64,
        table: String,
        limit: u64,
        offset: u64,
    ) -> Result<MobilePracticeTableData, MobileError> {
        let task = self.practice_task(task_id)?;
        let mut practice = self.practice()?;
        Self::ensure_practice_loaded(&mut practice, &task)?;

        practice
            .engine
            .table_data(&table, limit as usize, offset as usize)
            .map(MobilePracticeTableData::from)
            .map_err(to_mobile_error)
    }

    pub fn active_session(&self) -> Result<Option<MobileSessionProgress>, MobileError> {
        let service = self.service()?;

        service
            .active_session()
            .map(|progress| progress.map(MobileSessionProgress::from))
            .map_err(to_mobile_error)
    }

    pub fn start_topic_session(
        &self,
        topic_id: i64,
        limit: MobileQuestionLimit,
    ) -> Result<MobileQuizSession, MobileError> {
        self.start_session(SessionConfig {
            scope: QuizScope::Topic(TopicId(topic_id)),
            limit: limit.into(),
        })
    }

    pub fn start_course_session(
        &self,
        course_code: String,
        limit: MobileQuestionLimit,
    ) -> Result<MobileQuizSession, MobileError> {
        self.start_session(SessionConfig {
            scope: QuizScope::Course(course_code),
            limit: limit.into(),
        })
    }

    pub fn start_all_topics_session(
        &self,
        limit: MobileQuestionLimit,
    ) -> Result<MobileQuizSession, MobileError> {
        self.start_session(SessionConfig {
            scope: QuizScope::AllTopics,
            limit: limit.into(),
        })
    }

    pub fn start_mistake_session(
        &self,
        limit: MobileQuestionLimit,
    ) -> Result<MobileQuizSession, MobileError> {
        self.start_session(SessionConfig {
            scope: QuizScope::Mistakes,
            limit: limit.into(),
        })
    }

    pub fn start_mistake_course_session(
        &self,
        course_code: String,
        limit: MobileQuestionLimit,
    ) -> Result<MobileQuizSession, MobileError> {
        self.start_session(SessionConfig {
            scope: QuizScope::MistakesCourse(course_code),
            limit: limit.into(),
        })
    }

    pub fn start_mistake_topic_session(
        &self,
        topic_id: i64,
        limit: MobileQuestionLimit,
    ) -> Result<MobileQuizSession, MobileError> {
        self.start_session(SessionConfig {
            scope: QuizScope::MistakesTopic(TopicId(topic_id)),
            limit: limit.into(),
        })
    }

    pub fn current_question(&self, session_id: i64) -> Result<Option<MobileQuestion>, MobileError> {
        let service = self.service()?;

        service
            .current_question(SessionId(session_id))
            .map(|question| question.map(MobileQuestion::from))
            .map_err(to_mobile_error)
    }

    pub fn submit_answer(
        &self,
        session_id: i64,
        question_id: i64,
        answer_option_ids: Vec<i64>,
    ) -> Result<MobileAnswerResult, MobileError> {
        let answer_option_ids = answer_option_ids
            .into_iter()
            .map(AnswerOptionId)
            .collect::<Vec<_>>();

        let mut service = self.service()?;

        service
            .submit_answer(
                SessionId(session_id),
                QuestionId(question_id),
                &answer_option_ids,
            )
            .map(MobileAnswerResult::from)
            .map_err(to_mobile_error)
    }

    pub fn cancel_session(&self, session_id: i64) -> Result<(), MobileError> {
        let mut service = self.service()?;

        service
            .cancel_session(SessionId(session_id))
            .map_err(to_mobile_error)
    }

    pub fn statistics_all(
        &self,
        limit: MobileStatisticsLimit,
    ) -> Result<MobileTrainingStats, MobileError> {
        self.statistics(StatisticsFilter {
            scope: StatisticsScope::All,
            limit: limit.into(),
        })
    }

    pub fn statistics_course(
        &self,
        course_code: String,
        limit: MobileStatisticsLimit,
    ) -> Result<MobileTrainingStats, MobileError> {
        self.statistics(StatisticsFilter {
            scope: StatisticsScope::Course(course_code),
            limit: limit.into(),
        })
    }

    pub fn statistics_topic(
        &self,
        topic_id: i64,
        limit: MobileStatisticsLimit,
    ) -> Result<MobileTrainingStats, MobileError> {
        self.statistics(StatisticsFilter {
            scope: StatisticsScope::Topic(TopicId(topic_id)),
            limit: limit.into(),
        })
    }

    pub fn mistake_summaries(&self) -> Result<Vec<MobileQuestionMistakeSummary>, MobileError> {
        let service = self.service()?;

        service
            .mistake_summaries()
            .map(|summaries| {
                summaries
                    .into_iter()
                    .map(MobileQuestionMistakeSummary::from)
                    .collect()
            })
            .map_err(to_mobile_error)
    }

    pub fn mistake_detail(
        &self,
        question_id: i64,
    ) -> Result<MobileQuestionMistakeDetail, MobileError> {
        let service = self.service()?;

        service
            .mistake_detail(QuestionId(question_id))
            .map(MobileQuestionMistakeDetail::from)
            .map_err(to_mobile_error)
    }

    pub fn activity_stats(&self) -> Result<MobileActivityStats, MobileError> {
        let service = self.service()?;

        service
            .activity_stats()
            .map(MobileActivityStats::from)
            .map_err(to_mobile_error)
    }

    pub fn clear_statistics(&self) -> Result<(), MobileError> {
        let mut service = self.service()?;
        service.clear_statistics().map_err(to_mobile_error)
    }
}

impl MobileTrainer {
    fn practice(&self) -> Result<MutexGuard<'_, PracticeState>, MobileError> {
        self.practice
            .lock()
            .map_err(|error| MobileError::Core(error.to_string()))
    }

    fn practice_task(&self, task_id: i64) -> Result<PracticeTask, MobileError> {
        let service = self.service()?;
        service
            .practice_tasks()
            .map_err(to_mobile_error)?
            .into_iter()
            .find(|task| task.id == task_id)
            .ok_or_else(|| MobileError::Core(format!("practice task {task_id} was not found")))
    }

    fn ensure_practice_loaded(
        practice: &mut PracticeState,
        task: &PracticeTask,
    ) -> Result<(), MobileError> {
        if practice.task_id != Some(task.id) {
            practice.engine.load_task(task).map_err(to_mobile_error)?;
            practice.task_id = Some(task.id);
        }
        Ok(())
    }

    fn service(&self) -> Result<MutexGuard<'_, TrainerService<SqliteRepository>>, MobileError> {
        self.service
            .lock()
            .map_err(|error| MobileError::Core(error.to_string()))
    }

    fn start_session(&self, config: SessionConfig) -> Result<MobileQuizSession, MobileError> {
        let mut service = self.service()?;

        service
            .start_session(&config)
            .map(MobileQuizSession::from)
            .map_err(to_mobile_error)
    }

    fn statistics(&self, filter: StatisticsFilter) -> Result<MobileTrainingStats, MobileError> {
        let service = self.service()?;

        service
            .statistics(&filter)
            .map(MobileTrainingStats::from)
            .map_err(to_mobile_error)
    }
}
