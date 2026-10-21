use crate::{Question, QuestionId, QuestionLimit, QuizScope, TopicId};

pub const STUDY_DAY_QUESTION_TARGET: usize = 10;

pub const XP_UNIQUE_CORRECT_QUESTION: usize = 1;
pub const XP_PASSED_QUIZ_20: usize = 10;
pub const XP_PASSED_QUIZ_50: usize = 20;
pub const XP_PASSED_QUIZ_ALL: usize = 30;
pub const XP_MASTERED_TOPIC: usize = 50;
pub const XP_COMPLETED_COURSE: usize = 200;
pub const XP_PRACTICE_EASY: usize = 10;
pub const XP_PRACTICE_MEDIUM: usize = 20;
pub const XP_PRACTICE_HARD: usize = 35;
pub const XP_PRACTICE_FIRST_TRY: usize = 5;
pub const XP_PRACTICE_NO_HINTS: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XpSource {
    UniqueCorrectQuestion,
    PassedTwentyQuiz,
    PassedFiftyQuiz,
    PassedAllQuestionsQuiz,
    MasteredTopic,
    CompletedCourse,
    PracticeEasy,
    PracticeMedium,
    PracticeHard,
    PracticeFirstTry,
    PracticeNoHints,
}

impl XpSource {
    pub const fn points(self) -> usize {
        match self {
            Self::UniqueCorrectQuestion => XP_UNIQUE_CORRECT_QUESTION,
            Self::PassedTwentyQuiz => XP_PASSED_QUIZ_20,
            Self::PassedFiftyQuiz => XP_PASSED_QUIZ_50,
            Self::PassedAllQuestionsQuiz => XP_PASSED_QUIZ_ALL,
            Self::MasteredTopic => XP_MASTERED_TOPIC,
            Self::CompletedCourse => XP_COMPLETED_COURSE,
            Self::PracticeEasy => XP_PRACTICE_EASY,
            Self::PracticeMedium => XP_PRACTICE_MEDIUM,
            Self::PracticeHard => XP_PRACTICE_HARD,
            Self::PracticeFirstTry => XP_PRACTICE_FIRST_TRY,
            Self::PracticeNoHints => XP_PRACTICE_NO_HINTS,
        }
    }
}

