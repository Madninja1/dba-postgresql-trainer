use crate::{Question, QuestionId, TopicId};

pub const STUDY_DAY_QUESTION_TARGET: usize = 10;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressLevel {
    ColdStart,
    QueryRunner,
    WarmCache,
    BackendTamer,
    WalKeeper,
    IncidentHandler,
    DbaOneMaster,
    ProductionSurvivor,
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
    pub recent_days: Vec<DailyActivity>,
    pub course_progress: Vec<CourseProgress>,
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

    pub fn level(&self) -> ProgressLevel {
        let total_topics = self.total_topics();
        let passed_topics = self.passed_topics();

        if total_topics > 0 && passed_topics == total_topics {
            return ProgressLevel::ProductionSurvivor;
        }

        if self
            .course_progress
            .iter()
            .any(|progress| progress.course_code == "dba-1" && progress.completed())
        {
            return ProgressLevel::DbaOneMaster;
        }

        if self
            .course_progress
            .iter()
            .any(|progress| progress.course_code == "linux-dba-1" && progress.completed())
        {
            return ProgressLevel::IncidentHandler;
        }

        if passed_topics >= 10 {
            return ProgressLevel::WalKeeper;
        }

        if passed_topics >= 5 {
            return ProgressLevel::BackendTamer;
        }

        if self.best_streak >= 3 {
            return ProgressLevel::WarmCache;
        }

        if self.total_answered_questions >= 100 {
            return ProgressLevel::QueryRunner;
        }

        ProgressLevel::ColdStart
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
    fn progress_level_rewards_course_completion() {
        let dba_master = ActivityStats {
            course_progress: vec![course("dba-1", 3, 3), course("linux-dba-1", 2, 3)],
            ..ActivityStats::default()
        };

        assert_eq!(dba_master.level(), ProgressLevel::DbaOneMaster);

        let everything_mastered = ActivityStats {
            course_progress: vec![course("dba-1", 3, 3), course("linux-dba-1", 3, 3)],
            ..ActivityStats::default()
        };

        assert_eq!(
            everything_mastered.level(),
            ProgressLevel::ProductionSurvivor
        );
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
