package dev.dbatrainer.app

import android.content.Context
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import dev.dbatrainer.ffi.MobileActivityStats
import dev.dbatrainer.ffi.MobileAnswerResult
import dev.dbatrainer.ffi.MobileLab
import dev.dbatrainer.ffi.MobileQuestion
import dev.dbatrainer.ffi.MobileQuestionLimit
import dev.dbatrainer.ffi.MobileQuestionMistakeDetail
import dev.dbatrainer.ffi.MobileQuestionMistakeSummary
import dev.dbatrainer.ffi.MobileQuestionType
import dev.dbatrainer.ffi.MobileSessionProgress
import dev.dbatrainer.ffi.MobileStatisticsLimit
import dev.dbatrainer.ffi.MobileTopic
import dev.dbatrainer.ffi.MobileTopicMasteryStats
import dev.dbatrainer.ffi.MobileTrainer
import dev.dbatrainer.ffi.MobileTrainingStats
import java.io.File

enum class AppScreen {
    Home,
    TopicCourses,
    Topics,
    Courses,
    Limit,
    LabCourses,
    Labs,
    LabDetail,
    Quiz,
    Feedback,
    Results,
    Statistics,
}

enum class StatisticsPage {
    Root,
    Courses,
    Topics,
    GlobalModes,
    ScopeModes,
    Mistakes,
    MistakeTopics,
    MistakeQuestions,
    MistakeDetail,
    Activity,
    Detail,
}

data class CourseStatisticsRow(
    val courseCode: String,
    val stats: MobileTrainingStats,
)

data class TopicStatisticsRow(
    val topic: MobileTopic,
    val stats: MobileTrainingStats,
)

data class ModeStatisticsRow(
    val limit: MobileStatisticsLimit,
    val title: String,
    val stats: MobileTrainingStats,
)

class TrainerController(context: Context) {
    private val preferences = context.getSharedPreferences(
        "dba_trainer_settings",
        Context.MODE_PRIVATE,
    )

    var language by mutableStateOf(
        UiLanguage.fromCode(preferences.getString("ui_language", null)),
    )
        private set

    val strings: UiStrings
        get() = uiStrings(language)

    var screen by mutableStateOf(AppScreen.Home)
        private set

    var topics by mutableStateOf<List<MobileTopic>>(emptyList())
        private set

    var labs by mutableStateOf<List<MobileLab>>(emptyList())
        private set

    var topicMastery by mutableStateOf<Map<Long, MobileTopicMasteryStats>>(emptyMap())
        private set

    var selectedTopicCourse by mutableStateOf<String?>(null)
        private set

    var selectedLabCourse by mutableStateOf<String?>(null)
        private set

    var selectedLab by mutableStateOf<MobileLab?>(null)
        private set

    var pendingTopic by mutableStateOf<MobileTopic?>(null)
        private set

    var pendingCourse by mutableStateOf<String?>(null)
        private set

    var pendingMistakeReview by mutableStateOf(false)
        private set

    var currentQuestion by mutableStateOf<MobileQuestion?>(null)
        private set

    var feedback by mutableStateOf<MobileAnswerResult?>(null)
        private set

    var resumeSession by mutableStateOf<MobileSessionProgress?>(null)
        private set

    var answeredQuestions by mutableStateOf(0)
        private set

    var correctAnswers by mutableStateOf(0)
        private set

    var totalQuestions by mutableStateOf(0)
        private set

    var activeQuestionLimit by mutableStateOf<MobileQuestionLimit?>(
        preferences
            .getString("active_question_limit", null)
            ?.let { stored ->
                MobileQuestionLimit.entries.firstOrNull { limit -> limit.name == stored }
            },
    )
        private set

    var errorMessage by mutableStateOf<String?>(null)
        private set

    var showCancelDialog by mutableStateOf(false)
        private set

    val selectedOptionIds = mutableStateListOf<Long>()

    var statisticsPage by mutableStateOf(StatisticsPage.Root)
        private set

    var overallStatistics by mutableStateOf<MobileTrainingStats?>(null)
        private set

