use dba_trainer_domain::{
    PracticeDifficulty, ProgressLevel, QuestionLimit, StatisticsLimit, TopicMasteryStats,
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
    pub resume_found: &'static str,
    pub answered: &'static str,
    pub next_question: &'static str,
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
    pub career_path: &'static str,
    pub salary_disclaimer: &'static str,
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

    pub fn progress_level(&self, level: ProgressLevel) -> &'static str {
        match level {
            ProgressLevel::ColdStart => "Cold Start",
            ProgressLevel::QueryRunner => "Query Runner",
            ProgressLevel::WarmCache => "Warm Cache",
            ProgressLevel::BackendTamer => "Backend Tamer",
            ProgressLevel::WalKeeper => "WAL Keeper",
            ProgressLevel::IncidentHandler => "Incident Handler",
            ProgressLevel::DbaOneMaster => "DBA-1 Master",
            ProgressLevel::ProductionSurvivor => "Production Survivor",
        }
    }

    pub fn progress_message(&self, level: ProgressLevel) -> &'static str {
        if self.main_menu == "Main menu" {
            match level {
                ProgressLevel::ColdStart => {
                    "Warm-up is over. Ten questions are enough to make today count."
                }
                ProgressLevel::QueryRunner => {
                    "Queries are moving. Keep knowledge from turning into a cache miss."
                }
                ProgressLevel::WarmCache => {
                    "Cache is warm. Discipline is already doing more than random motivation."
                }
                ProgressLevel::BackendTamer => {
                    "Backend tamed. Weak spots are visible now; finish them deliberately."
                }
                ProgressLevel::WalKeeper => {
                    "WAL is under watch. Keep going without magic fixes or chmod 777."
                }
                ProgressLevel::IncidentHandler => {
                    "Incidents look less mystical now: observe first, then fix."
                }
                ProgressLevel::DbaOneMaster => {
                    "DBA-1 completed. Now you can at least tell what exactly broke."
                }
                ProgressLevel::ProductionSurvivor => {
                    "Production Survivor. Everything currently loaded is mastered; add the next level."
                }
            }
        } else {
            match level {
                ProgressLevel::ColdStart => {
                    "Разогрев закончен. Десять вопросов — и день уже не потерян."
                }
                ProgressLevel::QueryRunner => {
                    "Запросы пошли. Главное — не превращать знания в cache miss."
                }
                ProgressLevel::WarmCache => {
                    "Кэш прогрет. Дисциплина уже работает лучше случайной мотивации."
                }
                ProgressLevel::BackendTamer => {
                    "Backend приручён. Слабые места видно — добивай их осознанно."
                }
                ProgressLevel::WalKeeper => {
                    "WAL под присмотром. Продолжай без магии и без chmod 777."
                }
                ProgressLevel::IncidentHandler => {
                    "Инциденты уже не выглядят мистикой: сначала наблюдаем, потом чиним."
                }
                ProgressLevel::DbaOneMaster => {
                    "DBA-1 закрыт. Теперь ты хотя бы знаешь, что именно сломалось."
                }
                ProgressLevel::ProductionSurvivor => {
                    "Production Survivor. Всё загруженное освоено — пора добавлять следующий уровень."
                }
            }
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

    resume_title: "Unfinished quiz",
    resume_found: "An unfinished quiz was found.",
    answered: "Answered",
    next_question: "Next question",
    continue_quiz: "Continue quiz",
    cancel_quiz: "Cancel quiz",
    action: "Action",

    cancel_title: "Cancel quiz",
    cancel_warning: "A cancelled quiz cannot be resumed.\nIts answers remain in the database, but are not included in the main accuracy statistics.",

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
    career_path: "Career path (game)",
    salary_disclaimer: "Skills are counted. Salary is not guaranteed.",
    course_progress: "Course progress",
    recent_activity: "Recent 28 days",
    study_day_target: "A study day requires at least 10 answered questions.",

    all_modes: "All modes",
    twenty_questions: "20 questions",
    fifty_questions: "50 questions",
    all_questions: "All questions",

    error: "Error",

    footer_menu: "↑/↓ or j/k — select | Enter — open | q — quit",
    footer_topics: "↑/↓ or j/k — select | Enter — open | Esc/Backspace — back | q — quit",
    footer_courses: "↑/↓ or j/k — select | Enter — open | Esc/Backspace — back | q — quit",
    footer_setup: "↑/↓ or j/k — select | Enter — start | Esc/Backspace — back | q — quit",
    footer_quiz_single: "↑/↓ or j/k — select | Enter — answer | Esc/Backspace — cancel | q — quit",
    footer_quiz_multiple: "↑/↓ or j/k — select | Space — toggle | Enter — answer | Esc/Backspace — cancel | q — quit",
    footer_feedback: "↑/↓ — scroll | Enter — next question | Esc/Backspace — cancel | q — quit",
    footer_resume: "↑/↓ — select | Enter — confirm | q — quit",
    footer_cancel: "↑/↓ — select | Enter — confirm | Esc/Backspace — back | q — quit",
    footer_result: "Enter or Esc/Backspace — main menu | q — quit",
    footer_statistics: "↑/↓ — select | Enter — open | Esc/Backspace — back | c — clear all statistics | q — quit",
    footer_statistics_detail: "Enter, Esc or Backspace — back | c — clear all statistics | q — quit",
    footer_statistics_scroll: "↑/↓ or j/k — scroll | Esc/Backspace — back | c — clear all statistics | q — quit",
    footer_labs: "↑/↓ or j/k — select | Enter — open | Esc/Backspace — back | q — quit",
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

    resume_title: "Незавершённый тест",
    resume_found: "Обнаружен незавершённый тест.",
    answered: "Отвечено",
    next_question: "Следующий вопрос",
    continue_quiz: "Продолжить тест",
    cancel_quiz: "Отменить тест",
    action: "Действие",

    cancel_title: "Отмена теста",
    cancel_warning: "Отменённый тест нельзя будет продолжить.\nЕго ответы сохранятся в базе, но не будут учитываться в основной статистике.",

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
    active_days: "Дней с запуском",
    study_days: "Учебных дней",
    current_streak: "Текущая серия",
    best_streak: "Лучшая серия",
    today: "Сегодня",
    last_7_days: "За 7 дней",
    rank: "Ранг",
    career_path: "Путь к 500k ₽ (игровой)",
    salary_disclaimer: "Навыки считаем. Зарплату не гарантируем.",
    course_progress: "Прогресс по курсам",
    recent_activity: "Последние 28 дней",
    study_day_target: "Учебный день — минимум 10 отвеченных вопросов.",

    all_modes: "Все режимы",
    twenty_questions: "20 вопросов",
    fifty_questions: "50 вопросов",
    all_questions: "Все вопросы",

    error: "Ошибка",

    footer_menu: "↑/↓ или j/k — выбор | Enter — открыть | q — выход",
    footer_topics: "↑/↓ или j/k — выбор | Enter — открыть | Esc/Backspace — назад | q — выход",
    footer_courses: "↑/↓ или j/k — выбор | Enter — открыть | Esc/Backspace — назад | q — выход",
    footer_setup: "↑/↓ или j/k — выбор | Enter — начать | Esc/Backspace — назад | q — выход",
    footer_quiz_single: "↑/↓ или j/k — выбор | Enter — ответить | Esc/Backspace — отмена | q — выход",
    footer_quiz_multiple: "↑/↓ или j/k — выбор | Space — отметить | Enter — ответить | Esc/Backspace — отмена | q — выход",
    footer_feedback: "↑/↓ — прокрутка | Enter — следующий вопрос | Esc/Backspace — отмена | q — выход",
    footer_resume: "↑/↓ — выбор | Enter — подтвердить | q — выход",
    footer_cancel: "↑/↓ — выбор | Enter — подтвердить | Esc/Backspace — назад | q — выход",
    footer_result: "Enter или Esc/Backspace — главное меню | q — выход",
    footer_statistics: "↑/↓ — выбор | Enter — открыть | Esc/Backspace — назад | c — очистить всю статистику | q — выход",
    footer_statistics_detail: "Enter, Esc или Backspace — назад | c — очистить всю статистику | q — выход",
    footer_statistics_scroll: "↑/↓ или j/k — прокрутка | Esc/Backspace — назад | c — очистить всю статистику | q — выход",
    footer_labs: "↑/↓ или j/k — выбор | Enter — открыть | Esc/Backspace — назад | q — выход",
    footer_lab_detail: "↑/↓ или j/k — прокрутка | Esc/Backspace — назад | q — выход",
    footer_practice_tasks: "↑/↓ или j/k — выбор | Enter — открыть | Esc/Backspace — назад | q — выход",
    footer_practice_editor: "F2 панель (вкл. Обучение) | F3 подсказка | F4 решение | F5 выполнить | F6 сброс | F7 проверить | F8 далее | Esc/Backspace назад (Backspace удаляет в редакторе)",
    footer_back: "Esc/Backspace — назад | q — выход",
};
