package dev.dbatrainer.app

enum class UiLanguage(val code: String) {
    English("en"),
    Russian("ru"),
    ;

    companion object {
        fun fromCode(code: String?): UiLanguage =
            entries.firstOrNull { it.code == code } ?: English
    }
}

data class UiStrings(
    val appSubtitle: String,
    val topicQuiz: String,
    val generalQuiz: String,
    val statistics: String,
    val language: String,
    val topics: String,
    val topicsEmpty: String,
    val courses: String,
    val coursesEmpty: String,
    val chooseCourse: String,
    val questionCount: String,
    val twentyQuestions: String,
    val fiftyQuestions: String,
    val allQuestions: String,
    val chooseAllCorrect: String,
    val chooseOne: String,
    val answer: String,
    val correct: String,
    val incorrect: String,
    val explanation: String,
    val source: String,
    val nextQuestion: String,
    val yourAnswer: String,
    val correctAnswer: String,
    val correctAnswers: String,
    val quizCompleted: String,
    val home: String,
    val unfinishedQuiz: String,
    val continueQuiz: String,
    val cancelQuiz: String,
    val cancelQuizTitle: String,
    val cancelQuizMessage: String,
    val error: String,
    val selectAtLeastOne: String,
    val overall: String,
    val overallStatistics: String,
    val allCoursesTopicsModes: String,
    val byCoursesAndTopics: String,
    val courseTopicMode: String,
    val byModes: String,
    val modesDescription: String,
    val clearStatistics: String,
    val statisticsCourses: String,
    val courseStatistics: String,
    val allTopics: String,
    val statisticsModes: String,
    val statisticsUnavailable: String,
    val allModes: String,
    val completedTests: String,
    val cancelledTests: String,
    val answeredQuestions: String,
    val correctAnswersCount: String,
    val incorrectAnswersCount: String,
    val accuracy: String,
    val back: String,
    val courseMaterial: String,
    val postgresqlDocumentation: String,
    val rustCore: String,
) {
    fun questionProgress(current: Int, total: Int): String =
        if (this === ENGLISH_STRINGS) {
            "Question $current of $total"
        } else {
            "Вопрос $current из $total"
        }

    fun resumeMessage(answered: ULong, total: ULong): String =
        if (this === ENGLISH_STRINGS) {
            "Answered $answered of $total questions.\n\nContinue from the saved position?"
        } else {
            "Отвечено $answered из $total вопросов.\n\nПродолжить с сохранённого места?"
        }

    fun topicCount(value: Int): String =
        if (this === ENGLISH_STRINGS) {
            "$value topics"
        } else {
            "Тем: $value"
        }

    fun resultScore(correct: Int, total: Int): String =
        if (this === ENGLISH_STRINGS) {
            "$correct of $total correct"
        } else {
            "$correct из $total правильных"
        }

    fun percentCorrect(value: String): String =
        if (this === ENGLISH_STRINGS) {
            "$value correct"
        } else {
            "$value правильных ответов"
        }

    fun completedTests(value: ULong): String = "${completedTests}: $value"
    fun cancelledTests(value: ULong): String = "${cancelledTests}: $value"
    fun answeredQuestions(value: ULong): String = "${answeredQuestions}: $value"
    fun correctAnswers(value: ULong): String = "${correctAnswersCount}: $value"
    fun incorrectAnswers(value: ULong): String = "${incorrectAnswersCount}: $value"
    fun accuracy(value: String): String = "${accuracy}: $value"
}