    var courseStatistics by mutableStateOf<List<CourseStatisticsRow>>(emptyList())
        private set

    var topicStatistics by mutableStateOf<List<TopicStatisticsRow>>(emptyList())
        private set

    var globalModeStatistics by mutableStateOf<List<ModeStatisticsRow>>(emptyList())
        private set

    var scopeModeStatistics by mutableStateOf<List<ModeStatisticsRow>>(emptyList())
        private set

    var statisticsDetail by mutableStateOf<MobileTrainingStats?>(null)
        private set

    var statisticsDetailTitle by mutableStateOf("")
        private set

    var mistakeSummaries by mutableStateOf<List<MobileQuestionMistakeSummary>>(emptyList())
        private set

    var mistakeDetail by mutableStateOf<MobileQuestionMistakeDetail?>(null)
        private set

    var selectedMistakeCourse by mutableStateOf<String?>(null)
        private set

    var selectedMistakeTopic by mutableStateOf<MobileTopic?>(null)
        private set

    var activityStats by mutableStateOf<MobileActivityStats?>(null)
        private set

    var selectedStatisticsCourse by mutableStateOf<String?>(null)
        private set

    var selectedStatisticsTopic by mutableStateOf<MobileTopic?>(null)
        private set

    private var statisticsDetailReturnPage = StatisticsPage.Root
    private var trainer: MobileTrainer? = null
    private var sessionId: Long? = null

    init {
        runCoreAction {
            val databasePath = File(
                context.filesDir,
                "dba_trainer.db",
            ).absolutePath

            trainer = MobileTrainer(databasePath)
            requireTrainer().setContentLocale(language.code)
            topics = requireTrainer().topics()
            labs = requireTrainer().labs()
            activityStats = requireTrainer().activityStats()
            resumeSession = requireTrainer().activeSession()

            if (resumeSession != null && activeQuestionLimit == null) {
                activeQuestionLimit = inferQuestionLimit(resumeSession!!.totalQuestions)
            }
        }
    }

    fun passThresholdPercent(limit: MobileQuestionLimit): Int =
        when (limit) {
            MobileQuestionLimit.TWENTY -> 85
            MobileQuestionLimit.FIFTY -> 70
            MobileQuestionLimit.ALL -> 60
        }

    fun requiredCorrectAnswers(limit: MobileQuestionLimit, total: Int): Int {
        if (total <= 0) {
            return 0
        }

        val percent = passThresholdPercent(limit)
        return (total * percent + 99) / 100
    }

    val resultPassed: Boolean
        get() {
            val limit = activeQuestionLimit ?: return false
            return answeredQuestions > 0 &&
                correctAnswers >= requiredCorrectAnswers(limit, answeredQuestions)
        }

    val canGoBack: Boolean
        get() = screen != AppScreen.Home

    val courseCodes: List<String>
        get() = topics
            .map { topic -> topic.courseCode }
            .distinct()
            .sortedWith(courseCodeComparator)

    val topicsForSelectedCourse: List<MobileTopic>
        get() {
            val courseCode = selectedTopicCourse ?: return emptyList()
            return topics.filter { topic -> topic.courseCode == courseCode }
        }

    val labCourseCodes: List<String>
        get() = labs
            .map { lab -> lab.courseCode }
            .distinct()
            .sortedWith(courseCodeComparator)

    val labsForSelectedCourse: List<MobileLab>
        get() {
            val courseCode = selectedLabCourse ?: return emptyList()
            return labs.filter { lab -> lab.courseCode == courseCode }
        }

    val statisticsScopeTitle: String
        get() = selectedStatisticsTopic?.let { topic ->
            "${topic.courseCode.uppercase()} → ${topic.title}"
        } ?: selectedStatisticsCourse?.uppercase().orEmpty()

    val mistakeCourseCodes: List<String>
        get() = mistakeSummaries
            .map { summary -> summary.courseCode }
            .distinct()
            .sortedWith(courseCodeComparator)

