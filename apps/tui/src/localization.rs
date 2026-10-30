use dba_trainer_domain::{
    AchievementKind, AchievementRarity, PracticeDifficulty, ProgressRank, QuestionLimit,
    StatisticsLimit, TopicMasteryStats, XpSource,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiLanguage {
    English,
    Russian,
}

impl UiLanguage {
    pub const fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Russian => "ru",
        }
    }

    pub fn from_code(value: &str) -> Option<Self> {
        match value {
            "en" => Some(Self::English),
            "ru" => Some(Self::Russian),
            _ => None,
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::English => Self::Russian,
            Self::Russian => Self::English,
        }
    }
}

pub struct UiStrings {
    pub main_menu: &'static str,
    pub topic_quiz: &'static str,
    pub general_quiz: &'static str,
    pub statistics: &'static str,
    pub notes: &'static str,
    pub labs: &'static str,
    pub practice: &'static str,
    pub language: &'static str,
    pub quit: &'static str,
    pub english: &'static str,
    pub russian: &'static str,

    pub topics_title: &'static str,
    pub topics_empty: &'static str,
    pub courses_empty: &'static str,
    pub choose_topic_course: &'static str,
    pub choose_course: &'static str,
    pub topic: &'static str,
    pub notes_title: &'static str,
    pub notes_empty: &'static str,
    pub notes_missing: &'static str,
    pub choose_notes_course: &'static str,
    pub notes_simple: &'static str,
    pub notes_middle: &'static str,
    pub notes_strong: &'static str,
    pub labs_title: &'static str,
    pub labs_empty: &'static str,
    pub choose_lab_course: &'static str,

    pub practice_title: &'static str,
    pub practice_empty: &'static str,
    pub practice_missing: &'static str,
    pub practice_category: &'static str,
    pub practice_categories: &'static str,
    pub practice_continue: &'static str,
    pub practice_progress: &'static str,
    pub practice_solved: &'static str,
    pub practice_started: &'static str,
    pub practice_attempts: &'static str,
    pub practice_first_try: &'static str,
    pub practice_no_hints: &'static str,
    pub practice_difficulty: &'static str,
    pub practice_schema: &'static str,
    pub practice_data: &'static str,
    pub practice_learning: &'static str,
    pub practice_hints: &'static str,
    pub practice_solution: &'static str,
    pub practice_solution_locked: &'static str,
    pub practice_no_more_hints: &'static str,
    pub practice_hint_locked: &'static str,
    pub practice_last_task: &'static str,
    pub practice_failed_checks: &'static str,
    pub practice_enter_table: &'static str,
    pub sql_editor: &'static str,
    pub practice_output: &'static str,
    pub practice_check: &'static str,
    pub practice_correct: &'static str,
    pub practice_incorrect: &'static str,
    pub practice_actual_rows: &'static str,
    pub practice_expected_rows: &'static str,
    pub practice_affected_rows: &'static str,
    pub practice_no_output: &'static str,

    pub quiz: &'static str,
    pub question: &'static str,
    pub question_missing: &'static str,
    pub answers: &'static str,
    pub answer: &'static str,
    pub correct: &'static str,
    pub incorrect: &'static str,
    pub answer_missing: &'static str,
    pub option_missing: &'static str,
    pub your_answer: &'static str,
    pub correct_answer: &'static str,
    pub correct_answers: &'static str,
    pub explanation: &'static str,
    pub source: &'static str,
    pub course_material: &'static str,
    pub postgresql_docs: &'static str,
    pub select_at_least_one: &'static str,

    pub resume_title: &'static str,
    pub no_unfinished_quizzes: &'static str,
    pub save_and_exit_quiz: &'static str,
    pub stay_in_quiz: &'static str,
    pub answered: &'static str,
    pub continue_quiz: &'static str,
    pub cancel_quiz: &'static str,
    pub action: &'static str,

    pub cancel_title: &'static str,
    pub cancel_warning: &'static str,

    pub result_title: &'static str,
    pub correct_answers_count: &'static str,
    pub result: &'static str,
    pub pass_criteria: &'static str,
    pub pass_threshold: &'static str,
    pub pass_status: &'static str,
    pub passed: &'static str,
    pub not_passed: &'static str,

