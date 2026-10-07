use dba_trainer_domain::{QuestionLimit, StatisticsLimit};

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
    footer_back: "Esc/Backspace — back | q — quit",
};

static RUSSIAN: UiStrings = UiStrings {
    main_menu: "Главное меню",
    topic_quiz: "Тест по теме",
    general_quiz: "Общий тест",
    statistics: "Статистика",
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
    footer_back: "Esc/Backspace — назад | q — выход",
};