    val mistakeTopicsForSelectedCourse: List<MobileTopic>
        get() {
            val courseCode = selectedMistakeCourse ?: return emptyList()
            val topicIds = mistakeSummaries
                .asSequence()
                .filter { summary -> summary.courseCode == courseCode }
                .map { summary -> summary.topicId }
                .toSet()

            return topics
                .filter { topic -> topic.courseCode == courseCode && topic.id in topicIds }
                .sortedBy { topic -> topic.topicNumber }
        }

    val mistakesForSelectedTopic: List<MobileQuestionMistakeSummary>
        get() {
            val topicId = selectedMistakeTopic?.id ?: return emptyList()
            return mistakeSummaries.filter { summary -> summary.topicId == topicId }
        }

    fun mistakeCountForCourse(courseCode: String): Int =
        mistakeSummaries.count { summary -> summary.courseCode == courseCode }

    fun mistakeCountForTopic(topicId: Long): Int =
        mistakeSummaries.count { summary -> summary.topicId == topicId }

    fun changeLanguage(language: UiLanguage) {
        if (this.language == language) {
            return
        }

        runCoreAction {
            requireTrainer().setContentLocale(language.code)
            topics = requireTrainer().topics()
            labs = requireTrainer().labs()
            this.language = language
            preferences
                .edit()
                .putString("ui_language", language.code)
                .apply()
        }
    }

    fun openTopics() {
        selectedTopicCourse = null
        pendingTopic = null
        pendingCourse = null
        pendingMistakeReview = false
        screen = AppScreen.TopicCourses
    }

    fun chooseTopicCourse(courseCode: String) {
        runCoreAction {
            selectedTopicCourse = courseCode
            pendingTopic = null
            pendingCourse = null
            val visibleTopics = topics.filter { topic -> topic.courseCode == courseCode }
            topicMastery = visibleTopics.associate { topic ->
                topic.id to requireTrainer().topicMastery(topic.id)
            }
            screen = AppScreen.Topics
        }
    }

    fun topicMasteryFor(topicId: Long): MobileTopicMasteryStats? =
        topicMastery[topicId]

    fun openLabs() {
        selectedLabCourse = null
        selectedLab = null
        screen = AppScreen.LabCourses
    }

    fun chooseLabCourse(courseCode: String) {
        selectedLabCourse = courseCode
        selectedLab = null
        screen = AppScreen.Labs
    }

    fun chooseLab(lab: MobileLab) {
        selectedLab = lab
        screen = AppScreen.LabDetail
    }

    fun openGeneralQuiz() {
        pendingTopic = null
        pendingCourse = null
        pendingMistakeReview = false
        screen = AppScreen.Courses
    }

    fun chooseCourse(courseCode: String) {
        pendingTopic = null
        pendingCourse = courseCode
        pendingMistakeReview = false
        screen = AppScreen.Limit
    }

    fun chooseTopic(topic: MobileTopic) {
        pendingCourse = null
        pendingTopic = topic
        pendingMistakeReview = false
        screen = AppScreen.Limit
    }

    fun startQuiz(limit: MobileQuestionLimit) {
        runCoreAction {
            val session = when {
                pendingMistakeReview && pendingTopic != null -> {
                    requireTrainer().startMistakeTopicSession(
                        pendingTopic!!.id,
                        limit,
                    )
                }

                pendingMistakeReview && pendingCourse != null -> {
                    requireTrainer().startMistakeCourseSession(
                        pendingCourse!!,
                        limit,
                    )
                }

                pendingMistakeReview -> requireTrainer().startMistakeSession(limit)

                pendingTopic != null -> {
                    requireTrainer().startTopicSession(
                        pendingTopic!!.id,
                        limit,
                    )
                }

                pendingCourse != null -> {
                    requireTrainer().startCourseSession(
                        pendingCourse!!,
                        limit,
                    )
                }

                else -> requireTrainer().startAllTopicsSession(limit)
            }

            activeQuestionLimit = limit
            preferences
                .edit()
                .putString("active_question_limit", limit.name)
                .apply()

            sessionId = session.id
            answeredQuestions = 0
            correctAnswers = 0
            totalQuestions = session.totalQuestions.toInt()
            feedback = null
            selectedOptionIds.clear()
            loadCurrentQuestion()
        }
    }

