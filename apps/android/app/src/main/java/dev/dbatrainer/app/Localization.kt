package dev.dbatrainer.app

import dev.dbatrainer.ffi.MobileAchievementKind
import dev.dbatrainer.ffi.MobileAchievementRarity
import dev.dbatrainer.ffi.MobilePracticeDifficulty
import dev.dbatrainer.ffi.MobileProgressRank
import dev.dbatrainer.ffi.MobileTopicMasteryStats
import dev.dbatrainer.ffi.MobileXpSource

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
    val unfinishedQuizzes: String,
    val noUnfinishedQuizzes: String,
    val continueQuiz: String,
    val saveAndExitQuiz: String,
    val stayInQuiz: String,
    val cancelQuiz: String,
    val interruptQuizMessage: String,
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
    val clearScopeStatistics: String,
    val clearStatisticsConfirmTitle: String,
    val clearStatisticsConfirmMessage: String,
    val clearStatisticsConfirm: String,
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
    val xp: String,
    val nextRank: String,
    val rankScale: String,
    val xpRules: String,
    val xpAvailable: String,
    val xpOpportunities: String,
    val xpQuestions: String,
    val xpQuizzes: String,
    val xpTopics: String,
    val xpCourses: String,
    val xpPractice: String,
    val xpPassBonus: String,
    val xpReceived: String,
    val achievements: String,
    val mastery: String,
    val activeDays28: String,
    val lastActivity: String,
    val noRecentActivity: String,
    val sqlAttempts: String,
    val sqlSolved: String,
    val courseProgress: String,
    val recentActivity: String,
    val studyDayTarget: String,
    val back: String,
    val courseMaterial: String,
    val postgresqlDocumentation: String,
    val rustCore: String,
    val practice: String,
    val practiceSubtitle: String,
    val practiceCategories: String,
    val practiceTasks: String,
    val practiceEmpty: String,
    val practiceProgress: String,
    val practiceSolved: String,
    val practiceStarted: String,
    val practiceAttempts: String,
    val practiceFirstTry: String,
    val practiceNoHints: String,
    val practiceContinue: String,
    val practiceEditor: String,
    val practiceSchema: String,
    val practiceData: String,
    val practiceResult: String,
    val practiceLearning: String,
    val practiceRun: String,
    val practiceCheck: String,
    val practiceReset: String,
    val practiceHint: String,
    val practiceSolution: String,
    val practiceNext: String,
    val practiceCorrect: String,
    val practiceIncorrect: String,
    val practiceActualRows: String,
    val practiceExpectedRows: String,
    val practiceFailedChecks: String,
    val practiceNoOutput: String,
    val practiceAffectedRows: String,
    val practiceRows: String,
    val practiceOpenTable: String,
    val practiceNoMoreHints: String,
    val practiceHintLocked: String,
    val practiceSolutionLocked: String,
    val practiceLastTask: String,
    val practiceRules: String,
    val practiceRequireKeyword: String,
    val practiceForbidKeyword: String,
    val practiceForbidSelectStar: String,
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

    fun progressRank(rank: MobileProgressRank): String =
        when (rank) {
            MobileProgressRank.INIT -> "Init"
            MobileProgressRank.PSQL_TRAINEE -> "PSQL Trainee"
            MobileProgressRank.QUERY_ADEPT -> "Query Adept"
            MobileProgressRank.SCHEMA_KEEPER -> "Schema Keeper"
            MobileProgressRank.PLAN_READER -> "Plan Reader"
            MobileProgressRank.WAL_OPERATOR -> "WAL Operator"
            MobileProgressRank.POSTGRES_GUARDIAN -> "Postgres Guardian"
            MobileProgressRank.MAD_NINJA_DBA -> "MadNinja DBA"
        }

    fun rankDescription(rank: MobileProgressRank): String =
        if (this === ENGLISH_STRINGS) {
            when (rank) {
                MobileProgressRank.INIT -> "The journey starts here. Build the first reliable habits."
                MobileProgressRank.PSQL_TRAINEE -> "Basic SQL and quiz practice is becoming routine."
                MobileProgressRank.QUERY_ADEPT -> "Queries are no longer random commands; you can reason about results."
                MobileProgressRank.SCHEMA_KEEPER -> "You are building a stable picture of schemas, objects and data flow."
                MobileProgressRank.PLAN_READER -> "You are ready to reason about execution plans and query behavior."
                MobileProgressRank.WAL_OPERATOR -> "Operational PostgreSQL concepts are becoming part of your toolkit."
                MobileProgressRank.POSTGRES_GUARDIAN -> "A broad part of the DBA path is under control."
                MobileProgressRank.MAD_NINJA_DBA -> "Top rank: persistent practice across the whole trainer."
            }
        } else {
            when (rank) {
                MobileProgressRank.INIT -> "Старт обучения: формируем первые устойчивые привычки."
                MobileProgressRank.PSQL_TRAINEE -> "Базовые тесты и SQL-практика уже становятся рутиной."
                MobileProgressRank.QUERY_ADEPT -> "Запросы уже не набор команд: ты понимаешь, почему получается результат."
                MobileProgressRank.SCHEMA_KEEPER -> "Формируется целостное понимание схем, объектов и движения данных."
                MobileProgressRank.PLAN_READER -> "Можно переходить к осмысленному чтению планов и поведению запросов."
                MobileProgressRank.WAL_OPERATOR -> "Операционные механизмы PostgreSQL становятся рабочим инструментом."
                MobileProgressRank.POSTGRES_GUARDIAN -> "Большая часть DBA-пути уже под контролем."
                MobileProgressRank.MAD_NINJA_DBA -> "Верхний ранг: системная практика по всему тренажёру."
            }
        }

    fun achievementTitle(kind: MobileAchievementKind): String =
        when (kind) {
            MobileAchievementKind.FIRST_ANSWER -> if (this === ENGLISH_STRINGS) "First Blood" else "Первый ответ"
            MobileAchievementKind.HUNDRED_ANSWERS -> "100 Questions"
            MobileAchievementKind.PERFECT_TWENTY -> "20/20"
            MobileAchievementKind.QUIZ_VETERAN -> if (this === ENGLISH_STRINGS) "Quiz Veteran" else "Ветеран тестов"
            MobileAchievementKind.FIRST_SQL -> "SQL Initiate"
            MobileAchievementKind.ONE_SHOT -> "One Shot"
            MobileAchievementKind.QUERY_NINJA -> "Query Ninja"
            MobileAchievementKind.SEVEN_DAY_STREAK -> "Seven Days"
            MobileAchievementKind.THIRTY_DAY_STREAK -> "WAL Never Sleeps"
            MobileAchievementKind.COURSE_MASTER -> if (this === ENGLISH_STRINGS) "Course Master" else "Мастер курса"
            MobileAchievementKind.MAD_NINJA -> "MadNinja"
        }

    fun achievementDescription(kind: MobileAchievementKind): String =
        if (this === ENGLISH_STRINGS) {
            when (kind) {
                MobileAchievementKind.FIRST_ANSWER -> "Answer the first quiz question."
                MobileAchievementKind.HUNDRED_ANSWERS -> "Answer 100 quiz questions."
                MobileAchievementKind.PERFECT_TWENTY -> "Finish a 20-question quiz with 20 correct answers."
                MobileAchievementKind.QUIZ_VETERAN -> "Complete 10 quizzes."
                MobileAchievementKind.FIRST_SQL -> "Solve the first SQL practice task."
                MobileAchievementKind.ONE_SHOT -> "Solve an SQL practice task on the first check."
                MobileAchievementKind.QUERY_NINJA -> "Solve 25 SQL practice tasks."
                MobileAchievementKind.SEVEN_DAY_STREAK -> "Reach a 7-day study streak."
                MobileAchievementKind.THIRTY_DAY_STREAK -> "Reach a 30-day study streak."
                MobileAchievementKind.COURSE_MASTER -> "Master every topic in one course."
                MobileAchievementKind.MAD_NINJA -> "Reach the MadNinja DBA rank."
            }
        } else {
            when (kind) {
                MobileAchievementKind.FIRST_ANSWER -> "Ответить на первый вопрос теста."
                MobileAchievementKind.HUNDRED_ANSWERS -> "Ответить на 100 вопросов тестов."
                MobileAchievementKind.PERFECT_TWENTY -> "Пройти тест из 20 вопросов без единой ошибки."
                MobileAchievementKind.QUIZ_VETERAN -> "Завершить 10 тестов."
                MobileAchievementKind.FIRST_SQL -> "Решить первое задание SQL-практики."
                MobileAchievementKind.ONE_SHOT -> "Решить SQL-задачу с первой проверки."
                MobileAchievementKind.QUERY_NINJA -> "Решить 25 SQL-задач."
                MobileAchievementKind.SEVEN_DAY_STREAK -> "Держать учебную серию 7 дней."
                MobileAchievementKind.THIRTY_DAY_STREAK -> "Держать учебную серию 30 дней."
                MobileAchievementKind.COURSE_MASTER -> "Освоить все темы одного курса."
                MobileAchievementKind.MAD_NINJA -> "Достичь ранга MadNinja DBA."
            }
        }

    fun achievementRarity(value: MobileAchievementRarity): String =
        if (this === ENGLISH_STRINGS) {
            when (value) {
                MobileAchievementRarity.COMMON -> "Common"
                MobileAchievementRarity.RARE -> "Rare"
                MobileAchievementRarity.EPIC -> "Epic"
                MobileAchievementRarity.LEGENDARY -> "Legendary"
            }
        } else {
            when (value) {
                MobileAchievementRarity.COMMON -> "Обычная"
                MobileAchievementRarity.RARE -> "Редкая"
                MobileAchievementRarity.EPIC -> "Эпическая"
                MobileAchievementRarity.LEGENDARY -> "Легендарная"
            }
        }

    fun xpSource(value: MobileXpSource): String =
        if (this === ENGLISH_STRINGS) {
            when (value) {
                MobileXpSource.UNIQUE_CORRECT_QUESTION -> "First correct answer to a question"
                MobileXpSource.PASSED_TWENTY_QUIZ -> "First passed 20-question quiz for a scope"
                MobileXpSource.PASSED_FIFTY_QUIZ -> "First passed 50-question quiz for a scope"
                MobileXpSource.PASSED_ALL_QUESTIONS_QUIZ -> "First passed all-questions quiz for a scope"
                MobileXpSource.MASTERED_TOPIC -> "Topic mastered for the first time"
                MobileXpSource.COMPLETED_COURSE -> "Course completed for the first time"
                MobileXpSource.PRACTICE_EASY -> "Solved Easy SQL task"
                MobileXpSource.PRACTICE_MEDIUM -> "Solved Medium SQL task"
                MobileXpSource.PRACTICE_HARD -> "Solved Hard SQL task"
                MobileXpSource.PRACTICE_FIRST_TRY -> "SQL task solved on first check"
                MobileXpSource.PRACTICE_NO_HINTS -> "SQL task solved without hints"
            }
        } else {
            when (value) {
                MobileXpSource.UNIQUE_CORRECT_QUESTION -> "Первый правильный ответ на вопрос"
                MobileXpSource.PASSED_TWENTY_QUIZ -> "Первое прохождение 20 вопросов для раздела"
                MobileXpSource.PASSED_FIFTY_QUIZ -> "Первое прохождение 50 вопросов для раздела"
                MobileXpSource.PASSED_ALL_QUESTIONS_QUIZ -> "Первое прохождение всех вопросов для раздела"
                MobileXpSource.MASTERED_TOPIC -> "Тема впервые освоена"
                MobileXpSource.COMPLETED_COURSE -> "Курс впервые закрыт"
                MobileXpSource.PRACTICE_EASY -> "Решена лёгкая SQL-задача"
                MobileXpSource.PRACTICE_MEDIUM -> "Решена средняя SQL-задача"
                MobileXpSource.PRACTICE_HARD -> "Решена сложная SQL-задача"
                MobileXpSource.PRACTICE_FIRST_TRY -> "SQL-задача решена с первой проверки"
                MobileXpSource.PRACTICE_NO_HINTS -> "SQL-задача решена без подсказок"
            }
        }

    fun daysAgo(days: ULong): String =
        if (this === ENGLISH_STRINGS) {
            if (days == 0UL) "today" else "$days days ago"
        } else {
            if (days == 0UL) "сегодня" else "$days дн. назад"
        }

    fun practiceDifficulty(value: MobilePracticeDifficulty): String =
        if (this === ENGLISH_STRINGS) {
            when (value) {
                MobilePracticeDifficulty.EASY -> "Easy"
                MobilePracticeDifficulty.MEDIUM -> "Medium"
                MobilePracticeDifficulty.HARD -> "Hard"
            }
        } else {
            when (value) {
                MobilePracticeDifficulty.EASY -> "Легко"
                MobilePracticeDifficulty.MEDIUM -> "Средне"
                MobilePracticeDifficulty.HARD -> "Сложно"
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
    unfinishedQuizzes = "Unfinished quizzes",
    noUnfinishedQuizzes = "There are no unfinished quizzes.",
    continueQuiz = "Continue",
    saveAndExitQuiz = "Save and exit",
    stayInQuiz = "Stay in quiz",
    cancelQuiz = "Interrupt quiz",
    interruptQuizMessage = "This unfinished attempt will be marked as cancelled and cannot be resumed. Its answers will not be included in the main accuracy statistics.",
    cancelQuizTitle = "Leave quiz?",
    cancelQuizMessage = "Save and exit keeps this attempt available for later. Interrupt marks it as cancelled. Staying returns to the quiz.",
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
    clearScopeStatistics = "Clear selected statistics",
    clearStatisticsConfirmTitle = "Clear statistics?",
    clearStatisticsConfirmMessage = "The selected statistics and unfinished quizzes in this scope will be permanently deleted.",
    clearStatisticsConfirm = "Clear",
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
    xp = "XP",
    nextRank = "To next rank",
    rankScale = "Rank scale",
    xpRules = "How XP is earned",
    xpAvailable = "Available XP",
    xpOpportunities = "Where to earn XP",
    xpQuestions = "New correct answers",
    xpQuizzes = "Quiz completion bonuses",
    xpTopics = "Topic mastery",
    xpCourses = "Course completion",
    xpPractice = "SQL Practice",
    xpPassBonus = "pass bonus",
    xpReceived = "XP received",
    achievements = "Achievements",
    mastery = "Mastery",
    activeDays28 = "Active days in the last 28 days",
    lastActivity = "Last activity",
    noRecentActivity = "No learning activity in the last 28 days.",
    sqlAttempts = "SQL attempts",
    sqlSolved = "SQL tasks solved",
    courseProgress = "Course progress",
    recentActivity = "Activity in the last 28 days",
    studyDayTarget = "A study day is 10 answered questions or at least one solved SQL task.",
    back = "Back",
    courseMaterial = "Course material",
    postgresqlDocumentation = "PostgreSQL documentation",
    rustCore = "Rust core",
    practice = "SQL Practice",
    practiceSubtitle = "Local SQL sandbox with automatic checks",
    practiceCategories = "Practice sections",
    practiceTasks = "Tasks",
    practiceEmpty = "No SQL practice tasks are loaded yet.",
    practiceProgress = "Practice progress",
    practiceSolved = "Solved",
    practiceStarted = "Started",
    practiceAttempts = "Attempts",
    practiceFirstTry = "First try",
    practiceNoHints = "No hints",
    practiceContinue = "Continue last task",
    practiceEditor = "SQL editor",
    practiceSchema = "Schema",
    practiceData = "Table data",
    practiceResult = "Result",
    practiceLearning = "Learning",
    practiceRun = "Run",
    practiceCheck = "Check",
    practiceReset = "Reset",
    practiceHint = "Hint",
    practiceSolution = "Solution",
    practiceNext = "Next task",
    practiceCorrect = "Correct",
    practiceIncorrect = "Incorrect",
    practiceActualRows = "Actual rows",
    practiceExpectedRows = "Expected rows",
    practiceFailedChecks = "Failed checks",
    practiceNoOutput = "Run SQL or check the solution to see a result.",
    practiceAffectedRows = "Affected rows",
    practiceRows = "Rows",
    practiceOpenTable = "Open table",
    practiceNoMoreHints = "No more hints are available.",
    practiceHintLocked = "The next hint unlocks after another check attempt.",
    practiceSolutionLocked = "The solution unlocks after 3 attempts or after all hints are opened.",
    practiceLastTask = "This is the last task in the section.",
    practiceRules = "Rules",
    practiceRequireKeyword = "must use",
    practiceForbidKeyword = "do not use",
    practiceForbidSelectStar = "do not use SELECT *",
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
    unfinishedQuizzes = "Незавершённые тесты",
    noUnfinishedQuizzes = "Незавершённых тестов нет.",
    continueQuiz = "Продолжить",
    saveAndExitQuiz = "Сохранить и выйти",
    stayInQuiz = "Остаться в тесте",
    cancelQuiz = "Прервать тест",
    interruptQuizMessage = "Эта незавершённая попытка будет отмечена как отменённая и продолжить её будет нельзя. Ответы не попадут в основную статистику.",
    cancelQuizTitle = "Выйти из теста?",
    cancelQuizMessage = "«Сохранить и выйти» оставит попытку для продолжения позже. «Прервать тест» отменит её. Можно остаться в тесте.",
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
    clearScopeStatistics = "Очистить выбранную статистику",
    clearStatisticsConfirmTitle = "Очистить статистику?",
    clearStatisticsConfirmMessage = "Выбранная статистика и незавершённые тесты в этой области будут удалены без возможности восстановления.",
    clearStatisticsConfirm = "Очистить",
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
    activeDays = "Активных дней",
    studyDays = "Учебных дней",
    currentStreak = "Текущая серия",
    bestStreak = "Лучшая серия",
    today = "Сегодня",
    last7Days = "За 7 дней",
    rank = "Ранг",
    xp = "XP",
    nextRank = "До следующего ранга",
    rankScale = "Шкала рангов",
    xpRules = "За что начисляется XP",
    xpAvailable = "Доступно XP",
    xpOpportunities = "Где ещё заработать XP",
    xpQuestions = "Новые правильные ответы",
    xpQuizzes = "Бонусы за прохождение тестов",
    xpTopics = "Освоение тем",
    xpCourses = "Завершение курсов",
    xpPractice = "Практика SQL",
    xpPassBonus = "бонус за прохождение",
    xpReceived = "XP получен",
    achievements = "Достижения",
    mastery = "Мастерство",
    activeDays28 = "Активных дней за последние 28 дней",
    lastActivity = "Последняя активность",
    noRecentActivity = "За последние 28 дней учебной активности не было.",
    sqlAttempts = "Попыток SQL",
    sqlSolved = "Решено SQL-задач",
    courseProgress = "Прогресс по курсам",
    recentActivity = "Активность за последние 28 дней",
    studyDayTarget = "Учебный день — 10 отвеченных вопросов или хотя бы одна решённая SQL-задача.",
    back = "Назад",
    courseMaterial = "Материал курса",
    postgresqlDocumentation = "Документация PostgreSQL",
    rustCore = "Rust core",
    practice = "Практика SQL",
    practiceSubtitle = "Локальная SQL-песочница с автоматической проверкой",
    practiceCategories = "Разделы практики",
    practiceTasks = "Задания",
    practiceEmpty = "Задания SQL-практики пока не загружены.",
    practiceProgress = "Прогресс практики",
    practiceSolved = "Решено",
    practiceStarted = "Начато",
    practiceAttempts = "Попыток",
    practiceFirstTry = "С первой попытки",
    practiceNoHints = "Без подсказок",
    practiceContinue = "Продолжить последнее задание",
    practiceEditor = "SQL редактор",
    practiceSchema = "Схема",
    practiceData = "Данные",
    practiceResult = "Результат",
    practiceLearning = "Обучение",
    practiceRun = "Выполнить",
    practiceCheck = "Проверить",
    practiceReset = "Сброс",
    practiceHint = "Подсказка",
    practiceSolution = "Решение",
    practiceNext = "Следующее задание",
    practiceCorrect = "Верно",
    practiceIncorrect = "Неверно",
    practiceActualRows = "Получено строк",
    practiceExpectedRows = "Ожидалось строк",
    practiceFailedChecks = "Не пройдены проверки",
    practiceNoOutput = "Выполните SQL или проверьте решение, чтобы увидеть результат.",
    practiceAffectedRows = "Изменено строк",
    practiceRows = "Строк",
    practiceOpenTable = "Открыть таблицу",
    practiceNoMoreHints = "Больше подсказок нет.",
    practiceHintLocked = "Следующая подсказка откроется после ещё одной проверки.",
    practiceSolutionLocked = "Решение откроется после 3 попыток или после всех подсказок.",
    practiceLastTask = "Это последнее задание в разделе.",
    practiceRules = "Правила",
    practiceRequireKeyword = "нужно использовать",
    practiceForbidKeyword = "нельзя использовать",
    practiceForbidSelectStar = "нельзя использовать SELECT *",
)

fun uiStrings(language: UiLanguage): UiStrings = when (language) {
    UiLanguage.English -> ENGLISH_STRINGS
    UiLanguage.Russian -> RUSSIAN_STRINGS
}
