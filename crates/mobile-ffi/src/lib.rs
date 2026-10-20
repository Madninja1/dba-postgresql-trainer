mod error;
mod model;
mod trainer;

pub use error::MobileError;
pub use model::{
    MobileActivityStats, MobileAnswerOption, MobileAnswerResult, MobileCourseProgress,
    MobileDailyActivity, MobileLab, MobilePracticeCategoryStats, MobilePracticeCheckResult,
    MobilePracticeColumn, MobilePracticeDifficulty, MobilePracticeExecutionResult,
    MobilePracticeKind, MobilePracticeRow, MobilePracticeRule, MobilePracticeRuleKind,
    MobilePracticeSchema, MobilePracticeStats, MobilePracticeTable, MobilePracticeTableData,
    MobilePracticeTask, MobilePracticeTaskProgress, MobileProgressLevel, MobileQuestion,
    MobileQuestionAttemptHistory, MobileQuestionLimit, MobileQuestionMistakeDetail,
    MobileQuestionMistakeSummary, MobileQuestionType, MobileQuizSession, MobileSessionProgress,
    MobileSource, MobileSourceKind, MobileStatisticsLimit, MobileTopic, MobileTopicMasteryStats,
    MobileTrainingStats,
};
pub use trainer::MobileTrainer;

uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn core_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[uniffi::export]
pub fn hello_android() -> String {
    String::from("DBA Trainer: Rust core works")
}