    fun continueSavedSession() {
        val progress = resumeSession ?: return

        runCoreAction {
            pendingMistakeReview = progress.isReview
            pendingCourse = if (progress.isReview) progress.courseCode else null
            pendingTopic = if (progress.isReview) {
                progress.topicId?.let { topicId ->
                    topics.firstOrNull { topic -> topic.id == topicId }
                }
            } else {
                null
            }

            sessionId = progress.id
            answeredQuestions = progress.answeredQuestions.toInt()
            correctAnswers = progress.correctAnswers.toInt()
            totalQuestions = progress.totalQuestions.toInt()
            resumeSession = null
            feedback = null
            selectedOptionIds.clear()
            loadCurrentQuestion()
        }
    }

    fun discardSavedSession() {
        val progress = resumeSession ?: return

        runCoreAction {
            requireTrainer().cancelSession(progress.id)
            activityStats = requireTrainer().activityStats()
            resumeSession = null
            resetQuizState()
            screen = AppScreen.Home
        }
    }

    fun selectOption(optionId: Long) {
        val question = currentQuestion ?: return

        if (question.questionType == MobileQuestionType.SINGLE_CHOICE) {
            selectedOptionIds.clear()
            selectedOptionIds.add(optionId)
            return
        }

        if (selectedOptionIds.contains(optionId)) {
            selectedOptionIds.remove(optionId)
        } else {
            selectedOptionIds.add(optionId)
        }
    }

    fun submitAnswer() {
        val currentSessionId = sessionId ?: return
        val question = currentQuestion ?: return

        if (selectedOptionIds.isEmpty()) {
            errorMessage = strings.selectAtLeastOne
            return
        }

        runCoreAction {
            val result = requireTrainer().submitAnswer(
                currentSessionId,
                question.id,
                selectedOptionIds.toList(),
            )

            feedback = result
            answeredQuestions += 1

            if (result.isCorrect) {
                correctAnswers += 1
            }

            screen = AppScreen.Feedback
        }
    }

    fun nextQuestion() {
        runCoreAction {
            feedback = null
            selectedOptionIds.clear()
            loadCurrentQuestion()
        }
    }

    fun requestCancelQuiz() {
        if (sessionId != null) {
            showCancelDialog = true
        } else {
            back()
        }
    }

    fun dismissCancelDialog() {
        showCancelDialog = false
    }

    fun confirmCancelQuiz() {
        val currentSessionId = sessionId ?: return

        runCoreAction {
            requireTrainer().cancelSession(currentSessionId)
            activityStats = requireTrainer().activityStats()
            showCancelDialog = false
            resetQuizState()
            screen = AppScreen.Home
        }
    }

    fun finishResults() {
        val wasMistakeReview = pendingMistakeReview
        val mistakeReturnPage = when {
            pendingTopic != null -> StatisticsPage.MistakeQuestions
            pendingCourse != null -> StatisticsPage.MistakeTopics
            else -> StatisticsPage.Mistakes
        }

        resetQuizState()

        if (wasMistakeReview) {
            runCoreAction {
                mistakeSummaries = requireTrainer().mistakeSummaries()
                mistakeDetail = null
                statisticsPage = mistakeReturnPage
                screen = AppScreen.Statistics
            }
        } else {
            screen = AppScreen.Home
        }
    }

    fun openStatistics() {
        runCoreAction {
            refreshStatisticsOverview()
            resetStatisticsNavigation()
            screen = AppScreen.Statistics
        }
    }

    fun openOverallStatistics() {
        val stats = overallStatistics ?: return
        statisticsDetail = stats
        statisticsDetailTitle = strings.overallStatistics
        statisticsDetailReturnPage = StatisticsPage.Root
        statisticsPage = StatisticsPage.Detail
    }

    fun openStatisticsCourses() {
        statisticsPage = StatisticsPage.Courses
    }