    pub overall: &'static str,
    pub overall_statistics: &'static str,
    pub by_courses_topics: &'static str,
    pub by_modes: &'static str,
    pub statistics_courses: &'static str,
    pub all_topics: &'static str,
    pub statistics_modes: &'static str,
    pub statistics_unavailable: &'static str,
    pub all_statistics: &'static str,
    pub clear_statistics_title: &'static str,
    pub clear_statistics_warning: &'static str,
    pub clear_statistics_confirm: &'static str,
    pub clear_statistics_cancel: &'static str,
    pub all_courses: &'static str,
    pub completed_tests: &'static str,
    pub cancelled_tests: &'static str,
    pub answered_questions: &'static str,
    pub incorrect_answers_count: &'static str,
    pub accuracy: &'static str,
    pub mistakes: &'static str,
    pub work_on_mistakes: &'static str,
    pub review_course_mistakes: &'static str,
    pub review_topic_mistakes: &'static str,
    pub no_mistakes: &'static str,
    pub last_error: &'static str,
    pub mistake_history: &'static str,
    pub activity: &'static str,
    pub active_days: &'static str,
    pub study_days: &'static str,
    pub current_streak: &'static str,
    pub best_streak: &'static str,
    pub today: &'static str,
    pub last_7_days: &'static str,
    pub rank: &'static str,
    pub xp: &'static str,
    pub next_rank: &'static str,
    pub rank_scale: &'static str,
    pub xp_rules: &'static str,
    pub xp_available: &'static str,
    pub xp_opportunities: &'static str,
    pub xp_questions: &'static str,
    pub xp_quizzes: &'static str,
    pub xp_topics: &'static str,
    pub xp_courses: &'static str,
    pub xp_practice: &'static str,
    pub xp_pass_bonus: &'static str,
    pub xp_received: &'static str,
    pub achievements: &'static str,
    pub mastery: &'static str,
    pub active_days_28: &'static str,
    pub last_activity: &'static str,
    pub no_recent_activity: &'static str,
    pub sql_attempts: &'static str,
    pub sql_solved: &'static str,
    pub course_progress: &'static str,
    pub recent_activity: &'static str,
    pub study_day_target: &'static str,

    pub all_modes: &'static str,
    pub twenty_questions: &'static str,
    pub fifty_questions: &'static str,
    pub all_questions: &'static str,

    pub error: &'static str,

    pub footer_menu: &'static str,
    pub footer_topics: &'static str,
    pub footer_courses: &'static str,
    pub footer_setup: &'static str,
    pub footer_quiz_single: &'static str,
    pub footer_quiz_multiple: &'static str,
    pub footer_feedback: &'static str,
    pub footer_resume: &'static str,
    pub footer_cancel: &'static str,
    pub footer_result: &'static str,
    pub footer_statistics: &'static str,
    pub footer_statistics_detail: &'static str,
    pub footer_statistics_scroll: &'static str,
    pub footer_labs: &'static str,
    pub footer_notes: &'static str,
    pub footer_note_detail: &'static str,
    pub footer_lab_detail: &'static str,
    pub footer_practice_tasks: &'static str,
    pub footer_practice_editor: &'static str,
    pub footer_back: &'static str,
}

