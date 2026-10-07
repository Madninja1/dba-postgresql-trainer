use dba_trainer_domain::{
    AnswerResult, Question, QuestionLimit, QuestionType, QuizScope, QuizSession, SessionProgress,
    Source, SourceKind, StatisticsLimit, Topic, TrainingStats,
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
    pub title: String,
    pub description: Option<String>,
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
            title: value.title,
            description: value.description,
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
        let (topic_id, course_code) = match value.scope {
            QuizScope::Topic(topic_id) => (Some(topic_id.0), None),
            QuizScope::Course(course_code) => (None, Some(course_code)),
            QuizScope::AllTopics => (None, None),
        };

        Self {
            id: value.id.0,
            topic_id,
            course_code,
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