    fun chooseStatisticsCourse(courseCode: String) {
        runCoreAction {
            selectedStatisticsCourse = courseCode
            selectedStatisticsTopic = null

            topicStatistics = topics
                .filter { topic -> topic.courseCode == courseCode }
                .map { topic ->
                    TopicStatisticsRow(
                        topic = topic,
                        stats = requireTrainer().statisticsTopic(
                            topic.id,
                            MobileStatisticsLimit.ANY,
                        ),
                    )
                }

            statisticsPage = StatisticsPage.Topics
        }
    }

    fun openStatisticsGlobalModes() {
        statisticsPage = StatisticsPage.GlobalModes
    }

    fun openCourseAllTopicsModes() {
        val courseCode = selectedStatisticsCourse ?: return

        runCoreAction {
            selectedStatisticsTopic = null
            scopeModeStatistics = loadModeStatistics { limit ->
                requireTrainer().statisticsCourse(
                    courseCode,
                    limit,
                )
            }
            statisticsPage = StatisticsPage.ScopeModes
        }
    }

    fun openTopicModes(topic: MobileTopic) {
        runCoreAction {
            selectedStatisticsTopic = topic
            scopeModeStatistics = loadModeStatistics { limit ->
                requireTrainer().statisticsTopic(
                    topic.id,
                    limit,
                )
            }
            statisticsPage = StatisticsPage.ScopeModes
        }
    }

    fun openGlobalModeDetail(row: ModeStatisticsRow) {
        statisticsDetail = row.stats
        statisticsDetailTitle = row.title
        statisticsDetailReturnPage = StatisticsPage.GlobalModes
        statisticsPage = StatisticsPage.Detail
    }

    fun openScopeModeDetail(row: ModeStatisticsRow) {
        statisticsDetail = row.stats
        statisticsDetailTitle = buildString {
            append(statisticsScopeTitle)
            if (isNotEmpty()) {
                append(" → ")
            }
            append(row.title)
        }
        statisticsDetailReturnPage = StatisticsPage.ScopeModes
        statisticsPage = StatisticsPage.Detail
    }

    fun openMistakes() {
        runCoreAction {
            mistakeSummaries = requireTrainer().mistakeSummaries()
            mistakeDetail = null
            selectedMistakeCourse = null
            selectedMistakeTopic = null
            statisticsPage = StatisticsPage.Mistakes
        }
    }

    fun openMistakeCourse(courseCode: String) {
        selectedMistakeCourse = courseCode
        selectedMistakeTopic = null
        statisticsPage = StatisticsPage.MistakeTopics
    }

    fun openMistakeTopic(topic: MobileTopic) {
        selectedMistakeTopic = topic
        statisticsPage = StatisticsPage.MistakeQuestions
    }

    fun openMistakeDetail(summary: MobileQuestionMistakeSummary) {
        runCoreAction {
            mistakeDetail = requireTrainer().mistakeDetail(summary.questionId)
            statisticsPage = StatisticsPage.MistakeDetail
        }
    }

    fun openMistakeReview() {
        if (mistakeSummaries.isEmpty()) {
            return
        }

        pendingTopic = null
        pendingCourse = null
        pendingMistakeReview = true
        screen = AppScreen.Limit
    }

    fun openMistakeCourseReview() {
        val courseCode = selectedMistakeCourse ?: return
        if (mistakeCountForCourse(courseCode) == 0) {
            return
        }

        pendingTopic = null
        pendingCourse = courseCode
        pendingMistakeReview = true
        screen = AppScreen.Limit
    }

    fun openMistakeTopicReview() {
        val topic = selectedMistakeTopic ?: return
        if (mistakeCountForTopic(topic.id) == 0) {
            return
        }

        pendingCourse = null
        pendingTopic = topic
        pendingMistakeReview = true
        screen = AppScreen.Limit
    }

    fun openActivity() {
        runCoreAction {
            activityStats = requireTrainer().activityStats()
            statisticsPage = StatisticsPage.Activity
        }
    }

    fun clearStatistics() {
        runCoreAction {
            requireTrainer().clearStatistics()
            refreshStatisticsOverview()
            resetStatisticsNavigation()
            mistakeSummaries = emptyList()
            mistakeDetail = null
            activityStats = requireTrainer().activityStats()
        }
    }