impl UiStrings {
    pub fn language_name(&self, language: UiLanguage) -> &'static str {
        match language {
            UiLanguage::English => self.english,
            UiLanguage::Russian => self.russian,
        }
    }

    pub fn notes_part_label(&self, part: i64) -> String {
        match part {
            1 => self.notes_simple.to_string(),
            2 => self.notes_middle.to_string(),
            3 => self.notes_strong.to_string(),
            other => format!("Part {other}"),
        }
    }

    pub fn question_limit(&self, limit: QuestionLimit) -> &'static str {
        match limit {
            QuestionLimit::Twenty => self.twenty_questions,
            QuestionLimit::Fifty => self.fifty_questions,
            QuestionLimit::All => self.all_questions,
        }
    }

    pub fn statistics_limit(&self, limit: StatisticsLimit) -> &'static str {
        match limit {
            StatisticsLimit::Any => self.all_modes,
            StatisticsLimit::Twenty => self.twenty_questions,
            StatisticsLimit::Fifty => self.fifty_questions,
            StatisticsLimit::AllQuestions => self.all_questions,
        }
    }

    pub fn practice_difficulty(&self, difficulty: PracticeDifficulty) -> &'static str {
        let english = self.main_menu == "Main menu";
        match (english, difficulty) {
            (true, PracticeDifficulty::Easy) => "Easy",
            (true, PracticeDifficulty::Medium) => "Medium",
            (true, PracticeDifficulty::Hard) => "Hard",
            (false, PracticeDifficulty::Easy) => "Лёгкая",
            (false, PracticeDifficulty::Medium) => "Средняя",
            (false, PracticeDifficulty::Hard) => "Сложная",
        }
    }

    pub fn progress_rank(&self, rank: ProgressRank) -> &'static str {
        match rank {
            ProgressRank::Init => "Init",
            ProgressRank::PsqlTrainee => "PSQL Trainee",
            ProgressRank::QueryAdept => "Query Adept",
            ProgressRank::SchemaKeeper => "Schema Keeper",
            ProgressRank::PlanReader => "Plan Reader",
            ProgressRank::WalOperator => "WAL Operator",
            ProgressRank::PostgresGuardian => "Postgres Guardian",
            ProgressRank::MadNinjaDba => "MadNinja DBA",
        }
    }

    pub fn rank_description(&self, rank: ProgressRank) -> &'static str {
        let english = self.main_menu == "Main menu";
        match (english, rank) {
            (true, ProgressRank::Init) => {
                "The journey starts here. Build the first reliable habits."
            }
            (true, ProgressRank::PsqlTrainee) => "Basic SQL and quiz practice is becoming routine.",
            (true, ProgressRank::QueryAdept) => {
                "Queries are no longer random commands; you can reason about results."
            }
            (true, ProgressRank::SchemaKeeper) => {
                "You are building a stable picture of schemas, objects and data flow."
            }
            (true, ProgressRank::PlanReader) => {
                "You are ready to reason about execution plans and query behavior."
            }
            (true, ProgressRank::WalOperator) => {
                "Operational PostgreSQL concepts are becoming part of your toolkit."
            }
            (true, ProgressRank::PostgresGuardian) => {
                "A broad part of the DBA path is under control."
            }
            (true, ProgressRank::MadNinjaDba) => {
                "Top rank: persistent practice across the whole trainer."
            }
            (false, ProgressRank::Init) => "Старт обучения: формируем первые устойчивые привычки.",
            (false, ProgressRank::PsqlTrainee) => {
                "Базовые тесты и SQL-практика уже становятся рутиной."
            }
            (false, ProgressRank::QueryAdept) => {
                "Запросы уже не набор команд: ты понимаешь, почему получается результат."
            }
            (false, ProgressRank::SchemaKeeper) => {
                "Формируется целостное понимание схем, объектов и движения данных."
            }
            (false, ProgressRank::PlanReader) => {
                "Можно переходить к осмысленному чтению планов и поведению запросов."
            }
            (false, ProgressRank::WalOperator) => {
                "Операционные механизмы PostgreSQL становятся рабочим инструментом."
            }
            (false, ProgressRank::PostgresGuardian) => "Большая часть DBA-пути уже под контролем.",
            (false, ProgressRank::MadNinjaDba) => {
                "Верхний ранг: системная практика по всему тренажёру."
            }
        }
    }

    pub fn achievement_title(&self, kind: AchievementKind) -> &'static str {
        let english = self.main_menu == "Main menu";
        match (english, kind) {
            (true, AchievementKind::FirstAnswer) => "First Blood",
            (false, AchievementKind::FirstAnswer) => "Первый ответ",
            (_, AchievementKind::HundredAnswers) => "100 Questions",
            (_, AchievementKind::PerfectTwenty) => "20/20",
            (true, AchievementKind::QuizVeteran) => "Quiz Veteran",
            (false, AchievementKind::QuizVeteran) => "Ветеран тестов",
            (_, AchievementKind::FirstSql) => "SQL Initiate",
            (_, AchievementKind::OneShot) => "One Shot",
            (_, AchievementKind::QueryNinja) => "Query Ninja",
            (_, AchievementKind::SevenDayStreak) => "Seven Days",
            (_, AchievementKind::ThirtyDayStreak) => "WAL Never Sleeps",
            (true, AchievementKind::CourseMaster) => "Course Master",
            (false, AchievementKind::CourseMaster) => "Мастер курса",
            (_, AchievementKind::MadNinja) => "MadNinja",
        }
    }

    pub fn achievement_rarity(&self, rarity: AchievementRarity) -> &'static str {
        let english = self.main_menu == "Main menu";
        match (english, rarity) {
            (true, AchievementRarity::Common) => "Common",
            (true, AchievementRarity::Rare) => "Rare",
            (true, AchievementRarity::Epic) => "Epic",
            (true, AchievementRarity::Legendary) => "Legendary",
            (false, AchievementRarity::Common) => "Обычная",
            (false, AchievementRarity::Rare) => "Редкая",
            (false, AchievementRarity::Epic) => "Эпическая",
            (false, AchievementRarity::Legendary) => "Легендарная",
        }
    }

    pub fn xp_source(&self, source: XpSource) -> &'static str {
        let english = self.main_menu == "Main menu";
        match (english, source) {
            (true, XpSource::UniqueCorrectQuestion) => "First correct answer to a question",
            (true, XpSource::PassedTwentyQuiz) => "First passed 20-question quiz for a scope",
            (true, XpSource::PassedFiftyQuiz) => "First passed 50-question quiz for a scope",
            (true, XpSource::PassedAllQuestionsQuiz) => {
                "First passed all-questions quiz for a scope"
            }
            (true, XpSource::MasteredTopic) => "Topic mastered for the first time",
            (true, XpSource::CompletedCourse) => "Course completed for the first time",
            (true, XpSource::PracticeEasy) => "Solved Easy SQL task",
            (true, XpSource::PracticeMedium) => "Solved Medium SQL task",
            (true, XpSource::PracticeHard) => "Solved Hard SQL task",
            (true, XpSource::PracticeFirstTry) => "SQL task solved on first check",
            (true, XpSource::PracticeNoHints) => "SQL task solved without hints",
            (false, XpSource::UniqueCorrectQuestion) => "Первый правильный ответ на вопрос",
            (false, XpSource::PassedTwentyQuiz) => "Первое прохождение 20 вопросов для раздела",
            (false, XpSource::PassedFiftyQuiz) => "Первое прохождение 50 вопросов для раздела",
            (false, XpSource::PassedAllQuestionsQuiz) => {
                "Первое прохождение всех вопросов для раздела"
            }
            (false, XpSource::MasteredTopic) => "Тема впервые освоена",
            (false, XpSource::CompletedCourse) => "Курс впервые закрыт",
            (false, XpSource::PracticeEasy) => "Решена лёгкая SQL-задача",
            (false, XpSource::PracticeMedium) => "Решена средняя SQL-задача",
            (false, XpSource::PracticeHard) => "Решена сложная SQL-задача",
            (false, XpSource::PracticeFirstTry) => "SQL-задача решена с первой проверки",
            (false, XpSource::PracticeNoHints) => "SQL-задача решена без подсказок",
        }
    }

    pub fn topic_mastery_summary(&self, stats: Option<&TopicMasteryStats>) -> String {
        let Some(stats) = stats else {
            return String::new();
        };

        let targets = format!(
            "{:.0}/{:.0}/{:.0}%",
            QuestionLimit::Twenty.pass_threshold_percent(),
            QuestionLimit::Fifty.pass_threshold_percent(),
            QuestionLimit::All.pass_threshold_percent(),
        );

        if stats.completed_attempts == 0 {
            return if self.main_menu == "Main menu" {
                format!("no attempts · targets 20/50/all: {targets}")
            } else {
                format!("нет попыток · цели 20/50/все: {targets}")
            };
        }

        let best = stats.best_accuracy_percent();

        match stats.attempts_to_pass {
            Some(attempt) if self.main_menu == "Main menu" => {
                format!("best {best:.1}% · passed on attempt #{attempt}")
            }
            Some(attempt) => {
                format!("лучший {best:.1}% · пройдено с попытки №{attempt}")
            }
            None if self.main_menu == "Main menu" => format!(
                "best {best:.1}% · {} attempts · targets {targets}",
                stats.completed_attempts
            ),
            None => format!(
                "лучший {best:.1}% · попыток {} · цели {targets}",
                stats.completed_attempts
            ),
        }
    }
}

