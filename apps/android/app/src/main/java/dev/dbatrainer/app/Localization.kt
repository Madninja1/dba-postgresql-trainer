package dev.dbatrainer.app

import dev.dbatrainer.ffi.MobileProgressLevel
import dev.dbatrainer.ffi.MobileTopicMasteryStats

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
    val labs: String,
    val language: String,
    val topics: String,
    val topicsEmpty: String,
    val courses: String,
    val coursesEmpty: String,
    val chooseTopicCourse: String,
    val chooseCourse: String,
    val chooseLabCourse: String,
    val labsEmpty: String,
    val questionCount: String,
    val twentyQuestions: String,
    val fiftyQuestions: String,
    val allQuestions: String,
    val passCriteria: String,
    val passThreshold: String,
    val passStatus: String,
    val passed: String,
    val notPassed: String,
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
    val mistakes: String,
    val workOnMistakes: String,
    val reviewCourseMistakes: String,
    val reviewTopicMistakes: String,
    val noMistakes: String,
    val attemptHistory: String,
    val attempts: String,
    val lastError: String,
    val activity: String,
    val activeDays: String,
    val studyDays: String,
    val currentStreak: String,
    val bestStreak: String,
    val today: String,
    val last7Days: String,
    val rank: String,
    val careerPath: String,
    val salaryDisclaimer: String,
    val courseProgress: String,
    val recentActivity: String,
    val studyDayTarget: String,
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

    fun labCount(value: Int): String =
        if (this === ENGLISH_STRINGS) {
            "$value labs"
        } else {
            "Лабораторных: $value"
        }

    fun topicMasterySummary(stats: MobileTopicMasteryStats?): String {
        val targets = "85/70/60%"
        if (stats == null || stats.completedAttempts == 0UL) {
            return if (this === ENGLISH_STRINGS) {
                "No completed attempts · targets 20/50/all: $targets"
            } else {
                "Нет завершённых попыток · цели 20/50/все: $targets"
            }
        }

        val best = "%.1f%%".format(stats.bestAccuracyPercent)
        val passedAttempt = stats.attemptsToPass

        return if (this === ENGLISH_STRINGS) {
            if (passedAttempt != null) {
                "Best $best · passed on attempt #$passedAttempt"
            } else {
                "Best $best · attempts ${stats.completedAttempts} · targets $targets"
            }
        } else {
            if (passedAttempt != null) {
                "Лучший результат $best · пройдено с попытки №$passedAttempt"
            } else {
                "Лучший результат $best · попыток ${stats.completedAttempts} · цели $targets"
            }
        }
    }

    fun progressLevel(level: MobileProgressLevel): String =
        when (level) {
            MobileProgressLevel.COLD_START -> "Cold Start"
            MobileProgressLevel.QUERY_RUNNER -> "Query Runner"
            MobileProgressLevel.WARM_CACHE -> "Warm Cache"
            MobileProgressLevel.BACKEND_TAMER -> "Backend Tamer"
            MobileProgressLevel.WAL_KEEPER -> "WAL Keeper"
            MobileProgressLevel.INCIDENT_HANDLER -> "Incident Handler"
            MobileProgressLevel.DBA_ONE_MASTER -> "DBA-1 Master"
            MobileProgressLevel.PRODUCTION_SURVIVOR -> "Production Survivor"
        }

    fun progressMessage(level: MobileProgressLevel): String =
        if (this === ENGLISH_STRINGS) {
            when (level) {
                MobileProgressLevel.COLD_START -> "Warm-up is over. Ten questions are enough to make today count."
                MobileProgressLevel.QUERY_RUNNER -> "Queries are moving. Keep knowledge from turning into a cache miss."
                MobileProgressLevel.WARM_CACHE -> "Cache is warm. Discipline is already doing more than random motivation."
                MobileProgressLevel.BACKEND_TAMER -> "Backend tamed. Weak spots are visible now; finish them deliberately."
                MobileProgressLevel.WAL_KEEPER -> "WAL is under watch. Keep going without magic fixes or chmod 777."
                MobileProgressLevel.INCIDENT_HANDLER -> "Incidents look less mystical now: observe first, then fix."
                MobileProgressLevel.DBA_ONE_MASTER -> "DBA-1 completed. Now you can at least tell what exactly broke."
                MobileProgressLevel.PRODUCTION_SURVIVOR -> "Production Survivor. Everything currently loaded is mastered; add the next level."
            }
        } else {
            when (level) {
                MobileProgressLevel.COLD_START -> "Разогрев закончен. Десять вопросов — и день уже не потерян."
                MobileProgressLevel.QUERY_RUNNER -> "Запросы пошли. Главное — не превращать знания в cache miss."
                MobileProgressLevel.WARM_CACHE -> "Кэш прогрет. Дисциплина уже работает лучше случайной мотивации."
                MobileProgressLevel.BACKEND_TAMER -> "Backend приручён. Слабые места видно — добивай их осознанно."
                MobileProgressLevel.WAL_KEEPER -> "WAL под присмотром. Продолжай без магии и без chmod 777."
                MobileProgressLevel.INCIDENT_HANDLER -> "Инциденты уже не выглядят мистикой: сначала наблюдаем, потом чиним."
                MobileProgressLevel.DBA_ONE_MASTER -> "DBA-1 закрыт. Теперь ты хотя бы знаешь, что именно сломалось."
                MobileProgressLevel.PRODUCTION_SURVIVOR -> "Production Survivor. Всё загруженное освоено — пора добавлять следующий уровень."
            }
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
    labs = "Labs",
    language = "Language",
    topics = "Topics",
    topicsEmpty = "No topics are loaded yet.",
    courses = "Courses",
    coursesEmpty = "No courses are loaded yet.",
    chooseTopicCourse = "Choose a course for topic quiz",
    chooseCourse = "Choose a course block",
    chooseLabCourse = "Choose a course with labs",
    labsEmpty = "No labs are loaded yet.",
    questionCount = "Number of questions",
    twentyQuestions = "20 questions",
    fiftyQuestions = "50 questions",
    allQuestions = "All questions",
    passCriteria = "Pass criteria",
    passThreshold = "Pass threshold",
    passStatus = "Status",
    passed = "Passed",
    notPassed = "Not passed",
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
    mistakes = "Mistakes",
    workOnMistakes = "Review all mistakes",
    reviewCourseMistakes = "Review this course section",
    reviewTopicMistakes = "Review this topic",
    noMistakes = "No mistakes have been recorded yet.",
    attemptHistory = "Attempt history",
    attempts = "Attempts",
    lastError = "Last error",
    activity = "Activity and progress",
    activeDays = "Active days",
    studyDays = "Study days",
    currentStreak = "Current streak",
    bestStreak = "Best streak",
    today = "Today",
    last7Days = "Last 7 days",
    rank = "Rank",
    careerPath = "Career path (game)",
    salaryDisclaimer = "Skills are counted. Salary is not guaranteed.",
    courseProgress = "Course progress",
    recentActivity = "Recent 28 days",
    studyDayTarget = "A study day requires at least 10 answered questions.",
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
    labs = "Лабораторные",
    language = "Язык",
    topics = "Темы",
    topicsEmpty = "Темы пока не загружены.",
    courses = "Курсы",
    coursesEmpty = "Курсы пока не загружены.",
    chooseTopicCourse = "Выберите курс для теста по теме",
    chooseCourse = "Выберите блок курса",
    chooseLabCourse = "Выберите курс с лабораторными",
    labsEmpty = "Лабораторные пока не загружены.",
    questionCount = "Количество вопросов",
    twentyQuestions = "20 вопросов",
    fiftyQuestions = "50 вопросов",
    allQuestions = "Все вопросы",
    passCriteria = "Критерий прохождения",
    passThreshold = "Проходной порог",
    passStatus = "Статус",
    passed = "Пройдено",
    notPassed = "Не пройдено",
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
    mistakes = "Ошибки",
    workOnMistakes = "Пройти все ошибки",
    reviewCourseMistakes = "Пройти ошибки раздела",
    reviewTopicMistakes = "Пройти ошибки темы",
    noMistakes = "Ошибок пока нет.",
    attemptHistory = "История попыток",
    attempts = "Попыток",
    lastError = "Последняя ошибка",
    activity = "Активность и прогресс",
    activeDays = "Дней с запуском",
    studyDays = "Учебных дней",
    currentStreak = "Текущая серия",
    bestStreak = "Лучшая серия",
    today = "Сегодня",
    last7Days = "За 7 дней",
    rank = "Ранг",
    careerPath = "Путь к 500k ₽ (игровой)",
    salaryDisclaimer = "Навыки считаем. Зарплату не гарантируем.",
    courseProgress = "Прогресс по курсам",
    recentActivity = "Последние 28 дней",
    studyDayTarget = "Учебный день — минимум 10 отвеченных вопросов.",
    back = "Назад",
    courseMaterial = "Материал курса",
    postgresqlDocumentation = "Документация PostgreSQL",
    rustCore = "Rust core",
)

fun uiStrings(language: UiLanguage): UiStrings = when (language) {
    UiLanguage.English -> ENGLISH_STRINGS
    UiLanguage.Russian -> RUSSIAN_STRINGS
}