    fun dismissError() {
        errorMessage = null
    }

    fun back() {
        when (screen) {
            AppScreen.Home -> Unit
            AppScreen.TopicCourses -> screen = AppScreen.Home
            AppScreen.Topics -> screen = AppScreen.TopicCourses
            AppScreen.Courses -> screen = AppScreen.Home
            AppScreen.LabCourses -> screen = AppScreen.Home
            AppScreen.Labs -> {
                selectedLab = null
                screen = AppScreen.LabCourses
            }
            AppScreen.LabDetail -> screen = AppScreen.Labs
            AppScreen.Limit -> {
                screen = when {
                    pendingMistakeReview -> {
                        statisticsPage = when {
                            pendingTopic != null -> StatisticsPage.MistakeQuestions
                            pendingCourse != null -> StatisticsPage.MistakeTopics
                            else -> StatisticsPage.Mistakes
                        }
                        pendingMistakeReview = false
                        pendingTopic = null
                        pendingCourse = null
                        AppScreen.Statistics
                    }
                    pendingTopic != null -> AppScreen.Topics
                    pendingCourse != null -> AppScreen.Courses
                    else -> AppScreen.Home
                }
            }

            AppScreen.Quiz,
            AppScreen.Feedback -> requestCancelQuiz()

            AppScreen.Results -> finishResults()
            AppScreen.Statistics -> statisticsBack()
        }
    }

    private fun statisticsBack() {
        when (statisticsPage) {
            StatisticsPage.Root -> screen = AppScreen.Home
            StatisticsPage.Courses -> statisticsPage = StatisticsPage.Root
            StatisticsPage.Topics -> {
                selectedStatisticsCourse = null
                selectedStatisticsTopic = null
                topicStatistics = emptyList()
                statisticsPage = StatisticsPage.Courses
            }

            StatisticsPage.GlobalModes -> statisticsPage = StatisticsPage.Root
            StatisticsPage.ScopeModes -> {
                selectedStatisticsTopic = null
                scopeModeStatistics = emptyList()
                statisticsPage = StatisticsPage.Topics
            }

            StatisticsPage.Mistakes -> statisticsPage = StatisticsPage.Root
            StatisticsPage.MistakeTopics -> {
                selectedMistakeCourse = null
                selectedMistakeTopic = null
                statisticsPage = StatisticsPage.Mistakes
            }
            StatisticsPage.MistakeQuestions -> {
                selectedMistakeTopic = null
                statisticsPage = StatisticsPage.MistakeTopics
            }
            StatisticsPage.MistakeDetail -> {
                mistakeDetail = null
                statisticsPage = StatisticsPage.MistakeQuestions
            }
            StatisticsPage.Activity -> statisticsPage = StatisticsPage.Root
            StatisticsPage.Detail -> statisticsPage = statisticsDetailReturnPage
        }
    }

    private fun loadCurrentQuestion() {
        val currentSessionId = sessionId
            ?: error("Quiz session is missing")

        val question = requireTrainer().currentQuestion(currentSessionId)

        if (question == null) {
            currentQuestion = null
            feedback = null
            selectedOptionIds.clear()
            activityStats = requireTrainer().activityStats()
            screen = AppScreen.Results
            return
        }

        currentQuestion = question
        selectedOptionIds.clear()
        screen = AppScreen.Quiz
    }