pub fn ui_strings(language: UiLanguage) -> &'static UiStrings {
    match language {
        UiLanguage::English => &ENGLISH,
        UiLanguage::Russian => &RUSSIAN,
    }
}

static ENGLISH: UiStrings = UiStrings {
    main_menu: "Main menu",
    topic_quiz: "Quiz by topic",
    general_quiz: "General quiz",
    statistics: "Statistics",
    notes: "Notes",
    labs: "Labs",
    practice: "SQL practice",
    language: "Language",
    quit: "Quit",
    english: "English",
    russian: "Russian",

    topics_title: "Topics",
    topics_empty: "No topics are loaded yet.",
    courses_empty: "No course blocks are loaded yet.",
    choose_topic_course: "Quiz by topic: choose a course",
    choose_course: "General quiz: choose a course block",
    topic: "Topic",
    notes_title: "Notes",
    notes_empty: "No note topics are loaded yet.",
    notes_missing: "The note for this topic has not been added yet.",
    choose_notes_course: "Notes: choose a course",
    notes_simple: "Initial",
    notes_middle: "Middle",
    notes_strong: "Strong",
    labs_title: "Labs",
    labs_empty: "No labs are loaded yet.",
    choose_lab_course: "Labs: choose a course",

    practice_title: "SQL practice",
    practice_empty: "No SQL practice tasks are loaded yet.",
    practice_missing: "Practice task is not loaded.",
    practice_category: "Category",
    practice_categories: "Practice categories",
    practice_continue: "Continue last task",
    practice_progress: "Progress",
    practice_solved: "Solved",
    practice_started: "Started",
    practice_attempts: "Attempts",
    practice_first_try: "First try",
    practice_no_hints: "No hints",
    practice_difficulty: "Difficulty",
    practice_schema: "Schema",
    practice_data: "Table data",
    practice_learning: "Learning",
    practice_hints: "Hints",
    practice_solution: "Solution",
    practice_solution_locked: "Solution unlocks after 3 attempts or after all hints are opened.",
    practice_no_more_hints: "No more hints are available.",
    practice_hint_locked: "The next hint unlocks after another failed attempt.",
    practice_last_task: "This is the last task in the category.",
    practice_failed_checks: "Failed checks",
    practice_enter_table: "Enter — inspect selected table",
    sql_editor: "SQL editor",
    practice_output: "Result",
    practice_check: "Check",
    practice_correct: "Correct",
    practice_incorrect: "Not correct yet",
    practice_actual_rows: "Rows",
    practice_expected_rows: "Expected rows",
    practice_affected_rows: "Affected rows",
    practice_no_output: "Press F5 to run the current SQL.",

    quiz: "Quiz",
    question: "Question",
    question_missing: "Question is not loaded.",
    answers: "Answers",
    answer: "Answer",
    correct: "Correct",
    incorrect: "Incorrect",
    answer_missing: "Answer result is unavailable.",
    option_missing: "• option not found",
    your_answer: "Your answer",
    correct_answer: "Correct answer",
    correct_answers: "Correct answers",
    explanation: "Explanation",
    source: "Source",
    course_material: "Course material",
    postgresql_docs: "PostgreSQL documentation",
    select_at_least_one: "Select at least one answer option.",

    resume_title: "Unfinished quizzes",
    no_unfinished_quizzes: "There are no unfinished quizzes.",
    save_and_exit_quiz: "Save and exit",
    stay_in_quiz: "Stay in quiz",
    answered: "Answered",
    continue_quiz: "Continue quiz",
    cancel_quiz: "Interrupt quiz",
    action: "Action",

    cancel_title: "Leave quiz?",
    cancel_warning: "Save and exit keeps this attempt available for later. Interrupt marks it as cancelled. Staying returns to the quiz.",

    result_title: "Quiz result",
    correct_answers_count: "Correct answers",
    result: "Result",
    pass_criteria: "Pass criteria",
    pass_threshold: "Pass threshold",
    pass_status: "Status",
    passed: "Passed",
    not_passed: "Not passed",

    overall: "Overall",
    overall_statistics: "Overall statistics",
    by_courses_topics: "By courses and topics",
    by_modes: "By modes",
    statistics_courses: "Statistics → Courses",
    all_topics: "All topics",
    statistics_modes: "Statistics → Modes",
    statistics_unavailable: "Statistics are not loaded.",
    all_statistics: "all statistics",
    clear_statistics_title: "Clear statistics?",
    clear_statistics_warning: "Statistics for {scope} will be permanently deleted, including unfinished quizzes in this scope.",
    clear_statistics_confirm: "Clear",
    clear_statistics_cancel: "Cancel",
    all_courses: "All courses",
    completed_tests: "Completed quizzes",
    cancelled_tests: "Cancelled quizzes",
    answered_questions: "Answered questions",
    incorrect_answers_count: "Incorrect answers",
    accuracy: "Accuracy",
    mistakes: "Mistakes",
    work_on_mistakes: "Review all mistakes",
    review_course_mistakes: "Review this course section",
    review_topic_mistakes: "Review this topic",
    no_mistakes: "No mistakes have been recorded yet.",
    last_error: "Last error",
    mistake_history: "Attempt history",
    activity: "Activity and progress",
    active_days: "Active days",
    study_days: "Study days",
    current_streak: "Current streak",
    best_streak: "Best streak",
    today: "Today",
    last_7_days: "Last 7 days",
    rank: "Rank",
    xp: "XP",
    next_rank: "To next rank",
    rank_scale: "Rank scale",
    xp_rules: "How XP is earned",
    xp_available: "Available XP",
    xp_opportunities: "Where to earn XP",
    xp_questions: "New correct answers",
    xp_quizzes: "Quiz completion bonuses",
    xp_topics: "Topic mastery",
    xp_courses: "Course completion",
    xp_practice: "SQL Practice",
    xp_pass_bonus: "pass bonus",
    xp_received: "XP received",
    achievements: "Achievements",
    mastery: "Mastery",
    active_days_28: "Active days in the last 28 days",
    last_activity: "Last activity",
    no_recent_activity: "No learning activity in the last 28 days.",
    sql_attempts: "SQL attempts",
    sql_solved: "SQL tasks solved",
    course_progress: "Course progress",
    recent_activity: "Activity in the last 28 days",
    study_day_target: "A study day is 10 answered questions or at least one solved SQL task.",

    all_modes: "All modes",
    twenty_questions: "20 questions",
    fifty_questions: "50 questions",
    all_questions: "All questions",

    error: "Error",

    footer_menu: "↑/↓ or j/k — select | Enter — open | q — quit",
    footer_topics: "↑/↓ or j/k — select | Enter — open | Esc/Backspace — back | q — quit",
    footer_courses: "↑/↓ or j/k — select | Enter — open | Esc/Backspace — back | q — quit",
    footer_setup: "↑/↓ or j/k — select | Enter — start | Esc/Backspace — back | q — quit",
    footer_quiz_single: "↑/↓ or j/k — select | Enter — answer | Esc/Backspace — leave | q — quit",
    footer_quiz_multiple: "↑/↓ or j/k — select | Space — toggle | Enter — answer | Esc/Backspace — leave | q — quit",
    footer_feedback: "↑/↓ — scroll | Enter — next question | Esc/Backspace — leave | q — quit",
    footer_resume: "↑/↓ — select | Enter — continue | Esc/Backspace — back | q — quit",
    footer_cancel: "↑/↓ — select | Enter — confirm | Esc/Backspace — back | q — quit",
    footer_result: "Enter or Esc/Backspace — main menu | q — quit",
    footer_statistics: "↑/↓ — select | Enter — open | Esc/Backspace — back | c — clear current scope | q — quit",
    footer_statistics_detail: "Enter, Esc or Backspace — back | c — clear current scope | q — quit",
    footer_statistics_scroll: "↑/↓ or j/k — scroll | Esc/Backspace — back | c — clear current scope | q — quit",
    footer_labs: "↑/↓ or j/k — select | Enter — open | Esc/Backspace — back | q — quit",
    footer_notes: "↑/↓ or j/k — select | Enter — open | Esc/Backspace — back | q — quit",
    footer_note_detail: "↑/↓ or j/k — scroll | Esc/Backspace — back | q — quit",
    footer_lab_detail: "↑/↓ or j/k — scroll | Esc/Backspace — back | q — quit",
    footer_practice_tasks: "↑/↓ or j/k — select | Enter — open | Esc/Backspace — back | q — quit",
    footer_practice_editor: "F2 pane (incl. Learning) | F3 hint | F4 solution | F5 run | F6 reset | F7 check | F8 next | Esc/Backspace back (Backspace deletes in editor)",
    footer_back: "Esc/Backspace — back | q — quit",
};

