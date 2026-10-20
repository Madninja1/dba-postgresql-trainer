use dba_trainer_domain::{
    ActivityStats, AnswerResult, CourseProgress, DailyActivity, Lab, ProgressLevel, Question,
    QuestionAttemptHistory, QuestionLimit, QuestionMistakeDetail, QuestionMistakeSummary,
    QuestionType, QuizScope, QuizSession, SessionProgress, Source, SourceKind, StatisticsLimit,
    Topic, TopicMasteryStats, TrainingStats,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MobileQuestionLimit {
    Twenty,
    Fifty,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MobileStatisticsLimit {
    Any,
    Twenty,
    Fifty,
    AllQuestions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MobileQuestionType {
    SingleChoice,
    MultipleChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MobileSourceKind {
    CourseMaterial,
    PostgreSqlDocs,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileTopic {
    pub id: i64,
    pub course_code: String,
    pub slug: String,
    pub notes_part: i64,
    pub topic_number: i64,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileLab {
    pub id: i64,
    pub course_code: String,
    pub key: String,
    pub lab_number: i64,
    pub title: String,
    pub description: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileTopicMasteryStats {
    pub topic_id: i64,
    pub completed_attempts: u64,
    pub best_accuracy_percent: f64,
    pub pass_threshold_percent: f64,
    pub attempts_to_pass: Option<u64>,
    pub passed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MobileProgressLevel {
    ColdStart,
    QueryRunner,
    WarmCache,
    BackendTamer,
    WalKeeper,
    IncidentHandler,
    DbaOneMaster,
    ProductionSurvivor,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileQuestionMistakeSummary {
    pub question_id: i64,
    pub topic_id: i64,
    pub course_code: String,
    pub topic_title: String,
    pub question_text: String,
    pub total_attempts: u64,
    pub incorrect_attempts: u64,
    pub error_rate_percent: f64,
    pub last_incorrect_at: String,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileQuestionAttemptHistory {
    pub answered_at: String,
    pub is_correct: bool,
    pub selected_answers: Vec<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileQuestionMistakeDetail {
    pub summary: MobileQuestionMistakeSummary,
    pub question: MobileQuestion,
    pub correct_answers: Vec<String>,
    pub attempts: Vec<MobileQuestionAttemptHistory>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileDailyActivity {
    pub date: String,
    pub open_count: u64,
    pub answered_questions: u64,
    pub correct_answers: u64,
    pub completed_sessions: u64,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileCourseProgress {
    pub course_code: String,
    pub passed_topics: u64,
    pub total_topics: u64,
    pub completion_percent: f64,
    pub completed: bool,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileActivityStats {
    pub active_days: u64,
    pub study_days: u64,
    pub study_day_question_target: u64,
    pub current_streak: u64,
    pub best_streak: u64,
    pub total_answered_questions: u64,
    pub today_answered_questions: u64,
    pub today_correct_answers: u64,
    pub last_7_days_answered_questions: u64,
    pub passed_topics: u64,
    pub total_topics: u64,
    pub overall_mastery_percent: f64,
    pub level: MobileProgressLevel,
    pub recent_days: Vec<MobileDailyActivity>,
    pub course_progress: Vec<MobileCourseProgress>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileAnswerOption {
    pub id: i64,
    pub text: String,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileSource {
    pub kind: MobileSourceKind,
    pub module: String,
    pub section: String,
    pub locator: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileQuestion {
    pub id: i64,
    pub topic_id: i64,
    pub question_type: MobileQuestionType,
    pub text: String,
    pub explanation: String,
    pub source: MobileSource,
    pub options: Vec<MobileAnswerOption>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileQuizSession {
    pub id: i64,
    pub current_index: u64,
    pub total_questions: u64,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileSessionProgress {
    pub id: i64,
    pub topic_id: Option<i64>,
    pub course_code: Option<String>,
    pub is_review: bool,
    pub current_index: u64,
    pub total_questions: u64,
    pub answered_questions: u64,
    pub correct_answers: u64,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileAnswerResult {
    pub question_id: i64,
    pub selected_option_ids: Vec<i64>,
    pub correct_option_ids: Vec<i64>,
    pub is_correct: bool,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobileTrainingStats {
    pub completed_sessions: u64,
    pub cancelled_sessions: u64,
    pub answered_questions: u64,
    pub correct_answers: u64,
    pub incorrect_answers: u64,
    pub accuracy_percent: f64,
}

impl From<MobileQuestionLimit> for QuestionLimit {
    fn from(value: MobileQuestionLimit) -> Self {
        match value {
            MobileQuestionLimit::Twenty => Self::Twenty,
            MobileQuestionLimit::Fifty => Self::Fifty,
            MobileQuestionLimit::All => Self::All,
        }
    }
}

impl From<MobileStatisticsLimit> for StatisticsLimit {
    fn from(value: MobileStatisticsLimit) -> Self {
        match value {
            MobileStatisticsLimit::Any => Self::Any,
            MobileStatisticsLimit::Twenty => Self::Twenty,
            MobileStatisticsLimit::Fifty => Self::Fifty,
            MobileStatisticsLimit::AllQuestions => Self::AllQuestions,
        }
    }
}

impl From<Topic> for MobileTopic {
    fn from(value: Topic) -> Self {
        Self {
            id: value.id.0,
            course_code: value.course_code,
            slug: value.slug,
            notes_part: value.notes_part,
            topic_number: value.topic_number,
            title: value.title,
            description: value.description,
        }
    }
}

impl From<Lab> for MobileLab {
    fn from(value: Lab) -> Self {
        Self {
            id: value.id,
            course_code: value.course_code,
            key: value.key,
            lab_number: value.lab_number,
            title: value.title,
            description: value.description,
            body: value.body,
        }
    }
}

impl From<TopicMasteryStats> for MobileTopicMasteryStats {
    fn from(value: TopicMasteryStats) -> Self {
        Self {
            topic_id: value.topic_id.0,
            completed_attempts: value.completed_attempts as u64,
            best_accuracy_percent: value.best_accuracy_percent(),
            pass_threshold_percent: dba_trainer_domain::TOPIC_PASS_THRESHOLD_PERCENT,
            attempts_to_pass: value.attempts_to_pass.map(|attempt| attempt as u64),
            passed: value.passed(),
        }
    }
}

impl From<Question> for MobileQuestion {
    fn from(value: Question) -> Self {
        Self {
            id: value.id.0,
            topic_id: value.topic_id.0,
            question_type: value.question_type.into(),
            text: value.text,
            explanation: value.explanation,
            source: value.source.into(),
            options: value
                .options
                .into_iter()
                .map(|option| MobileAnswerOption {
                    id: option.id.0,
                    text: option.text,
                })
                .collect(),
        }
    }
}

impl From<QuestionType> for MobileQuestionType {
    fn from(value: QuestionType) -> Self {
        match value {
            QuestionType::SingleChoice => Self::SingleChoice,
            QuestionType::MultipleChoice => Self::MultipleChoice,
        }
    }
}

impl From<Source> for MobileSource {
    fn from(value: Source) -> Self {
        Self {
            kind: value.kind.into(),
            module: value.module,
            section: value.section,
            locator: value.locator,
            url: value.url,
        }
    }
}

impl From<SourceKind> for MobileSourceKind {
    fn from(value: SourceKind) -> Self {
        match value {
            SourceKind::CourseMaterial => Self::CourseMaterial,
            SourceKind::PostgreSqlDocs => Self::PostgreSqlDocs,
        }
    }
}

impl From<QuizSession> for MobileQuizSession {
    fn from(value: QuizSession) -> Self {
        Self {
            id: value.id.0,
            current_index: value.current_index as u64,
            total_questions: value.total_questions() as u64,
        }
    }
}

impl From<SessionProgress> for MobileSessionProgress {
    fn from(value: SessionProgress) -> Self {
        let (topic_id, course_code, is_review) = match value.scope {
            QuizScope::Topic(topic_id) => (Some(topic_id.0), None, false),
            QuizScope::Course(course_code) => (None, Some(course_code), false),
            QuizScope::AllTopics => (None, None, false),
            QuizScope::Mistakes => (None, None, true),
            QuizScope::MistakesCourse(course_code) => (None, Some(course_code), true),
            QuizScope::MistakesTopic(topic_id) => (Some(topic_id.0), None, true),
        };

        Self {
            id: value.id.0,
            topic_id,
            course_code,
            is_review,
            current_index: value.current_index as u64,
            total_questions: value.total_questions as u64,
            answered_questions: value.answered_questions as u64,
            correct_answers: value.correct_answers as u64,
        }
    }
}

impl From<AnswerResult> for MobileAnswerResult {
    fn from(value: AnswerResult) -> Self {
        Self {
            question_id: value.question_id.0,
            selected_option_ids: value
                .selected_option_ids
                .into_iter()
                .map(|id| id.0)
                .collect(),
            correct_option_ids: value
                .correct_option_ids
                .into_iter()
                .map(|id| id.0)
                .collect(),
            is_correct: value.is_correct,
        }
    }
}

impl From<ProgressLevel> for MobileProgressLevel {
    fn from(value: ProgressLevel) -> Self {
        match value {
            ProgressLevel::ColdStart => Self::ColdStart,
            ProgressLevel::QueryRunner => Self::QueryRunner,
            ProgressLevel::WarmCache => Self::WarmCache,
            ProgressLevel::BackendTamer => Self::BackendTamer,
            ProgressLevel::WalKeeper => Self::WalKeeper,
            ProgressLevel::IncidentHandler => Self::IncidentHandler,
            ProgressLevel::DbaOneMaster => Self::DbaOneMaster,
            ProgressLevel::ProductionSurvivor => Self::ProductionSurvivor,
        }
    }
}

impl From<QuestionMistakeSummary> for MobileQuestionMistakeSummary {
    fn from(value: QuestionMistakeSummary) -> Self {
        let error_rate_percent = value.error_rate_percent();

        Self {
            question_id: value.question_id.0,
            topic_id: value.topic_id.0,
            course_code: value.course_code,
            topic_title: value.topic_title,
            question_text: value.question_text,
            total_attempts: value.total_attempts as u64,
            incorrect_attempts: value.incorrect_attempts as u64,
            error_rate_percent,
            last_incorrect_at: value.last_incorrect_at,
        }
    }
}

impl From<QuestionAttemptHistory> for MobileQuestionAttemptHistory {
    fn from(value: QuestionAttemptHistory) -> Self {
        Self {
            answered_at: value.answered_at,
            is_correct: value.is_correct,
            selected_answers: value.selected_answers,
        }
    }
}

impl From<QuestionMistakeDetail> for MobileQuestionMistakeDetail {
    fn from(value: QuestionMistakeDetail) -> Self {
        Self {
            summary: value.summary.into(),
            question: value.question.into(),
            correct_answers: value.correct_answers,
            attempts: value.attempts.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<DailyActivity> for MobileDailyActivity {
    fn from(value: DailyActivity) -> Self {
        Self {
            date: value.date,
            open_count: value.open_count as u64,
            answered_questions: value.answered_questions as u64,
            correct_answers: value.correct_answers as u64,
            completed_sessions: value.completed_sessions as u64,
        }
    }
}

impl From<CourseProgress> for MobileCourseProgress {
    fn from(value: CourseProgress) -> Self {
        let completion_percent = value.completion_percent();
        let completed = value.completed();

        Self {
            course_code: value.course_code,
            passed_topics: value.passed_topics as u64,
            total_topics: value.total_topics as u64,
            completion_percent,
            completed,
        }
    }
}

impl From<ActivityStats> for MobileActivityStats {
    fn from(value: ActivityStats) -> Self {
        let passed_topics = value.passed_topics();
        let total_topics = value.total_topics();
        let overall_mastery_percent = value.overall_mastery_percent();
        let level = value.level();

        Self {
            active_days: value.active_days as u64,
            study_days: value.study_days as u64,
            study_day_question_target: dba_trainer_domain::STUDY_DAY_QUESTION_TARGET as u64,
            current_streak: value.current_streak as u64,
            best_streak: value.best_streak as u64,
            total_answered_questions: value.total_answered_questions as u64,
            today_answered_questions: value.today_answered_questions as u64,
            today_correct_answers: value.today_correct_answers as u64,
            last_7_days_answered_questions: value.last_7_days_answered_questions as u64,
            passed_topics: passed_topics as u64,
            total_topics: total_topics as u64,
            overall_mastery_percent,
            level: level.into(),
            recent_days: value.recent_days.into_iter().map(Into::into).collect(),
            course_progress: value.course_progress.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<TrainingStats> for MobileTrainingStats {
    fn from(value: TrainingStats) -> Self {
        Self {
            completed_sessions: value.completed_sessions as u64,
            cancelled_sessions: value.cancelled_sessions as u64,
            answered_questions: value.answered_questions as u64,
            correct_answers: value.correct_answers as u64,
            incorrect_answers: value.incorrect_answers() as u64,
            accuracy_percent: value.accuracy_percent(),
        }
    }
}
