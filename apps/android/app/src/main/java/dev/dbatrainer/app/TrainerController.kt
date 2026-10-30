package dev.dbatrainer.app

import android.content.Context
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import dev.dbatrainer.ffi.MobileActivityStats
import dev.dbatrainer.ffi.MobileAnswerResult
import dev.dbatrainer.ffi.MobileLab
import dev.dbatrainer.ffi.MobilePracticeCategoryStats
import dev.dbatrainer.ffi.MobilePracticeCheckResult
import dev.dbatrainer.ffi.MobilePracticeExecutionResult
import dev.dbatrainer.ffi.MobilePracticeSchema
import dev.dbatrainer.ffi.MobilePracticeStats
import dev.dbatrainer.ffi.MobilePracticeTableData
import dev.dbatrainer.ffi.MobilePracticeDifficulty
import dev.dbatrainer.ffi.MobilePracticeTask
import dev.dbatrainer.ffi.MobilePracticeTaskProgress
import dev.dbatrainer.ffi.MobilePracticeXpOpportunity
import dev.dbatrainer.ffi.MobileQuestion
import dev.dbatrainer.ffi.MobileQuestionLimit
import dev.dbatrainer.ffi.MobileQuestionMistakeDetail
import dev.dbatrainer.ffi.MobileQuestionMistakeSummary
import dev.dbatrainer.ffi.MobileQuizXpOpportunity
import dev.dbatrainer.ffi.MobileQuizXpScopeKind
import dev.dbatrainer.ffi.MobileQuestionType
import dev.dbatrainer.ffi.MobileSessionProgress
import dev.dbatrainer.ffi.MobileStatisticsLimit
import dev.dbatrainer.ffi.MobileTopic
import dev.dbatrainer.ffi.MobileTopicMasteryStats
import dev.dbatrainer.ffi.MobileTrainer
import dev.dbatrainer.ffi.MobileTrainingStats
import dev.dbatrainer.app.game.DbaWorldController
import dev.dbatrainer.app.game.domain.KnowledgeSnapshot
import dev.dbatrainer.app.game.domain.KnowledgeTopicProgress
import dev.dbatrainer.app.game.engine.KnowledgeEngine
import java.io.File

enum class AppScreen {
    Home,
    UnfinishedQuizzes,
    TopicCourses,
    Topics,
    NotesCourses,
    NotesTopics,
    NoteDetail,
    Courses,
    Limit,
    LabCourses,
    Labs,
    LabDetail,
    PracticeCategories,
    PracticeTasks,
    PracticeWorkbench,
    Quiz,
    Feedback,
    Results,
    Statistics,
    DbaWorld,
}

enum class PracticePane {
    Editor,
    Schema,
    Data,
    Result,
    Learning,
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
    Practice,
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

sealed interface StatisticsResetTarget {
    data object All : StatisticsResetTarget
    data class Course(val courseCode: String) : StatisticsResetTarget
    data class Topic(val topic: MobileTopic) : StatisticsResetTarget
    data class GlobalMode(
        val limit: MobileStatisticsLimit,
        val title: String,
    ) : StatisticsResetTarget
    data class CourseMode(
        val courseCode: String,
        val limit: MobileStatisticsLimit,
        val title: String,
    ) : StatisticsResetTarget
    data class TopicMode(
        val topic: MobileTopic,
        val limit: MobileStatisticsLimit,
        val title: String,
    ) : StatisticsResetTarget
}

class TrainerController(context: Context) {
    val dbaWorld = DbaWorldController(context)

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

    var selectedNotesCourse by mutableStateOf<String?>(null)
        private set

    var selectedNoteTopic by mutableStateOf<MobileTopic?>(null)
        private set

    var selectedNoteBody by mutableStateOf("")
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

    var unfinishedSessions by mutableStateOf<List<MobileSessionProgress>>(emptyList())
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

    var pendingDiscardSession by mutableStateOf<MobileSessionProgress?>(null)
        private set

    val selectedOptionIds = mutableStateListOf<Long>()

    var practiceTasks by mutableStateOf<List<MobilePracticeTask>>(emptyList())
        private set

    var practiceProgress by mutableStateOf<List<MobilePracticeTaskProgress>>(emptyList())
        private set

    var practiceStats by mutableStateOf<MobilePracticeStats?>(null)
        private set

    var selectedPracticeCategory by mutableStateOf<String?>(null)
        private set