static RUSSIAN: UiStrings = UiStrings {
    main_menu: "Главное меню",
    topic_quiz: "Тест по теме",
    general_quiz: "Общий тест",
    statistics: "Статистика",
    notes: "Конспекты",
    labs: "Лабораторные",
    practice: "Практика SQL",
    language: "Язык",
    quit: "Выход",
    english: "English",
    russian: "Русский",

    topics_title: "Темы",
    topics_empty: "Темы пока не загружены.",
    courses_empty: "Курсы пока не загружены.",
    choose_topic_course: "Тест по теме: выберите курс",
    choose_course: "Общий тест: выберите курс",
    topic: "Тема",
    notes_title: "Конспекты",
    notes_empty: "Темы для конспектов пока не загружены.",
    notes_missing: "Конспект для этой темы пока не добавлен.",
    choose_notes_course: "Конспекты: выберите курс",
    notes_simple: "Initial",
    notes_middle: "Middle",
    notes_strong: "Strong",
    labs_title: "Лабораторные",
    labs_empty: "Лабораторные пока не загружены.",
    choose_lab_course: "Лабораторные: выберите курс",

    practice_title: "Практика SQL",
    practice_empty: "Задания по SQL пока не загружены.",
    practice_missing: "Задание практики не загружено.",
    practice_category: "Раздел",
    practice_categories: "Разделы практики",
    practice_continue: "Продолжить последнее задание",
    practice_progress: "Прогресс",
    practice_solved: "Решено",
    practice_started: "Начато",
    practice_attempts: "Попыток",
    practice_first_try: "С первой попытки",
    practice_no_hints: "Без подсказок",
    practice_difficulty: "Сложность",
    practice_schema: "Схема",
    practice_data: "Данные таблицы",
    practice_learning: "Обучение",
    practice_hints: "Подсказки",
    practice_solution: "Решение",
    practice_solution_locked: "Решение откроется после 3 попыток или после просмотра всех подсказок.",
    practice_no_more_hints: "Больше подсказок нет.",
    practice_hint_locked: "Следующая подсказка откроется после ещё одной неудачной попытки.",
    practice_last_task: "Это последнее задание в разделе.",
    practice_failed_checks: "Не пройдены проверки",
    practice_enter_table: "Enter — открыть выбранную таблицу",
    sql_editor: "SQL редактор",
    practice_output: "Результат",
    practice_check: "Проверка",
    practice_correct: "Решение принято",
    practice_incorrect: "Пока неверно",
    practice_actual_rows: "Строк",
    practice_expected_rows: "Ожидалось строк",
    practice_affected_rows: "Изменено строк",
    practice_no_output: "Нажмите F5, чтобы выполнить текущий SQL.",

    quiz: "Тест",
    question: "Вопрос",
    question_missing: "Вопрос не загружен.",
    answers: "Ответы",
    answer: "Ответ",
    correct: "Верно",
    incorrect: "Неверно",
    answer_missing: "Результат ответа отсутствует.",
    option_missing: "• вариант не найден",
    your_answer: "Ваш ответ",
    correct_answer: "Правильный ответ",
    correct_answers: "Правильные ответы",
    explanation: "Объяснение",
    source: "Источник",
    course_material: "Материал курса",
    postgresql_docs: "Документация PostgreSQL",
    select_at_least_one: "Выберите хотя бы один вариант ответа.",

    resume_title: "Незавершённые тесты",
    no_unfinished_quizzes: "Незавершённых тестов нет.",
    save_and_exit_quiz: "Сохранить и выйти",
    stay_in_quiz: "Остаться в тесте",
    answered: "Отвечено",
    continue_quiz: "Продолжить тест",
    cancel_quiz: "Прервать тест",
    action: "Действие",

    cancel_title: "Выйти из теста?",
    cancel_warning: "«Сохранить и выйти» оставит попытку для продолжения позже. «Прервать» отменит её. Можно остаться в тесте.",

    result_title: "Результат теста",
    correct_answers_count: "Правильных ответов",
    result: "Результат",
    pass_criteria: "Критерий прохождения",
    pass_threshold: "Проходной порог",
    pass_status: "Статус",
    passed: "Пройдено",
    not_passed: "Не пройдено",

    overall: "Общая",
    overall_statistics: "Общая статистика",
    by_courses_topics: "По курсам и темам",
    by_modes: "По режимам",
    statistics_courses: "Статистика → Курсы",
    all_topics: "Все темы",
    statistics_modes: "Статистика → Режимы",
    statistics_unavailable: "Статистика не загружена.",
    all_statistics: "вся статистика",
    clear_statistics_title: "Сбросить статистику?",
    clear_statistics_warning: "Статистика для «{scope}» будет удалена безвозвратно, включая незавершённые тесты в этой области.",
    clear_statistics_confirm: "Сбросить",
    clear_statistics_cancel: "Отмена",
    all_courses: "Все курсы",
    completed_tests: "Завершено тестов",
    cancelled_tests: "Отменено тестов",
    answered_questions: "Отвечено вопросов",
    incorrect_answers_count: "Неправильных ответов",
    accuracy: "Точность",
    mistakes: "Ошибки",
    work_on_mistakes: "Пройти все ошибки",
    review_course_mistakes: "Пройти ошибки раздела",
    review_topic_mistakes: "Пройти ошибки темы",
    no_mistakes: "Ошибок пока нет.",
    last_error: "Последняя ошибка",
    mistake_history: "История попыток",
    activity: "Активность и прогресс",
    active_days: "Активных дней",
    study_days: "Учебных дней",
    current_streak: "Текущая серия",
    best_streak: "Лучшая серия",
    today: "Сегодня",
    last_7_days: "За 7 дней",
    rank: "Ранг",
    xp: "XP",
    next_rank: "До следующего ранга",
    rank_scale: "Шкала рангов",
    xp_rules: "За что начисляется XP",
    xp_available: "Доступно XP",
    xp_opportunities: "Где ещё заработать XP",
    xp_questions: "Новые правильные ответы",
    xp_quizzes: "Бонусы за прохождение тестов",
    xp_topics: "Освоение тем",
    xp_courses: "Завершение курсов",
    xp_practice: "Практика SQL",
    xp_pass_bonus: "бонус за прохождение",
    xp_received: "XP получен",
    achievements: "Достижения",
    mastery: "Мастерство",
    active_days_28: "Активных дней за последние 28 дней",
    last_activity: "Последняя активность",
    no_recent_activity: "За последние 28 дней учебной активности не было.",
    sql_attempts: "Попыток SQL",
    sql_solved: "Решено SQL-задач",
    course_progress: "Прогресс по курсам",
    recent_activity: "Активность за последние 28 дней",
    study_day_target: "Учебный день — 10 отвеченных вопросов или хотя бы одна решённая SQL-задача.",

    all_modes: "Все режимы",
    twenty_questions: "20 вопросов",
    fifty_questions: "50 вопросов",
    all_questions: "Все вопросы",

    error: "Ошибка",

    footer_menu: "↑/↓ или j/k — выбор | Enter — открыть | q — выход",
    footer_topics: "↑/↓ или j/k — выбор | Enter — открыть | Esc/Backspace — назад | q — выход",
    footer_courses: "↑/↓ или j/k — выбор | Enter — открыть | Esc/Backspace — назад | q — выход",
    footer_setup: "↑/↓ или j/k — выбор | Enter — начать | Esc/Backspace — назад | q — выход",
    footer_quiz_single: "↑/↓ или j/k — выбор | Enter — ответить | Esc/Backspace — выйти | q — выход",
    footer_quiz_multiple: "↑/↓ или j/k — выбор | Space — отметить | Enter — ответить | Esc/Backspace — выйти | q — выход",
    footer_feedback: "↑/↓ — прокрутка | Enter — следующий вопрос | Esc/Backspace — выйти | q — выход",
    footer_resume: "↑/↓ — выбор | Enter — продолжить | Esc/Backspace — назад | q — выход",
    footer_cancel: "↑/↓ — выбор | Enter — подтвердить | Esc/Backspace — назад | q — выход",
    footer_result: "Enter или Esc/Backspace — главное меню | q — выход",
    footer_statistics: "↑/↓ — выбор | Enter — открыть | Esc/Backspace — назад | c — сброс текущей области | q — выход",
    footer_statistics_detail: "Enter, Esc или Backspace — назад | c — сброс текущей области | q — выход",
    footer_statistics_scroll: "↑/↓ или j/k — прокрутка | Esc/Backspace — назад | c — сброс текущей области | q — выход",
    footer_labs: "↑/↓ или j/k — выбор | Enter — открыть | Esc/Backspace — назад | q — выход",
    footer_notes: "↑/↓ или j/k — выбор | Enter — открыть | Esc/Backspace — назад | q — выход",
    footer_note_detail: "↑/↓ или j/k — прокрутка | Esc/Backspace — назад | q — выход",
    footer_lab_detail: "↑/↓ или j/k — прокрутка | Esc/Backspace — назад | q — выход",
    footer_practice_tasks: "↑/↓ или j/k — выбор | Enter — открыть | Esc/Backspace — назад | q — выход",
    footer_practice_editor: "F2 панель (вкл. Обучение) | F3 подсказка | F4 решение | F5 выполнить | F6 сброс | F7 проверить | F8 далее | Esc/Backspace назад (Backspace удаляет в редакторе)",
    footer_back: "Esc/Backspace — назад | q — выход",
};