    private fun refreshStatisticsOverview() {
        val trainer = requireTrainer()

        overallStatistics = trainer.statisticsAll(
            MobileStatisticsLimit.ANY,
        )

        val courseCodes = topics
            .map { topic -> topic.courseCode }
            .distinct()
            .sortedWith(courseCodeComparator)

        courseStatistics = courseCodes.map { courseCode ->
            CourseStatisticsRow(
                courseCode = courseCode,
                stats = trainer.statisticsCourse(
                    courseCode,
                    MobileStatisticsLimit.ANY,
                ),
            )
        }

        globalModeStatistics = listOf(
            ModeStatisticsRow(
                limit = MobileStatisticsLimit.TWENTY,
                title = strings.twentyQuestions,
                stats = trainer.statisticsAll(
                    MobileStatisticsLimit.TWENTY,
                ),
            ),
            ModeStatisticsRow(
                limit = MobileStatisticsLimit.FIFTY,
                title = strings.fiftyQuestions,
                stats = trainer.statisticsAll(
                    MobileStatisticsLimit.FIFTY,
                ),
            ),
            ModeStatisticsRow(
                limit = MobileStatisticsLimit.ALL_QUESTIONS,
                title = strings.allQuestions,
                stats = trainer.statisticsAll(
                    MobileStatisticsLimit.ALL_QUESTIONS,
                ),
            ),
        )
    }

    private fun loadModeStatistics(
        load: (MobileStatisticsLimit) -> MobileTrainingStats,
    ): List<ModeStatisticsRow> = listOf(
        ModeStatisticsRow(
            limit = MobileStatisticsLimit.ANY,
            title = strings.allModes,
            stats = load(MobileStatisticsLimit.ANY),
        ),
        ModeStatisticsRow(
            limit = MobileStatisticsLimit.TWENTY,
            title = strings.twentyQuestions,
            stats = load(MobileStatisticsLimit.TWENTY),
        ),
        ModeStatisticsRow(
            limit = MobileStatisticsLimit.FIFTY,
            title = strings.fiftyQuestions,
            stats = load(MobileStatisticsLimit.FIFTY),
        ),
        ModeStatisticsRow(
            limit = MobileStatisticsLimit.ALL_QUESTIONS,
            title = strings.allQuestions,
            stats = load(MobileStatisticsLimit.ALL_QUESTIONS),
        ),
    )

    private fun resetStatisticsNavigation() {
        statisticsPage = StatisticsPage.Root
        selectedStatisticsCourse = null
        selectedStatisticsTopic = null
        topicStatistics = emptyList()
        scopeModeStatistics = emptyList()
        statisticsDetail = null
        statisticsDetailTitle = ""
        mistakeDetail = null
        selectedMistakeCourse = null
        selectedMistakeTopic = null
        statisticsDetailReturnPage = StatisticsPage.Root
    }

    private fun resetQuizState() {
        sessionId = null
        currentQuestion = null
        feedback = null
        selectedOptionIds.clear()
        answeredQuestions = 0
        correctAnswers = 0
        totalQuestions = 0
        activeQuestionLimit = null
        preferences
            .edit()
            .remove("active_question_limit")
            .apply()
        selectedTopicCourse = null
        pendingTopic = null
        pendingCourse = null
        pendingMistakeReview = false
    }

    private fun requireTrainer(): MobileTrainer =
        trainer ?: error("Rust core is not initialized")

    private inline fun runCoreAction(action: () -> Unit) {
        try {
            action()
        } catch (error: Exception) {
            errorMessage = error.message
                ?: error::class.java.simpleName
        }
    }

    private companion object {
        val courseCodeComparator = Comparator<String> { left, right ->
            val leftParts = splitCourseCode(left)
            val rightParts = splitCourseCode(right)

            val prefixComparison = leftParts.first.compareTo(rightParts.first)

            when {
                prefixComparison != 0 -> prefixComparison
                leftParts.second != null && rightParts.second != null ->
                    leftParts.second!!.compareTo(rightParts.second!!)
                else -> left.compareTo(right)
            }
        }

        fun inferQuestionLimit(totalQuestions: ULong): MobileQuestionLimit =
            when (totalQuestions) {
                20UL -> MobileQuestionLimit.TWENTY
                50UL -> MobileQuestionLimit.FIFTY
                else -> MobileQuestionLimit.ALL
            }

        fun splitCourseCode(value: String): Pair<String, Int?> {
            val separator = value.lastIndexOf('-')

            if (separator <= 0 || separator == value.lastIndex) {
                return value to null
            }

            return value.substring(0, separator) to
                value.substring(separator + 1).toIntOrNull()
        }
    }
}
