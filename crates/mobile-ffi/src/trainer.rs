use std::sync::{Arc, Mutex, MutexGuard};

use dba_trainer_application::TrainerService;
use dba_trainer_content::load_builtin_bundles;
use dba_trainer_domain::{
    AnswerOptionId, QuestionId, QuizScope, SessionConfig, SessionId, StatisticsFilter,
    StatisticsScope, TopicId,
};
use dba_trainer_storage_sqlite::SqliteRepository;

use crate::{
    MobileAnswerResult, MobileError, MobileQuestion, MobileQuestionLimit, MobileQuizSession,
    MobileSessionProgress, MobileStatisticsLimit, MobileTopic, MobileTrainingStats,
    error::to_mobile_error,
};

#[derive(uniffi::Object)]
pub struct MobileTrainer {
    service: Mutex<TrainerService<SqliteRepository>>,
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

        Ok(Arc::new(Self {
            service: Mutex::new(TrainerService::new(repository)),
        }))
    }

    pub fn set_content_locale(&self, locale: String) -> Result<(), MobileError> {
        let mut service = self.service()?;

        service.set_content_locale(&locale).map_err(to_mobile_error)
    }

    pub fn topics(&self) -> Result<Vec<MobileTopic>, MobileError> {
        let service = self.service()?;

        service
            .topics()
            .map(|topics| topics.into_iter().map(MobileTopic::from).collect())
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

    pub fn clear_statistics(&self) -> Result<(), MobileError> {
        let mut service = self.service()?;
        service.clear_statistics().map_err(to_mobile_error)
    }
}

impl MobileTrainer {
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