    var currentPracticeTask by mutableStateOf<MobilePracticeTask?>(null)
        private set

    var practiceSql by mutableStateOf("")
        private set

    var practiceSchema by mutableStateOf<MobilePracticeSchema?>(null)
        private set

    var practiceTableData by mutableStateOf<MobilePracticeTableData?>(null)
        private set

    var practiceOutput by mutableStateOf<MobilePracticeExecutionResult?>(null)
        private set

    var practiceCheck by mutableStateOf<MobilePracticeCheckResult?>(null)
        private set

    var practicePane by mutableStateOf(PracticePane.Editor)
        private set

    var practiceHintCount by mutableStateOf(0)
        private set

    var practiceShowSolution by mutableStateOf(false)
        private set

    var practiceMessage by mutableStateOf<String?>(null)
        private set

    var practiceError by mutableStateOf<String?>(null)
        private set

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

    var statisticsDetailLimit by mutableStateOf<MobileStatisticsLimit?>(null)
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

    var pendingStatisticsReset by mutableStateOf<StatisticsResetTarget?>(null)
        private set

    private var statisticsDetailReturnPage = StatisticsPage.Root
    private var returnToDbaWorldAfterLearning = false
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
            refreshPracticeState()
            activityStats = requireTrainer().activityStats()
            unfinishedSessions = requireTrainer().unfinishedSessions()
        }
    }

    fun passThresholdPercent(limit: MobileQuestionLimit): Int =
        when (limit) {
            MobileQuestionLimit.TWENTY -> 85
            MobileQuestionLimit.FIFTY -> 70
            MobileQuestionLimit.ALL -> 60
        }

    fun questionLimitTitle(limit: MobileQuestionLimit): String =
        when (limit) {
            MobileQuestionLimit.TWENTY -> strings.twentyQuestions
            MobileQuestionLimit.FIFTY -> strings.fiftyQuestions
            MobileQuestionLimit.ALL -> strings.allQuestions
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

    val topicsForSelectedNotesCourse: List<MobileTopic>
        get() {
            val courseCode = selectedNotesCourse ?: return emptyList()
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

    val practiceCategories: List<String>
        get() = practiceTasks
            .map { task -> task.category }
            .distinct()
            .sorted()

    val practiceTasksForSelectedCategory: List<MobilePracticeTask>
        get() {
            val category = selectedPracticeCategory ?: return emptyList()
            return listOf(
                MobilePracticeDifficulty.EASY,
                MobilePracticeDifficulty.MEDIUM,
                MobilePracticeDifficulty.HARD,
            ).flatMap { difficulty ->
                practiceTasks.filter { task ->
                    task.category == category && task.difficulty == difficulty
                }
            }
        }

    val resumablePracticeTask: MobilePracticeTask?
        get() {
            val taskId = practiceProgress
                .firstOrNull { progress -> progress.started && !progress.solved }
                ?.taskId
                ?: return null
            return practiceTasks.firstOrNull { task -> task.id == taskId }
        }

    val practiceSolutionAvailable: Boolean
        get() {
            val task = currentPracticeTask ?: return false
            val attempts = practiceProgressFor(task.id)?.attempts?.toInt() ?: 0
            return attempts >= 3 ||
                (task.hints.isNotEmpty() && practiceHintCount >= task.hints.size)
        }

    fun practiceProgressFor(taskId: Long): MobilePracticeTaskProgress? =
        practiceProgress.firstOrNull { progress -> progress.taskId == taskId }

    fun practiceCategoryStats(category: String): MobilePracticeCategoryStats? =
        practiceStats?.categories?.firstOrNull { stats -> stats.category == category }

    fun practiceXpOpportunityFor(taskId: Long): MobilePracticeXpOpportunity? =
        activityStats
            ?.practiceXpOpportunities
            ?.firstOrNull { opportunity -> opportunity.taskId == taskId }

    fun practiceXpAvailable(taskId: Long): ULong =
        practiceXpOpportunityFor(taskId)?.totalAvailableXp ?: 0UL

    fun practiceCategoryXpAvailable(category: String): ULong =
        practiceTasks
            .asSequence()
            .filter { task -> task.category == category }
            .fold(0UL) { total, task -> total + practiceXpAvailable(task.id) }

    fun changeLanguage(language: UiLanguage) {
        if (this.language == language) {
            return
        }

        runCoreAction {
            requireTrainer().setContentLocale(language.code)
            topics = requireTrainer().topics()
            labs = requireTrainer().labs()
            refreshPracticeState()
            resetPracticeWorkbench()
            selectedNoteBody = ""
            this.language = language
            preferences
                .edit()
                .putString("ui_language", language.code)
                .apply()
        }
    }

    fun openPractice() {
        runCoreAction {
            refreshPracticeState()
            selectedPracticeCategory = null
            resetPracticeWorkbench()
            screen = AppScreen.PracticeCategories
        }
    }

    fun choosePracticeCategory(category: String) {
        selectedPracticeCategory = category
        screen = AppScreen.PracticeTasks
    }

    fun resumePractice() {
        resumablePracticeTask?.let(::openPracticeTask)
    }

    fun openPracticeTask(task: MobilePracticeTask) {
        runCoreAction {
            val schema = requireTrainer().practiceLoadTask(task.id)
            val progress = practiceProgressFor(task.id)
            val sql = progress
                ?.lastSql
                ?.takeIf { value -> value.isNotBlank() }
                ?: task.starterSql

            currentPracticeTask = task
            selectedPracticeCategory = task.category
            practiceSql = sql
            practiceSchema = schema
            practiceTableData = null
            practiceOutput = null
            practiceCheck = null
            practicePane = PracticePane.Editor
            practiceHintCount = minOf(
                progress?.hintsRevealed?.toInt() ?: 0,
                task.hints.size,
            )
            practiceShowSolution = false
            practiceMessage = null
            practiceError = null
            screen = AppScreen.PracticeWorkbench
        }
    }

    fun updatePracticeSql(sql: String) {
        practiceSql = sql
        practiceCheck = null
        practiceError = null
        practiceMessage = null
    }

    fun selectPracticePane(pane: PracticePane) {
        practicePane = pane
    }

    fun persistPracticeDraft() {
        val task = currentPracticeTask ?: return
        runCoreAction {
            requireTrainer().practiceSaveDraft(task.id, practiceSql)
        }
    }

    fun runPracticeSql() {
        val task = currentPracticeTask ?: return
        practiceMessage = null
        practiceError = null
        try {
            val output = requireTrainer().practiceExecute(task.id, practiceSql)
            practiceOutput = output
            practiceCheck = null
            practiceSchema = requireTrainer().practiceSchema(task.id)
            practiceTableData = null
            practicePane = PracticePane.Result
            refreshPracticeState()
        } catch (error: Exception) {
            practiceError = error.message ?: error::class.java.simpleName
            practicePane = PracticePane.Learning
        }
    }

    fun checkPracticeSql() {
        val task = currentPracticeTask ?: return
        practiceMessage = null
        practiceError = null
        try {
            val result = requireTrainer().practiceCheck(task.id, practiceSql)
            practiceCheck = result
            refreshPracticeState()
            activityStats = requireTrainer().activityStats()
            practicePane = PracticePane.Result
        } catch (error: Exception) {
            practiceError = error.message ?: error::class.java.simpleName
            practicePane = PracticePane.Learning
        }
    }

    fun resetPracticeSandbox() {
        val task = currentPracticeTask ?: return
        runCoreAction {
            practiceSchema = requireTrainer().practiceReset(task.id)
            practiceSql = task.starterSql
            practiceTableData = null
            practiceOutput = null
            practiceCheck = null
            practicePane = PracticePane.Editor
            practiceShowSolution = false
            practiceMessage = null
            practiceError = null
            refreshPracticeState()
        }
    }

    fun showPracticeHint() {
        val task = currentPracticeTask ?: return
        val attempts = practiceProgressFor(task.id)?.attempts?.toInt() ?: 0
        val unlocked = minOf(attempts + 1, task.hints.size)

        practiceMessage = null
        when {
            practiceHintCount < unlocked -> runCoreAction {
                requireTrainer().practiceRecordHint(task.id)
                practiceHintCount += 1
                refreshPracticeState()
                activityStats = requireTrainer().activityStats()
            }
            practiceHintCount >= task.hints.size ->
                practiceMessage = strings.practiceNoMoreHints
            else ->
                practiceMessage = strings.practiceHintLocked
        }
        practicePane = PracticePane.Learning
    }

    fun togglePracticeSolution() {
        practiceMessage = null
        if (practiceSolutionAvailable) {
            practiceShowSolution = !practiceShowSolution
        } else {
            practiceMessage = strings.practiceSolutionLocked
        }
        practicePane = PracticePane.Learning
    }

    fun openPracticeTable(table: String) {
        val task = currentPracticeTask ?: return
        runCoreAction {
            practiceTableData = requireTrainer().practiceTableData(
                task.id,
                table,
                PRACTICE_TABLE_PREVIEW_LIMIT.toULong(),
                0UL,
            )
            practicePane = PracticePane.Data
        }
    }

    fun nextPracticeTask() {
        val current = currentPracticeTask ?: return
        val tasks = practiceTasksForSelectedCategory
        val index = tasks.indexOfFirst { task -> task.id == current.id }
        val next = tasks.getOrNull(index + 1)

        if (next == null) {
            practiceMessage = strings.practiceLastTask
            practicePane = PracticePane.Learning
            return
        }

        savePracticeDraft()
        openPracticeTask(next)
    }

    fun openDbaWorld() {
        runCoreAction {
            returnToDbaWorldAfterLearning = false
            unfinishedSessions = requireTrainer().unfinishedSessions()
            dbaWorld.open(buildDbaWorldKnowledgeSnapshot())
            screen = AppScreen.DbaWorld
        }
    }

    fun openDbaWorldKnowledgeNote(courseCode: String, topicSlug: String) {
        runCoreAction {
            val topic = requireDbaWorldTopic(courseCode, topicSlug)
            returnToDbaWorldAfterLearning = true
            selectedNotesCourse = courseCode
            selectedNoteTopic = topic
            selectedNoteBody = requireTrainer().topicNote(topic.slug, language.code)
            screen = AppScreen.NoteDetail
        }
    }

    fun openDbaWorldKnowledgeQuiz(courseCode: String, topicSlug: String) {
        runCoreAction {
            val topic = requireDbaWorldTopic(courseCode, topicSlug)
            returnToDbaWorldAfterLearning = true
            pendingCourse = null
            pendingTopic = topic
            pendingMistakeReview = false
            screen = AppScreen.Limit
        }
    }

    fun openDbaWorldKnowledgeLab(courseCode: String, topicSlug: String) {
        runCoreAction {
            val topic = requireDbaWorldTopic(courseCode, topicSlug)
            val lab = labs.firstOrNull { candidate ->
                candidate.courseCode == courseCode && candidate.labNumber == topic.topicNumber
            } ?: error("No lab for ${topic.title}")
            returnToDbaWorldAfterLearning = true
            selectedLabCourse = courseCode
            selectedLab = lab
            screen = AppScreen.LabDetail
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

    fun topicXpOpportunity(topicId: Long): MobileQuizXpOpportunity? =
        activityStats
            ?.quizXpOpportunities
            ?.firstOrNull { opportunity ->
                opportunity.scopeKind == MobileQuizXpScopeKind.TOPIC &&
                    opportunity.topicId == topicId
            }

    fun courseXpOpportunity(courseCode: String): MobileQuizXpOpportunity? =
        activityStats
            ?.quizXpOpportunities
            ?.firstOrNull { opportunity ->
                opportunity.scopeKind == MobileQuizXpScopeKind.COURSE &&
                    opportunity.courseCode == courseCode
            }

    fun allTopicsXpOpportunity(): MobileQuizXpOpportunity? =
        activityStats
            ?.quizXpOpportunities
            ?.firstOrNull { opportunity ->
                opportunity.scopeKind == MobileQuizXpScopeKind.ALL_TOPICS
            }

    fun questionXpAvailable(questionId: Long): Boolean =
        activityStats?.questionXpAvailableIds?.contains(questionId) == true

    fun topicXpAvailable(topicId: Long): ULong =
        topicXpOpportunity(topicId)?.totalAvailableXp ?: 0UL

    fun courseXpAvailable(courseCode: String): ULong =
        courseXpOpportunity(courseCode)?.totalAvailableXp ?: 0UL

    fun topicCourseXpAvailable(courseCode: String): ULong =
        topics
            .asSequence()
            .filter { topic -> topic.courseCode == courseCode }
            .sumOf { topic -> topicXpAvailable(topic.id) }

    val pendingQuizXpOpportunity: MobileQuizXpOpportunity?
        get() {
            if (pendingMistakeReview) {
                return null
            }

            return when {
                pendingTopic != null -> topicXpOpportunity(pendingTopic!!.id)
                pendingCourse != null -> courseXpOpportunity(pendingCourse!!)
                else -> allTopicsXpOpportunity()
            }
        }

    fun openNotes() {
        selectedNotesCourse = null
        selectedNoteTopic = null
        selectedNoteBody = ""
        screen = AppScreen.NotesCourses
    }

    fun chooseNotesCourse(courseCode: String) {
        selectedNotesCourse = courseCode
        selectedNoteTopic = null
        selectedNoteBody = ""
        screen = AppScreen.NotesTopics
    }

    fun openNote(topic: MobileTopic) {
        runCoreAction {
            selectedNoteTopic = topic
            selectedNoteBody = requireTrainer().topicNote(topic.slug, language.code)
            screen = AppScreen.NoteDetail
        }
    }

    fun notesPartLabel(part: Long): String = when (part) {
        1L -> strings.notesSimple
        2L -> strings.notesMiddle
        3L -> strings.notesStrong
        else -> "Part $part"
    }

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

    fun openUnfinishedQuizzes() {
        runCoreAction {
            unfinishedSessions = requireTrainer().unfinishedSessions()
            screen = AppScreen.UnfinishedQuizzes
        }
    }

    fun continueSavedSession(progress: MobileSessionProgress) {
        runCoreAction {
            pendingMistakeReview = progress.isReview
            pendingCourse = progress.courseCode
            pendingTopic = progress.topicId?.let { topicId ->
                topics.firstOrNull { topic -> topic.id == topicId }
            }

            activeQuestionLimit = progress.questionLimit
            preferences
                .edit()
                .putString("active_question_limit", progress.questionLimit.name)
                .apply()

            sessionId = progress.id
            answeredQuestions = progress.answeredQuestions.toInt()
            correctAnswers = progress.correctAnswers.toInt()
            totalQuestions = progress.totalQuestions.toInt()
            feedback = null
            selectedOptionIds.clear()
            loadCurrentQuestion()
        }
    }

    fun requestDiscardSavedSession(progress: MobileSessionProgress) {
        pendingDiscardSession = progress
    }

    fun dismissDiscardSavedSession() {
        pendingDiscardSession = null
    }

    fun confirmDiscardSavedSession() {
        val progress = pendingDiscardSession ?: return

        runCoreAction {
            requireTrainer().cancelSession(progress.id)
            activityStats = requireTrainer().activityStats()
            unfinishedSessions = requireTrainer().unfinishedSessions()
            pendingDiscardSession = null
            if (unfinishedSessions.isEmpty()) {
                screen = AppScreen.Home
            }
        }
    }

    val unfinishedSessionsForPendingQuiz: List<MobileSessionProgress>
        get() = unfinishedSessions.filter { progress ->
            if (progress.isReview != pendingMistakeReview) {
                return@filter false
            }

            when {
                pendingTopic != null -> progress.topicId == pendingTopic!!.id
                pendingCourse != null -> progress.topicId == null && progress.courseCode == pendingCourse
                else -> progress.topicId == null && progress.courseCode == null
            }
        }

    fun sessionTitle(progress: MobileSessionProgress): String {
        val scopeTitle = progress.topicId?.let { topicId ->
            topics.firstOrNull { topic -> topic.id == topicId }?.let { topic ->
                "${topic.courseCode.uppercase()} → ${topic.topicNumber}. ${topic.title}"
            }
        } ?: progress.courseCode?.uppercase() ?: strings.generalQuiz

        return if (progress.isReview) {
            "${strings.mistakes}: $scopeTitle"
        } else {
            scopeTitle
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

            activityStats = requireTrainer().activityStats()
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

    fun saveAndExitQuiz() {
        if (sessionId == null) {
            back()
            return
        }

        runCoreAction {
            showCancelDialog = false
            activityStats = requireTrainer().activityStats()
            resetQuizState()
            unfinishedSessions = requireTrainer().unfinishedSessions()
            if (returnToDbaWorldAfterLearning) {
                refreshDbaWorldKnowledge()
                returnToDbaWorldAfterLearning = false
                screen = AppScreen.DbaWorld
            } else {
                screen = AppScreen.Home
            }
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
            unfinishedSessions = requireTrainer().unfinishedSessions()
            if (returnToDbaWorldAfterLearning) {
                refreshDbaWorldKnowledge()
                returnToDbaWorldAfterLearning = false
                screen = AppScreen.DbaWorld
            } else {
                screen = AppScreen.Home
            }
        }
    }

    fun finishResults() {
        val wasMistakeReview = pendingMistakeReview
        val mistakeReturnPage = when {
            pendingTopic != null -> StatisticsPage.MistakeQuestions
            pendingCourse != null -> StatisticsPage.MistakeTopics
            else -> StatisticsPage.Mistakes
        }

        val returnToWorld = returnToDbaWorldAfterLearning
        resetQuizState()

        if (returnToWorld) {
            runCoreAction {
                unfinishedSessions = requireTrainer().unfinishedSessions()
                refreshDbaWorldKnowledge()
                returnToDbaWorldAfterLearning = false
                screen = AppScreen.DbaWorld
            }
        } else if (wasMistakeReview) {
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
        statisticsDetailLimit = null
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
        statisticsDetailLimit = row.limit
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
        statisticsDetailLimit = row.limit
        statisticsDetailReturnPage = StatisticsPage.ScopeModes
        statisticsPage = StatisticsPage.Detail
    }

    fun requestClearCurrentQuestionModeStatistics() {
        val limit = statisticsDetailLimit ?: return
        if (limit == MobileStatisticsLimit.ANY) {
            return
        }

        val title = statisticsDetailTitle
        pendingStatisticsReset = when {
            selectedStatisticsTopic != null -> StatisticsResetTarget.TopicMode(
                topic = selectedStatisticsTopic!!,
                limit = limit,
                title = title,
            )
            selectedStatisticsCourse != null -> StatisticsResetTarget.CourseMode(
                courseCode = selectedStatisticsCourse!!,
                limit = limit,
                title = title,
            )
            else -> StatisticsResetTarget.GlobalMode(
                limit = limit,
                title = title,
            )
        }
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

    fun openPracticeStatistics() {
        runCoreAction {
            refreshPracticeState()
            statisticsPage = StatisticsPage.Practice
        }
    }

    fun requestClearStatistics() {
        pendingStatisticsReset = StatisticsResetTarget.All
    }

    fun requestClearCurrentStatisticsScope() {
        val topic = selectedStatisticsTopic
        val courseCode = selectedStatisticsCourse

        pendingStatisticsReset = when {
            topic != null -> StatisticsResetTarget.Topic(topic)
            courseCode != null -> StatisticsResetTarget.Course(courseCode)
            else -> return
        }
    }

    fun dismissStatisticsReset() {
        pendingStatisticsReset = null
    }

    fun confirmStatisticsReset() {
        val target = pendingStatisticsReset ?: return

        runCoreAction {
            when (target) {
                StatisticsResetTarget.All -> requireTrainer().clearStatistics()
                is StatisticsResetTarget.Course ->
                    requireTrainer().clearStatisticsCourse(target.courseCode)
                is StatisticsResetTarget.Topic ->
                    requireTrainer().clearStatisticsTopic(target.topic.id)
                is StatisticsResetTarget.GlobalMode ->
                    requireTrainer().clearStatisticsLimit(target.limit)
                is StatisticsResetTarget.CourseMode ->
                    requireTrainer().clearStatisticsCourseLimit(
                        target.courseCode,
                        target.limit,
                    )
                is StatisticsResetTarget.TopicMode ->
                    requireTrainer().clearStatisticsTopicLimit(
                        target.topic.id,
                        target.limit,
                    )
            }

            pendingStatisticsReset = null
            refreshStatisticsOverview()
            mistakeSummaries = requireTrainer().mistakeSummaries()
            mistakeDetail = null
            activityStats = requireTrainer().activityStats()
            unfinishedSessions = requireTrainer().unfinishedSessions()
            refreshPracticeState()

            when (target) {
                StatisticsResetTarget.All -> resetStatisticsNavigation()
                is StatisticsResetTarget.Course -> {
                    selectedStatisticsCourse = target.courseCode
                    selectedStatisticsTopic = null
                    topicStatistics = topics
                        .filter { topic -> topic.courseCode == target.courseCode }
                        .map { topic ->
                            TopicStatisticsRow(
                                topic = topic,
                                stats = requireTrainer().statisticsTopic(
                                    topic.id,
                                    MobileStatisticsLimit.ANY,
                                ),
                            )
                        }
                    scopeModeStatistics = loadModeStatistics { limit ->
                        requireTrainer().statisticsCourse(target.courseCode, limit)
                    }
                }
                is StatisticsResetTarget.Topic -> {
                    selectedStatisticsCourse = target.topic.courseCode
                    selectedStatisticsTopic = target.topic
                    topicStatistics = topics
                        .filter { topic -> topic.courseCode == target.topic.courseCode }
                        .map { topic ->
                            TopicStatisticsRow(
                                topic = topic,
                                stats = requireTrainer().statisticsTopic(
                                    topic.id,
                                    MobileStatisticsLimit.ANY,
                                ),
                            )
                        }
                    scopeModeStatistics = loadModeStatistics { limit ->
                        requireTrainer().statisticsTopic(target.topic.id, limit)
                    }
                }
                is StatisticsResetTarget.GlobalMode -> {
                    selectedStatisticsCourse = null
                    selectedStatisticsTopic = null
                    statisticsDetail = requireTrainer().statisticsAll(target.limit)
                    statisticsDetailTitle = target.title
                    statisticsDetailLimit = target.limit
                    statisticsDetailReturnPage = StatisticsPage.GlobalModes
                    statisticsPage = StatisticsPage.Detail
                }
                is StatisticsResetTarget.CourseMode -> {
                    selectedStatisticsCourse = target.courseCode
                    selectedStatisticsTopic = null
                    topicStatistics = topics
                        .filter { topic -> topic.courseCode == target.courseCode }
                        .map { topic ->
                            TopicStatisticsRow(
                                topic = topic,
                                stats = requireTrainer().statisticsTopic(
                                    topic.id,
                                    MobileStatisticsLimit.ANY,
                                ),
                            )
                        }
                    scopeModeStatistics = loadModeStatistics { limit ->
                        requireTrainer().statisticsCourse(target.courseCode, limit)
                    }
                    statisticsDetail = requireTrainer().statisticsCourse(
                        target.courseCode,
                        target.limit,
                    )
                    statisticsDetailTitle = target.title
                    statisticsDetailLimit = target.limit
                    statisticsDetailReturnPage = StatisticsPage.ScopeModes
                    statisticsPage = StatisticsPage.Detail
                }
                is StatisticsResetTarget.TopicMode -> {
                    selectedStatisticsCourse = target.topic.courseCode
                    selectedStatisticsTopic = target.topic
                    topicStatistics = topics
                        .filter { topic -> topic.courseCode == target.topic.courseCode }
                        .map { topic ->
                            TopicStatisticsRow(
                                topic = topic,
                                stats = requireTrainer().statisticsTopic(
                                    topic.id,
                                    MobileStatisticsLimit.ANY,
                                ),
                            )
                        }
                    scopeModeStatistics = loadModeStatistics { limit ->
                        requireTrainer().statisticsTopic(target.topic.id, limit)
                    }
                    statisticsDetail = requireTrainer().statisticsTopic(
                        target.topic.id,
                        target.limit,
                    )
                    statisticsDetailTitle = target.title
                    statisticsDetailLimit = target.limit
                    statisticsDetailReturnPage = StatisticsPage.ScopeModes
                    statisticsPage = StatisticsPage.Detail
                }
            }
        }
    }

    fun dismissError() {
        errorMessage = null
    }

    fun back() {
        when (screen) {
            AppScreen.Home -> Unit
            AppScreen.DbaWorld -> {
                if (!dbaWorld.back()) {
                    screen = AppScreen.Home
                }
            }
            AppScreen.UnfinishedQuizzes -> screen = AppScreen.Home
            AppScreen.TopicCourses -> screen = AppScreen.Home
            AppScreen.Topics -> screen = AppScreen.TopicCourses
            AppScreen.NotesCourses -> {
                selectedNotesCourse = null
                screen = AppScreen.Home
            }
            AppScreen.NotesTopics -> {
                selectedNoteTopic = null
                selectedNoteBody = ""
                selectedNotesCourse = null
                screen = AppScreen.NotesCourses
            }
            AppScreen.NoteDetail -> {
                selectedNoteTopic = null
                selectedNoteBody = ""
                if (returnToDbaWorldAfterLearning) {
                    selectedNotesCourse = null
                    runCoreAction {
                        refreshDbaWorldKnowledge()
                        returnToDbaWorldAfterLearning = false
                        screen = AppScreen.DbaWorld
                    }
                } else {
                    screen = AppScreen.NotesTopics
                }
            }
            AppScreen.Courses -> screen = AppScreen.Home
            AppScreen.LabCourses -> screen = AppScreen.Home
            AppScreen.Labs -> {
                selectedLab = null
                screen = AppScreen.LabCourses
            }
            AppScreen.LabDetail -> {
                selectedLab = null
                if (returnToDbaWorldAfterLearning) {
                    selectedLabCourse = null
                    runCoreAction {
                        refreshDbaWorldKnowledge()
                        returnToDbaWorldAfterLearning = false
                        screen = AppScreen.DbaWorld
                    }
                } else {
                    screen = AppScreen.Labs
                }
            }
            AppScreen.PracticeCategories -> screen = AppScreen.Home
            AppScreen.PracticeTasks -> {
                selectedPracticeCategory = null
                screen = AppScreen.PracticeCategories
            }
            AppScreen.PracticeWorkbench -> practiceBack()
            AppScreen.Limit -> {
                if (returnToDbaWorldAfterLearning) {
                    pendingTopic = null
                    pendingCourse = null
                    pendingMistakeReview = false
                    runCoreAction {
                        refreshDbaWorldKnowledge()
                        returnToDbaWorldAfterLearning = false
                        screen = AppScreen.DbaWorld
                    }
                } else {
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
            StatisticsPage.Practice -> statisticsPage = StatisticsPage.Root
            StatisticsPage.Detail -> statisticsPage = statisticsDetailReturnPage
        }
    }

    private fun practiceBack() {
        when (practicePane) {
            PracticePane.Data -> practicePane = PracticePane.Schema
            PracticePane.Schema,
            PracticePane.Result,
            PracticePane.Learning -> practicePane = PracticePane.Editor
            PracticePane.Editor -> {
                savePracticeDraft()
                resetPracticeWorkbench()
                screen = AppScreen.PracticeTasks
            }
        }
    }

    private fun savePracticeDraft() {
        val task = currentPracticeTask ?: return
        runCoreAction {
            requireTrainer().practiceSaveDraft(task.id, practiceSql)
            refreshPracticeState()
        }
    }

    private fun refreshPracticeState() {
        val trainer = requireTrainer()
        practiceTasks = trainer.practiceTasks()
        practiceProgress = trainer.practiceProgress()
        practiceStats = trainer.practiceStats()
    }

    private fun resetPracticeWorkbench() {
        currentPracticeTask = null
        practiceSql = ""
        practiceSchema = null
        practiceTableData = null
        practiceOutput = null
        practiceCheck = null
        practicePane = PracticePane.Editor
        practiceHintCount = 0
        practiceShowSolution = false
        practiceMessage = null
        practiceError = null
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
            unfinishedSessions = requireTrainer().unfinishedSessions()
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
        statisticsDetailLimit = null
        mistakeDetail = null
        selectedMistakeCourse = null
        selectedMistakeTopic = null
        statisticsDetailReturnPage = StatisticsPage.Root
    }

    private fun requireDbaWorldTopic(courseCode: String, topicSlug: String): MobileTopic =
        topics.firstOrNull { topic ->
            topic.courseCode == courseCode && topic.slug == topicSlug
        } ?: error("DBA World topic not found: $courseCode/$topicSlug")

    private fun buildDbaWorldKnowledgeSnapshot(): KnowledgeSnapshot {
        val trainer = requireTrainer()
        val progressByKey = topics
            .map { topic ->
                val mastery = trainer.topicMastery(topic.id)
                val unfinished = unfinishedSessions.any { session ->
                    !session.isReview && session.topicId == topic.id
                }
                val started = mastery.completedAttempts > 0UL || unfinished
                val progress = KnowledgeTopicProgress(
                    courseCode = topic.courseCode,
                    topicSlug = topic.slug,
                    topicNumber = topic.topicNumber,
                    title = topic.title,
                    started = started,
                    passed = mastery.passed,
                    completedAttempts = mastery.completedAttempts.toLong(),
                    bestAccuracyPercent = mastery.bestAccuracyPercent,
                    hasLab = labs.any { lab ->
                        lab.courseCode == topic.courseCode && lab.labNumber == topic.topicNumber
                    },
                )
                KnowledgeSnapshot.topicKey(topic.courseCode, topic.slug) to progress
            }
            .toMap()

        return KnowledgeSnapshot(progressByKey)
    }

    private fun refreshDbaWorldKnowledge() {
        unfinishedSessions = requireTrainer().unfinishedSessions()
        dbaWorld.updateKnowledge(buildDbaWorldKnowledgeSnapshot())
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
        const val PRACTICE_TABLE_PREVIEW_LIMIT = 1_000L

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
