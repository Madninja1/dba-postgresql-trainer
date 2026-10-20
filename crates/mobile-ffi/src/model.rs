use dba_trainer_domain::{
    ActivityStats, AnswerResult, CourseProgress, DailyActivity, Lab, PracticeCategoryStats,
    PracticeCheckResult, PracticeColumn, PracticeDifficulty, PracticeExecutionResult, PracticeKind,
    PracticeRule, PracticeSchema, PracticeStats, PracticeTable, PracticeTableData, PracticeTask,
    PracticeTaskProgress, ProgressLevel, Question, QuestionAttemptHistory, QuestionLimit,
    QuestionMistakeDetail, QuestionMistakeSummary, QuestionType, QuizScope, QuizSession,
    SessionProgress, Source, SourceKind, StatisticsLimit, Topic, TopicMasteryStats, TrainingStats,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MobilePracticeKind {
    WriteQuery,
    FixQuery,
    FindLogicalError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MobilePracticeDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MobilePracticeRuleKind {
    RequireKeyword,
    ForbidKeyword,
    ForbidSelectStar,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeRule {
    pub kind: MobilePracticeRuleKind,
    pub value: Option<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeTask {
    pub id: i64,
    pub key: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub kind: MobilePracticeKind,
    pub difficulty: MobilePracticeDifficulty,
    pub starter_sql: String,
    pub solution_sql: String,
    pub solution_explanation: Option<String>,
    pub hints: Vec<String>,
    pub rules: Vec<MobilePracticeRule>,
    pub sort_order: i64,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeTaskProgress {
    pub task_id: i64,
    pub last_sql: String,
    pub attempts: u64,
    pub solved: bool,
    pub solved_attempt: Option<u64>,
    pub updated_at: String,
    pub started: bool,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeCategoryStats {
    pub category: String,
    pub total_tasks: u64,
    pub started_tasks: u64,
    pub solved_tasks: u64,
    pub total_attempts: u64,
    pub solved_first_try: u64,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeStats {
    pub total_tasks: u64,
    pub started_tasks: u64,
    pub solved_tasks: u64,
    pub total_attempts: u64,
    pub solved_first_try: u64,
    pub solved_percent: f64,
    pub categories: Vec<MobilePracticeCategoryStats>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeRow {
    pub values: Vec<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeExecutionResult {
    pub columns: Vec<String>,
    pub rows: Vec<MobilePracticeRow>,
    pub affected_rows: Option<u64>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeCheckResult {
    pub is_correct: bool,
    pub actual_rows: u64,
    pub expected_rows: u64,
    pub failed_checks: Vec<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeColumn {
    pub name: String,
    pub data_type: String,
    pub not_null: bool,
    pub primary_key: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeTable {
    pub name: String,
    pub columns: Vec<MobilePracticeColumn>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeSchema {
    pub tables: Vec<MobilePracticeTable>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct MobilePracticeTableData {
    pub table: String,
    pub result: MobilePracticeExecutionResult,
    pub offset: u64,
    pub has_more: bool,
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

impl From<PracticeKind> for MobilePracticeKind {
    fn from(value: PracticeKind) -> Self {
        match value {
            PracticeKind::WriteQuery => Self::WriteQuery,
            PracticeKind::FixQuery => Self::FixQuery,
            PracticeKind::FindLogicalError => Self::FindLogicalError,
        }
    }
}

impl From<PracticeDifficulty> for MobilePracticeDifficulty {
    fn from(value: PracticeDifficulty) -> Self {
        match value {
            PracticeDifficulty::Easy => Self::Easy,
            PracticeDifficulty::Medium => Self::Medium,
            PracticeDifficulty::Hard => Self::Hard,
        }
    }
}

impl From<PracticeRule> for MobilePracticeRule {
    fn from(value: PracticeRule) -> Self {
        match value {
            PracticeRule::RequireKeyword(value) => Self {
                kind: MobilePracticeRuleKind::RequireKeyword,
                value: Some(value),
            },
            PracticeRule::ForbidKeyword(value) => Self {
                kind: MobilePracticeRuleKind::ForbidKeyword,
                value: Some(value),
            },
            PracticeRule::ForbidSelectStar => Self {
                kind: MobilePracticeRuleKind::ForbidSelectStar,
                value: None,
            },
        }
    }
}

impl From<PracticeTask> for MobilePracticeTask {
    fn from(value: PracticeTask) -> Self {
        Self {
            id: value.id,
            key: value.key,
            category: value.category,
            title: value.title,
            description: value.description,
            kind: value.kind.into(),
            difficulty: value.difficulty.into(),
            starter_sql: value.starter_sql,
            solution_sql: value.solution_sql,
            solution_explanation: value.solution_explanation,
            hints: value.hints,
            rules: value.rules.into_iter().map(Into::into).collect(),
            sort_order: value.sort_order,
        }
    }
}

impl From<PracticeTaskProgress> for MobilePracticeTaskProgress {
    fn from(value: PracticeTaskProgress) -> Self {
        let started = value.is_started();

        Self {
            task_id: value.task_id,
            last_sql: value.last_sql,
            attempts: value.attempts as u64,
            solved: value.solved,
            solved_attempt: value.solved_attempt.map(|attempt| attempt as u64),
            updated_at: value.updated_at,
            started,
        }
    }
}

impl From<PracticeCategoryStats> for MobilePracticeCategoryStats {
    fn from(value: PracticeCategoryStats) -> Self {
        Self {
            category: value.category,
            total_tasks: value.total_tasks as u64,
            started_tasks: value.started_tasks as u64,
            solved_tasks: value.solved_tasks as u64,
            total_attempts: value.total_attempts as u64,
            solved_first_try: value.solved_first_try as u64,
        }
    }
}

impl From<PracticeStats> for MobilePracticeStats {
    fn from(value: PracticeStats) -> Self {
        let solved_percent = value.solved_percent();

        Self {
            total_tasks: value.total_tasks as u64,
            started_tasks: value.started_tasks as u64,
            solved_tasks: value.solved_tasks as u64,
            total_attempts: value.total_attempts as u64,
            solved_first_try: value.solved_first_try as u64,
            solved_percent,
            categories: value.categories.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<PracticeExecutionResult> for MobilePracticeExecutionResult {
    fn from(value: PracticeExecutionResult) -> Self {
        Self {
            columns: value.columns,
            rows: value
                .rows
                .into_iter()
                .map(|row| MobilePracticeRow {
                    values: row.into_iter().map(|value| value.to_string()).collect(),
                })
                .collect(),
            affected_rows: value.affected_rows.map(|count| count as u64),
        }
    }
}

impl From<PracticeCheckResult> for MobilePracticeCheckResult {
    fn from(value: PracticeCheckResult) -> Self {
        Self {
            is_correct: value.is_correct,
            actual_rows: value.actual_rows as u64,
            expected_rows: value.expected_rows as u64,
            failed_checks: value.failed_checks,
        }
    }
}

impl From<PracticeColumn> for MobilePracticeColumn {
    fn from(value: PracticeColumn) -> Self {
        Self {
            name: value.name,
            data_type: value.data_type,
            not_null: value.not_null,
            primary_key: value.primary_key,
            default_value: value.default_value,
        }
    }
}

impl From<PracticeTable> for MobilePracticeTable {
    fn from(value: PracticeTable) -> Self {
        Self {
            name: value.name,
            columns: value.columns.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<PracticeSchema> for MobilePracticeSchema {
    fn from(value: PracticeSchema) -> Self {
        Self {
            tables: value.tables.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<PracticeTableData> for MobilePracticeTableData {
    fn from(value: PracticeTableData) -> Self {
        Self {
            table: value.table,
            result: value.result.into(),
            offset: value.offset as u64,
            has_more: value.has_more,
        }
    }
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