private val ENGLISH_STRINGS = UiStrings(
    appSubtitle = "Offline technical trainer for databases and programming",
    topicQuiz = "Quiz by topic",
    generalQuiz = "General quiz",
    statistics = "Statistics",
    language = "Language",
    topics = "Topics",
    topicsEmpty = "No topics are loaded yet.",
    courses = "Courses",
    coursesEmpty = "No courses are loaded yet.",
    chooseCourse = "Choose a course block",
    questionCount = "Number of questions",
    twentyQuestions = "20 questions",
    fiftyQuestions = "50 questions",
    allQuestions = "All questions",
    chooseAllCorrect = "Select all correct options",
    chooseOne = "Select one option",
    answer = "Answer",
    correct = "Correct",
    incorrect = "Incorrect",
    explanation = "Explanation",
    source = "Source",
    nextQuestion = "Next question",
    yourAnswer = "Your answer",
    correctAnswer = "Correct answer",
    correctAnswers = "Correct answers",
    quizCompleted = "Quiz completed",
    home = "Home",
    unfinishedQuiz = "Unfinished quiz",
    continueQuiz = "Continue",
    cancelQuiz = "Cancel quiz",
    cancelQuizTitle = "Cancel quiz?",
    cancelQuizMessage = "This quiz will remain in history as cancelled. Its answers will not be included in the main accuracy statistics.",
    error = "Error",
    selectAtLeastOne = "Select at least one answer option.",
    overall = "Overall",
    overallStatistics = "Overall statistics",
    allCoursesTopicsModes = "All courses, topics and modes",
    byCoursesAndTopics = "By courses and topics",
    courseTopicMode = "course → topic → mode",
    byModes = "By modes",
    modesDescription = "20 / 50 / all questions",
    clearStatistics = "Clear statistics",
    statisticsCourses = "Statistics → Courses",
    courseStatistics = "Course statistics",
    allTopics = "All topics",
    statisticsModes = "Statistics → Modes",
    statisticsUnavailable = "Statistics are not available yet.",
    allModes = "All modes",
    completedTests = "Completed quizzes",
    cancelledTests = "Cancelled quizzes",
    answeredQuestions = "Answered questions",
    correctAnswersCount = "Correct answers",
    incorrectAnswersCount = "Incorrect answers",
    accuracy = "Accuracy",
    back = "Back",
    courseMaterial = "Course material",
    postgresqlDocumentation = "PostgreSQL documentation",
    rustCore = "Rust core",
)

private val RUSSIAN_STRINGS = UiStrings(
    appSubtitle = "Офлайн-тренажёр по базам данных и программированию",
    topicQuiz = "Тест по теме",
    generalQuiz = "Общий тест",
    statistics = "Статистика",
    language = "Язык",
    topics = "Темы",
    topicsEmpty = "Темы пока не загружены.",
    courses = "Курсы",
    coursesEmpty = "Курсы пока не загружены.",
    chooseCourse = "Выберите блок курса",
    questionCount = "Количество вопросов",
    twentyQuestions = "20 вопросов",
    fiftyQuestions = "50 вопросов",
    allQuestions = "Все вопросы",
    chooseAllCorrect = "Выберите все правильные варианты",
    chooseOne = "Выберите один вариант",
    answer = "Ответить",
    correct = "Верно",
    incorrect = "Неверно",
    explanation = "Объяснение",
    source = "Источник",
    nextQuestion = "Следующий вопрос",
    yourAnswer = "Ваш ответ",
    correctAnswer = "Правильный ответ",
    correctAnswers = "Правильные ответы",
    quizCompleted = "Тест завершён",
    home = "На главную",
    unfinishedQuiz = "Незавершённый тест",
    continueQuiz = "Продолжить",
    cancelQuiz = "Отменить тест",
    cancelQuizTitle = "Отменить тест?",
    cancelQuizMessage = "Прогресс этого теста останется в истории как отменённый. В основную статистику ответы не попадут.",
    error = "Ошибка",
    selectAtLeastOne = "Выберите хотя бы один вариант ответа.",
    overall = "Общая",
    overallStatistics = "Общая статистика",
    allCoursesTopicsModes = "Все курсы, темы и режимы",
    byCoursesAndTopics = "По курсам и темам",
    courseTopicMode = "курс → тема → режим",
    byModes = "По режимам",
    modesDescription = "20 / 50 / все вопросы",
    clearStatistics = "Очистить статистику",
    statisticsCourses = "Статистика → Курсы",
    courseStatistics = "Статистика курса",
    allTopics = "Все темы",
    statisticsModes = "Статистика → Режимы",
    statisticsUnavailable = "Статистика пока недоступна.",
    allModes = "Все режимы",
    completedTests = "Завершено тестов",
    cancelledTests = "Отменено тестов",
    answeredQuestions = "Отвечено вопросов",
    correctAnswersCount = "Правильных ответов",
    incorrectAnswersCount = "Неправильных ответов",
    accuracy = "Точность",
    back = "Назад",
    courseMaterial = "Материал курса",
    postgresqlDocumentation = "Документация PostgreSQL",
    rustCore = "Rust core",
)

fun uiStrings(language: UiLanguage): UiStrings = when (language) {
    UiLanguage.English -> ENGLISH_STRINGS
    UiLanguage.Russian -> RUSSIAN_STRINGS
}