pub const XP_SOURCES: [XpSource; 11] = [
    XpSource::UniqueCorrectQuestion,
    XpSource::PassedTwentyQuiz,
    XpSource::PassedFiftyQuiz,
    XpSource::PassedAllQuestionsQuiz,
    XpSource::MasteredTopic,
    XpSource::CompletedCourse,
    XpSource::PracticeEasy,
    XpSource::PracticeMedium,
    XpSource::PracticeHard,
    XpSource::PracticeFirstTry,
    XpSource::PracticeNoHints,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuizXpOpportunityScope {
    Topic(TopicId),
    Course(String),
    AllTopics,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuizXpOpportunity {
    pub scope: QuizXpOpportunityScope,
    pub unique_question_xp: usize,
    pub twenty_bonus_xp: usize,
    pub fifty_bonus_xp: usize,
    pub all_bonus_xp: usize,
    pub mastery_xp: usize,
    pub course_completion_xp: usize,
}

impl QuizXpOpportunity {
    pub fn pass_bonus_for_limit(&self, limit: QuestionLimit) -> usize {
        match limit {
            QuestionLimit::Twenty => self.twenty_bonus_xp,
            QuestionLimit::Fifty => self.fifty_bonus_xp,
            QuestionLimit::All => self.all_bonus_xp,
        }
    }

    pub fn total_available_xp(&self) -> usize {
        self.unique_question_xp
            + self.twenty_bonus_xp
            + self.fifty_bonus_xp
            + self.all_bonus_xp
            + self.mastery_xp
            + self.course_completion_xp
    }

    pub fn matches_scope(&self, scope: &QuizScope) -> bool {
        match (&self.scope, scope) {
            (QuizXpOpportunityScope::Topic(left), QuizScope::Topic(right)) => left == right,
            (QuizXpOpportunityScope::Course(left), QuizScope::Course(right)) => left == right,
            (QuizXpOpportunityScope::AllTopics, QuizScope::AllTopics) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeXpOpportunity {
    pub task_id: i64,
    pub solve_xp: usize,
    pub first_try_xp: usize,
    pub no_hints_xp: usize,
}

impl PracticeXpOpportunity {
    pub fn total_available_xp(&self) -> usize {
        self.solve_xp + self.first_try_xp + self.no_hints_xp
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct XpOpportunitySummary {
    pub question_xp: usize,
    pub quiz_xp: usize,
    pub mastery_xp: usize,
    pub course_xp: usize,
    pub practice_xp: usize,
}

impl XpOpportunitySummary {
    pub fn total_available_xp(&self) -> usize {
        self.question_xp + self.quiz_xp + self.mastery_xp + self.course_xp + self.practice_xp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressRank {
    #[default]
    Init,
    PsqlTrainee,
    QueryAdept,
    SchemaKeeper,
    PlanReader,
    WalOperator,
    PostgresGuardian,
    MadNinjaDba,
}

impl ProgressRank {
    pub const ALL: [Self; 8] = [
        Self::Init,
        Self::PsqlTrainee,
        Self::QueryAdept,
        Self::SchemaKeeper,
        Self::PlanReader,
        Self::WalOperator,
        Self::PostgresGuardian,
        Self::MadNinjaDba,
    ];

    pub const fn minimum_xp(self) -> usize {
        match self {
            Self::Init => 0,
            Self::PsqlTrainee => 100,
            Self::QueryAdept => 300,
            Self::SchemaKeeper => 700,
            Self::PlanReader => 1_400,
            Self::WalOperator => 2_500,
            Self::PostgresGuardian => 4_000,
            Self::MadNinjaDba => 6_000,
        }
    }

    pub const fn next(self) -> Option<Self> {
        match self {
            Self::Init => Some(Self::PsqlTrainee),
            Self::PsqlTrainee => Some(Self::QueryAdept),
            Self::QueryAdept => Some(Self::SchemaKeeper),
            Self::SchemaKeeper => Some(Self::PlanReader),
            Self::PlanReader => Some(Self::WalOperator),
            Self::WalOperator => Some(Self::PostgresGuardian),
            Self::PostgresGuardian => Some(Self::MadNinjaDba),
            Self::MadNinjaDba => None,
        }
    }

    pub fn from_xp(xp: usize) -> Self {
        Self::ALL
            .into_iter()
            .rev()
            .find(|rank| xp >= rank.minimum_xp())
            .unwrap_or(Self::Init)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AchievementRarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AchievementKind {
    FirstAnswer,
    HundredAnswers,
    PerfectTwenty,
    QuizVeteran,
    FirstSql,
    OneShot,
    QueryNinja,
    SevenDayStreak,
    ThirtyDayStreak,
    CourseMaster,
    MadNinja,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AchievementProgress {
    pub kind: AchievementKind,
    pub rarity: AchievementRarity,
    pub unlocked: bool,
    pub current: usize,
    pub target: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionMistakeSummary {
    pub question_id: QuestionId,
    pub topic_id: TopicId,
    pub course_code: String,
    pub topic_title: String,
    pub question_text: String,
    pub total_attempts: usize,
    pub incorrect_attempts: usize,
    pub last_incorrect_at: String,
}

impl QuestionMistakeSummary {
    pub fn error_rate_percent(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }

        (self.incorrect_attempts as f64 / self.total_attempts as f64) * 100.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionAttemptHistory {
    pub answered_at: String,
    pub is_correct: bool,
    pub selected_answers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionMistakeDetail {
    pub summary: QuestionMistakeSummary,
    pub question: Question,
    pub correct_answers: Vec<String>,
    pub attempts: Vec<QuestionAttemptHistory>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DailyActivity {
    pub date: String,
    pub open_count: usize,
    pub answered_questions: usize,
    pub correct_answers: usize,
    pub completed_sessions: usize,
    pub practice_attempts: usize,
    pub practice_solved: usize,
}

impl DailyActivity {
    pub fn has_learning_activity(&self) -> bool {
        self.answered_questions > 0
            || self.completed_sessions > 0
            || self.practice_attempts > 0
            || self.practice_solved > 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CourseProgress {
    pub course_code: String,
    pub passed_topics: usize,
    pub total_topics: usize,
}

impl CourseProgress {
    pub fn completion_percent(&self) -> f64 {
        if self.total_topics == 0 {
            return 0.0;
        }

        (self.passed_topics as f64 / self.total_topics as f64) * 100.0
    }

    pub fn completed(&self) -> bool {
        self.total_topics > 0 && self.passed_topics == self.total_topics
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ActivityStats {
    pub active_days: usize,
    pub study_days: usize,
    pub current_streak: usize,
    pub best_streak: usize,
    pub total_answered_questions: usize,
    pub today_answered_questions: usize,
    pub today_correct_answers: usize,
    pub last_7_days_answered_questions: usize,
    pub xp: usize,
    pub last_activity_date: Option<String>,
    pub days_since_last_activity: Option<usize>,
    pub recent_days: Vec<DailyActivity>,
    pub course_progress: Vec<CourseProgress>,
    pub achievements: Vec<AchievementProgress>,
    pub xp_opportunity_summary: XpOpportunitySummary,
    pub question_xp_available_ids: Vec<QuestionId>,
    pub quiz_xp_opportunities: Vec<QuizXpOpportunity>,
    pub practice_xp_opportunities: Vec<PracticeXpOpportunity>,
}

impl ActivityStats {
    pub fn total_topics(&self) -> usize {
        self.course_progress
            .iter()
            .map(|progress| progress.total_topics)
            .sum()
    }

    pub fn passed_topics(&self) -> usize {
        self.course_progress
            .iter()
            .map(|progress| progress.passed_topics)
            .sum()
    }

    pub fn overall_mastery_percent(&self) -> f64 {
        let total_topics = self.total_topics();

        if total_topics == 0 {
            return 0.0;
        }

        (self.passed_topics() as f64 / total_topics as f64) * 100.0
    }

    pub fn rank(&self) -> ProgressRank {
        ProgressRank::from_xp(self.xp)
    }

    pub fn next_rank(&self) -> Option<ProgressRank> {
        self.rank().next()
    }

    pub fn question_xp_available(&self, question_id: QuestionId) -> bool {
        self.question_xp_available_ids.contains(&question_id)
    }

    pub fn quiz_xp_opportunity(&self, scope: &QuizScope) -> Option<&QuizXpOpportunity> {
        self.quiz_xp_opportunities
            .iter()
            .find(|opportunity| opportunity.matches_scope(scope))
    }

    pub fn practice_xp_opportunity(&self, task_id: i64) -> Option<&PracticeXpOpportunity> {
        self.practice_xp_opportunities
            .iter()
            .find(|opportunity| opportunity.task_id == task_id)
    }

    pub fn rank_progress_percent(&self) -> f64 {
        let rank = self.rank();
        let Some(next) = rank.next() else {
            return 100.0;
        };

        let floor = rank.minimum_xp();
        let span = next.minimum_xp().saturating_sub(floor);
        if span == 0 {
            return 100.0;
        }

        let earned = self.xp.saturating_sub(floor).min(span);
        earned as f64 / span as f64 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn course(course_code: &str, passed_topics: usize, total_topics: usize) -> CourseProgress {
        CourseProgress {
            course_code: course_code.to_string(),
            passed_topics,
            total_topics,
        }
    }

    #[test]
    fn error_rate_uses_all_attempts() {
        let summary = QuestionMistakeSummary {
            question_id: QuestionId(1),
            topic_id: TopicId(1),
            course_code: String::from("dba-1"),
            topic_title: String::from("Test"),
            question_text: String::from("Question"),
            total_attempts: 4,
            incorrect_attempts: 3,
            last_incorrect_at: String::from("2026-10-19 12:00:00"),
        };

        assert_eq!(summary.error_rate_percent(), 75.0);
    }

    #[test]
    fn rank_is_driven_only_by_accumulated_xp() {
        assert_eq!(ProgressRank::from_xp(0), ProgressRank::Init);
        assert_eq!(ProgressRank::from_xp(99), ProgressRank::Init);
        assert_eq!(ProgressRank::from_xp(100), ProgressRank::PsqlTrainee);
        assert_eq!(ProgressRank::from_xp(1_399), ProgressRank::SchemaKeeper);
        assert_eq!(ProgressRank::from_xp(6_000), ProgressRank::MadNinjaDba);
    }

    #[test]
    fn rank_progress_is_measured_inside_current_rank() {
        let stats = ActivityStats {
            xp: 200,
            ..ActivityStats::default()
        };

        assert_eq!(stats.rank(), ProgressRank::PsqlTrainee);
        assert_eq!(stats.rank_progress_percent(), 50.0);
    }

    #[test]
    fn xp_opportunity_summary_adds_distinct_reward_sources() {
        let summary = XpOpportunitySummary {
            question_xp: 10,
            quiz_xp: 20,
            mastery_xp: 50,
            course_xp: 200,
            practice_xp: 35,
        };

        assert_eq!(summary.total_available_xp(), 315);
    }

    #[test]
    fn overall_mastery_is_based_on_passed_topics() {
        let stats = ActivityStats {
            course_progress: vec![course("dba-1", 3, 4), course("rust-1", 1, 4)],
            ..ActivityStats::default()
        };

        assert_eq!(stats.passed_topics(), 4);
        assert_eq!(stats.total_topics(), 8);
        assert_eq!(stats.overall_mastery_percent(), 50.0);
    }
}
