package dev.dbatrainer.app.ui

import androidx.activity.compose.BackHandler
import androidx.compose.animation.core.RepeatMode
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.infiniteRepeatable
import androidx.compose.animation.core.rememberInfiniteTransition
import androidx.compose.animation.core.tween
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.ScrollState
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.Checkbox
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.RadioButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.Alignment
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import dev.dbatrainer.app.AppLaunchMode
import dev.dbatrainer.app.AppScreen
import dev.dbatrainer.app.CourseStatisticsRow
import dev.dbatrainer.app.ModeStatisticsRow
import dev.dbatrainer.app.StatisticsPage
import dev.dbatrainer.app.StatisticsResetTarget
import dev.dbatrainer.app.TopicStatisticsRow
import dev.dbatrainer.app.TrainerController
import dev.dbatrainer.app.UiLanguage
import dev.dbatrainer.app.UiStrings
import dev.dbatrainer.app.uiStrings
import dev.dbatrainer.app.game3d.rememberDbaWorldLauncher
import dev.dbatrainer.ffi.MobileActivityStats
import dev.dbatrainer.ffi.MobileAchievementProgress
import dev.dbatrainer.ffi.MobileAnswerResult
import dev.dbatrainer.ffi.MobileLab
import dev.dbatrainer.ffi.MobileQuestion
import dev.dbatrainer.ffi.MobileQuestionLimit
import dev.dbatrainer.ffi.MobileQuestionMistakeSummary
import dev.dbatrainer.ffi.MobileQuizXpOpportunity
import dev.dbatrainer.ffi.MobileSessionProgress
import dev.dbatrainer.ffi.MobileStatisticsLimit
import dev.dbatrainer.ffi.MobileQuestionType
import dev.dbatrainer.ffi.MobileSource
import dev.dbatrainer.ffi.MobileTopic
import dev.dbatrainer.ffi.MobileTrainingStats
import dev.dbatrainer.ffi.coreVersion
import kotlin.math.cos
import kotlin.math.sin

private val LocalUiStrings = staticCompositionLocalOf {
    uiStrings(UiLanguage.English)
}

@Composable
fun TrainerApp(controller: TrainerController) {
    CompositionLocalProvider(
        LocalUiStrings provides controller.strings,
    ) {
        val strings = LocalUiStrings.current

    BackHandler(
        enabled = controller.canGoBack,
    ) {
        controller.back()
    }

    Surface(
        modifier = Modifier.fillMaxSize(),
        color = MaterialTheme.colorScheme.background,
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .safeDrawingPadding(),
        ) {
            when (controller.screen) {
                AppScreen.ModeSelection -> ModeSelectionScreen(controller)
                AppScreen.Home -> HomeScreen(controller)
                AppScreen.Settings -> SettingsScreen(controller)
                AppScreen.UnfinishedQuizzes -> UnfinishedQuizzesScreen(controller)
                AppScreen.TopicCourses -> TopicCoursesScreen(controller)
                AppScreen.Topics -> TopicsScreen(controller)
                AppScreen.NotesCourses -> NotesCoursesScreen(controller)
                AppScreen.NotesTopics -> NotesTopicsScreen(controller)
                AppScreen.NoteDetail -> NoteDetailScreen(controller)
                AppScreen.Courses -> CoursesScreen(controller)
                AppScreen.Limit -> LimitScreen(controller)
                AppScreen.LabCourses -> LabCoursesScreen(controller)
                AppScreen.Labs -> LabsScreen(controller)
                AppScreen.LabDetail -> LabDetailScreen(controller)
                AppScreen.PracticeCategories -> PracticeCategoriesScreen(controller)
                AppScreen.PracticeTasks -> PracticeTasksScreen(controller)
                AppScreen.PracticeWorkbench -> PracticeWorkbenchScreen(controller)
                AppScreen.Quiz -> QuizScreen(controller)
                AppScreen.Feedback -> FeedbackScreen(controller)
                AppScreen.Results -> ResultsScreen(controller)
                AppScreen.Statistics -> StatisticsScreen(controller)
                AppScreen.DbaWorld -> DbaWorld3DLauncherScreen(controller)
            }
        }
    }

    if (controller.newlyUnlockedAchievements.isNotEmpty()) {
        AchievementCelebrationOverlay(
            achievements = controller.newlyUnlockedAchievements,
            onDismiss = controller::dismissAchievementCelebration,
        )
    }

    if (controller.showCancelDialog) {
        AlertDialog(
            onDismissRequest = controller::dismissCancelDialog,
            title = {
                Text(strings.cancelQuizTitle)
            },
            text = {
                Text(strings.cancelQuizMessage)
            },
            confirmButton = {
                Column(
                    modifier = Modifier.fillMaxWidth(),
                    verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
                ) {
                    Button(
                        modifier = Modifier.fillMaxWidth(),
                        onClick = controller::saveAndExitQuiz,
                    ) {
                        Text(strings.saveAndExitQuiz)
                    }
                    OutlinedButton(
                        modifier = Modifier.fillMaxWidth(),
                        onClick = controller::confirmCancelQuiz,
                    ) {
                        Text(strings.cancelQuiz)
                    }
                    TextButton(
                        modifier = Modifier.fillMaxWidth(),
                        onClick = controller::dismissCancelDialog,
                    ) {
                        Text(strings.stayInQuiz)
                    }
                }
            },
        )
    }

    controller.pendingStatisticsReset?.let { target ->
        val scopeLabel = when (target) {
            StatisticsResetTarget.All -> strings.overallStatistics
            is StatisticsResetTarget.Course -> target.courseCode.uppercase()
            is StatisticsResetTarget.Topic -> target.topic.title
            is StatisticsResetTarget.GlobalMode -> target.title
            is StatisticsResetTarget.CourseMode -> target.title
            is StatisticsResetTarget.TopicMode -> target.title
        }

        AlertDialog(
            onDismissRequest = controller::dismissStatisticsReset,
            title = {
                Text(strings.clearStatisticsConfirmTitle)
            },
            text = {
                Text("${strings.clearStatisticsConfirmMessage}\n\n$scopeLabel")
            },
            confirmButton = {
                Button(
                    onClick = controller::confirmStatisticsReset,
                ) {
                    Text(strings.clearStatisticsConfirm)
                }
            },
            dismissButton = {
                TextButton(
                    onClick = controller::dismissStatisticsReset,
                ) {
                    Text(strings.back)
                }
            },
        )
    }

    controller.pendingDiscardSession?.let { session ->
        AlertDialog(
            onDismissRequest = controller::dismissDiscardSavedSession,
            title = {
                Text(strings.cancelQuiz)
            },
            text = {
                Text(
                    "${strings.interruptQuizMessage}\n\n${controller.sessionTitle(session)}",
                )
            },
            confirmButton = {
                Button(
                    onClick = controller::confirmDiscardSavedSession,
                ) {
                    Text(strings.cancelQuiz)
                }
            },
            dismissButton = {
                TextButton(
                    onClick = controller::dismissDiscardSavedSession,
                ) {
                    Text(strings.back)
                }
            },
        )
    }

    controller.errorMessage?.let { message ->
        AlertDialog(
            onDismissRequest = controller::dismissError,
            title = {
                Text(strings.error)
            },
            text = {
                Text(message)
            },
            confirmButton = {
                TextButton(
                    onClick = controller::dismissError,
                ) {
                    Text("OK")
                }
            },
        )
    }
    }
}

@Composable
private fun DbaWorld3DLauncherScreen(controller: TrainerController) {
    val launcher = rememberDbaWorldLauncher(
        onReturned = controller::openModeSelection,
    )

    LaunchedEffect(Unit) {
        launcher.launch()
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(24.dp),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text(
            text = "DBA World 3D",
            style = MaterialTheme.typography.headlineMedium,
            fontWeight = FontWeight.Bold,
        )
        Spacer(modifier = Modifier.height(12.dp))
        Text(
            text = if (controller.language == UiLanguage.Russian) {
                "Запускается 3D-прототип на Godot. После выхода вы вернётесь к выбору режима."
            } else {
                "Launching the Godot 3D prototype. Leaving the game returns to mode selection."
            },
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Spacer(modifier = Modifier.height(20.dp))
        Button(
            onClick = launcher::launch,
            enabled = !launcher.isRunning,
        ) {
            Text(if (controller.language == UiLanguage.Russian) "Запустить 3D мир" else "Launch 3D world")
        }
        TextButton(onClick = controller::openModeSelection) {
            Text(if (controller.language == UiLanguage.Russian) "Назад к выбору режима" else "Back to mode selection")
        }
    }
}

@Composable
private fun ModeSelectionScreen(controller: TrainerController) {
    val russian = controller.language == UiLanguage.Russian

    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(
            horizontal = LayoutSpacing.screenHorizontal,
            vertical = LayoutSpacing.screenVertical,
        ),
        verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
    ) {
        item {
            Text(
                text = "DBA Trainer",
                style = MaterialTheme.typography.headlineLarge,
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = if (russian) {
                    "Выберите режим. Прогресс обучения и DBA World используют общие данные."
                } else {
                    "Choose a mode. Learning progress and DBA World share data."
                },
                style = MaterialTheme.typography.bodyLarge,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }

        item {
            ModeChoiceCard(
                title = if (russian) "Обучение" else "Learning",
                subtitle = if (russian) {
                    "Курсы, конспекты, тесты, лабораторные и статистика."
                } else {
                    "Courses, notes, quizzes, labs, and statistics."
                },
                action = if (russian) "Перейти к обучению" else "Open Learning",
                onClick = controller::selectLearningMode,
            )
        }

        item {
            ModeChoiceCard(
                title = "DBA World",
                subtitle = if (russian) {
                    "3D-режим: работа инженера, инциденты, инфраструктура и карьера."
                } else {
                    "3D game mode: engineering work, incidents, infrastructure, and career."
                },
                action = if (russian) "Запустить DBA World 3D" else "Launch DBA World 3D",
                onClick = controller::selectDbaWorldMode,
            )
        }

        item {
            OutlinedButton(
                onClick = controller::openSettings,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(if (russian) "Настройки" else "Settings")
            }
        }
    }
}

@Composable
private fun ModeChoiceCard(
    title: String,
    subtitle: String,
    action: String,
    onClick: () -> Unit,
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Column(
            modifier = Modifier.padding(LayoutSpacing.card),
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            Text(
                text = title,
                style = MaterialTheme.typography.headlineSmall,
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = subtitle,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            Button(
                onClick = onClick,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(action)
            }
        }
    }
}

@Composable
private fun SettingsScreen(controller: TrainerController) {
    val russian = controller.language == UiLanguage.Russian

    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(
            horizontal = LayoutSpacing.screenHorizontal,
            vertical = LayoutSpacing.screenVertical,
        ),
        verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
    ) {
        item {
            Text(
                text = if (russian) "Настройки" else "Settings",
                style = MaterialTheme.typography.headlineLarge,
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = if (russian) {
                    "Режим запуска можно изменить в любой момент. Это не меняет прогресс обучения или состояние DBA World."
                } else {
                    "You can change the startup mode at any time. This does not reset learning progress or DBA World state."
                },
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }

        item {
            Text(
                text = if (russian) "При запуске приложения" else "On app launch",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
        }

        item {
            StartupModeRow(
                title = if (russian) "Спрашивать каждый раз" else "Ask every time",
                subtitle = if (russian) "Показывать выбор Обучение / DBA World." else "Show the Learning / DBA World chooser.",
                selected = controller.startupMode == AppLaunchMode.ASK,
                onClick = { controller.changeStartupMode(AppLaunchMode.ASK) },
            )
            StartupModeRow(
                title = if (russian) "Обучение" else "Learning",
                subtitle = if (russian) "Сразу открывать учебный режим." else "Open Learning immediately.",
                selected = controller.startupMode == AppLaunchMode.LEARNING,
                onClick = { controller.changeStartupMode(AppLaunchMode.LEARNING) },
            )
            StartupModeRow(
                title = "DBA World",
                subtitle = if (russian) "Сразу открывать игровой режим." else "Open the game mode immediately.",
                selected = controller.startupMode == AppLaunchMode.DBA_WORLD,
                onClick = { controller.changeStartupMode(AppLaunchMode.DBA_WORLD) },
            )
        }

        item {
            Text(
                text = controller.strings.language,
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                LanguageButton(
                    text = "EN",
                    selected = controller.language == UiLanguage.English,
                    onClick = { controller.changeLanguage(UiLanguage.English) },
                )
                LanguageButton(
                    text = "RU",
                    selected = controller.language == UiLanguage.Russian,
                    onClick = { controller.changeLanguage(UiLanguage.Russian) },
                )
            }
        }

        item {
            OutlinedButton(
                onClick = controller::openModeSelection,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(if (russian) "← К выбору режима" else "← Back to mode selection")
            }
        }
    }
}

@Composable
private fun StartupModeRow(
    title: String,
    subtitle: String,
    selected: Boolean,
    onClick: () -> Unit,
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick)
            .padding(vertical = 6.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        RadioButton(
            selected = selected,
            onClick = onClick,
        )
        Column(modifier = Modifier.weight(1f)) {
            Text(title, fontWeight = FontWeight.SemiBold)
            Text(
                text = subtitle,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
private fun HomeScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(
            horizontal = LayoutSpacing.screenHorizontal,
            vertical = LayoutSpacing.screenVertical,
        ),
        verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
    ) {
        item {
            Text(
                text = if (controller.language == UiLanguage.Russian) "Обучение" else "Learning",
                style = MaterialTheme.typography.headlineLarge,
                fontWeight = FontWeight.Bold,
            )
        }

        controller.activityStats?.let { activity ->
            item {
                RankSummaryCard(activity)
            }
        }

        item {
            Button(
                onClick = controller::openTopics,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(strings.topicQuiz)
            }
        }

        item {
            OutlinedButton(
                onClick = controller::openNotes,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(strings.notes)
            }
        }

        item {
            OutlinedButton(
                onClick = controller::openGeneralQuiz,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(strings.generalQuiz)
            }
        }

        if (controller.unfinishedSessions.isNotEmpty()) {
            item {
                OutlinedButton(
                    onClick = controller::openUnfinishedQuizzes,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text("${strings.unfinishedQuizzes} (${controller.unfinishedSessions.size})")
                }
            }
        }

        item {
            OutlinedButton(
                onClick = controller::openLabs,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(strings.labs)
            }
        }

        item {
            OutlinedButton(
                onClick = controller::openPractice,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(strings.practice)
            }
        }

        item {
            OutlinedButton(
                onClick = controller::openStatistics,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(strings.statistics)
            }
        }

        item {
            OutlinedButton(
                onClick = controller::openSettings,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(if (controller.language == UiLanguage.Russian) "Настройки" else "Settings")
            }
        }

        item {
            Spacer(Modifier.height(8.dp))
            Text(
                text = "${strings.rustCore} ${coreVersion()}",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
private fun UnfinishedQuizzesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val sessions = controller.unfinishedSessions

    TrainerScaffold(
        title = strings.unfinishedQuizzes,
        onBack = controller::back,
    ) { modifier ->
        if (sessions.isEmpty()) {
            EmptyMessage(
                text = strings.noUnfinishedQuizzes,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            items(
                items = sessions,
                key = { session -> session.id },
            ) { session ->
                UnfinishedQuizCard(
                    controller = controller,
                    session = session,
                    onContinue = { controller.continueSavedSession(session) },
                    onCancel = { controller.requestDiscardSavedSession(session) },
                )
            }
        }
    }
}

@Composable
private fun UnfinishedQuizCard(
    controller: TrainerController,
    session: MobileSessionProgress,
    onContinue: () -> Unit,
    onCancel: () -> Unit,
) {
    val strings = LocalUiStrings.current

    Card(
        modifier = Modifier.fillMaxWidth(),
    ) {
        Column(
            modifier = Modifier.padding(LayoutSpacing.card),
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            Text(
                text = controller.sessionTitle(session),
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
            Text(
                text = "${strings.answeredQuestions}: ${session.answeredQuestions} / ${session.totalQuestions} · ${controller.questionLimitTitle(session.questionLimit)}",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            LinearProgressIndicator(
                progress = {
                    if (session.totalQuestions == 0UL) 0f
                    else session.answeredQuestions.toFloat() / session.totalQuestions.toFloat()
                },
                modifier = Modifier.fillMaxWidth(),
            )
            Row(
                horizontalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
            ) {
                Button(onClick = onContinue) {
                    Text(strings.continueQuiz)
                }
                TextButton(onClick = onCancel) {
                    Text(strings.cancelQuiz)
                }
            }
        }
    }
}

@Composable
private fun LanguageButton(
    text: String,
    selected: Boolean,
    onClick: () -> Unit,
) {
    if (selected) {
        Button(onClick = onClick) {
            Text(text)
        }
    } else {
        OutlinedButton(onClick = onClick) {
            Text(text)
        }
    }
}

@Composable
private fun TopicCoursesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    CourseSelectionScreen(
        controller = controller,
        title = strings.topicQuiz,
        prompt = strings.chooseTopicCourse,
        onChoose = controller::chooseTopicCourse,
        xpForCourse = controller::topicCourseXpAvailable,
    )
}

@Composable
private fun TopicsScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val courseCode = controller.selectedTopicCourse

    TrainerScaffold(
        title = if (courseCode == null) {
            strings.topics
        } else {
            "${strings.topics} → ${courseCode.uppercase()}"
        },
        onBack = controller::back,
    ) { modifier ->
        val topics = controller.topicsForSelectedCourse

        if (topics.isEmpty()) {
            EmptyMessage(
                text = strings.topicsEmpty,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            itemsIndexed(
                items = topics,
                key = { _, topic -> topic.id },
            ) { index, topic ->
                TopicCard(
                    topic = topic,
                    displayNumber = topicDisplayNumber(topics, index),
                    masterySummary = strings.topicMasterySummary(
                        controller.topicMasteryFor(topic.id),
                    ),
                    xpAvailable = controller.topicXpAvailable(topic.id),
                    onClick = {
                        controller.chooseTopic(topic)
                    },
                )
            }
        }
    }
}

@Composable
private fun NotesCoursesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    CourseSelectionScreen(
        controller = controller,
        title = strings.notes,
        prompt = strings.chooseNotesCourse,
        onChoose = controller::chooseNotesCourse,
        xpForCourse = null,
    )
}

@Composable
private fun NotesTopicsScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val courseCode = controller.selectedNotesCourse
    val topics = controller.topicsForSelectedNotesCourse

    TrainerScaffold(
        title = if (courseCode == null) {
            strings.notes
        } else {
            "${strings.notes} → ${courseCode.uppercase()}"
        },
        onBack = controller::back,
    ) { modifier ->
        if (topics.isEmpty()) {
            EmptyMessage(
                text = strings.notesEmpty,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        val showLevelGroups = topics.all { topic -> topic.notesPart in 1L..3L }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            if (showLevelGroups) {
                topics
                    .groupBy { topic -> topic.notesPart }
                    .forEach { (part, partTopics) ->
                        item(key = "notes-part-$part") {
                            Text(
                                text = controller.notesPartLabel(part),
                                style = MaterialTheme.typography.titleMedium,
                                fontWeight = FontWeight.Bold,
                                color = MaterialTheme.colorScheme.primary,
                            )
                        }

                        items(
                            items = partTopics,
                            key = { topic -> topic.id },
                        ) { topic ->
                            val index = topics.indexOfFirst { candidate -> candidate.id == topic.id }
                            NoteTopicCard(
                                topic = topic,
                                displayNumber = topicDisplayNumber(topics, index),
                                onClick = { controller.openNote(topic) },
                            )
                        }
                    }
            } else {
                items(
                    items = topics,
                    key = { topic -> topic.id },
                ) { topic ->
                    val index = topics.indexOfFirst { candidate -> candidate.id == topic.id }
                    NoteTopicCard(
                        topic = topic,
                        displayNumber = topicDisplayNumber(topics, index),
                        onClick = { controller.openNote(topic) },
                    )
                }
            }
        }
    }
}

@Composable
private fun NoteTopicCard(
    topic: MobileTopic,
    displayNumber: String,
    onClick: () -> Unit,
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Column(
            modifier = Modifier.padding(LayoutSpacing.card),
            verticalArrangement = Arrangement.spacedBy(6.dp),
        ) {
            Text(
                text = "$displayNumber. ${topic.title}",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
            topic.description?.takeIf { it.isNotBlank() }?.let { description ->
                Text(
                    text = description,
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun NoteDetailScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val topic = controller.selectedNoteTopic
    val scrollState = rememberScrollState()

    TrainerScaffold(
        title = topic?.let {
            "${it.courseCode.uppercase()} · ${controller.notesPartLabel(it.notesPart)} · ${it.topicNumber}. ${it.title}"
        } ?: strings.notes,
        onBack = controller::back,
    ) { modifier ->
        if (topic == null) {
            EmptyMessage(
                text = strings.notesEmpty,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        Box(modifier = modifier) {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .verticalScroll(scrollState)
                    .padding(end = LayoutSpacing.section),
            ) {
                val markdown = controller.selectedNoteBody
                if (markdown.isBlank()) {
                    Text(
                        text = strings.notesMissing,
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                } else {
                    LabMarkdownContent(
                        markdown = markdown,
                        modifier = Modifier.fillMaxWidth(),
                    )
                }
            }

            LabScrollPositionIndicator(
                scrollState = scrollState,
                modifier = Modifier
                    .align(Alignment.CenterEnd)
                    .fillMaxHeight()
                    .padding(vertical = 4.dp),
            )
        }
    }
}

@Composable
private fun CoursesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    CourseSelectionScreen(
        controller = controller,
        title = strings.generalQuiz,
        prompt = strings.chooseCourse,
        onChoose = controller::chooseCourse,
        xpForCourse = controller::courseXpAvailable,
    )
}

@Composable
private fun CourseSelectionScreen(
    controller: TrainerController,
    title: String,
    prompt: String,
    onChoose: (String) -> Unit,
    xpForCourse: ((String) -> ULong)?,
) {
    val strings = LocalUiStrings.current

    TrainerScaffold(
        title = title,
        onBack = controller::back,
    ) { modifier ->
        if (controller.courseCodes.isEmpty()) {
            EmptyMessage(
                text = strings.coursesEmpty,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                Text(
                    text = prompt,
                    style = MaterialTheme.typography.titleLarge,
                )
            }

            items(
                items = controller.courseCodes,
                key = { courseCode -> courseCode },
            ) { courseCode ->
                Card(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { onChoose(courseCode) },
                ) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                    ) {
                        Text(
                            text = courseCode.uppercase(),
                            style = MaterialTheme.typography.titleLarge,
                            fontWeight = FontWeight.SemiBold,
                        )

                        val topicCount = controller.topics.count { topic ->
                            topic.courseCode == courseCode
                        }

                        Spacer(Modifier.height(4.dp))
                        Text(
                            text = strings.topicCount(topicCount),
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )

                        xpForCourse?.invoke(courseCode)?.takeIf { xp -> xp > 0UL }?.let { xp ->
                            Spacer(Modifier.height(4.dp))
                            Text(
                                text = "${strings.xpAvailable}: $xp XP",
                                style = MaterialTheme.typography.labelLarge,
                                color = MaterialTheme.colorScheme.primary,
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun TopicCard(
    topic: MobileTopic,
    displayNumber: String,
    masterySummary: String,
    xpAvailable: ULong,
    onClick: () -> Unit,
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Column(
            modifier = Modifier.padding(LayoutSpacing.card),
        ) {
            Text(
                text = topic.courseCode.uppercase(),
                style = MaterialTheme.typography.labelLarge,
                color = MaterialTheme.colorScheme.primary,
            )

            Spacer(Modifier.height(4.dp))

            Text(
                text = "$displayNumber. ${topic.title}",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )

            Spacer(Modifier.height(6.dp))

            Text(
                text = masterySummary,
                style = MaterialTheme.typography.labelLarge,
                color = MaterialTheme.colorScheme.primary,
            )

            if (xpAvailable > 0UL) {
                Spacer(Modifier.height(4.dp))
                Text(
                    text = "${LocalUiStrings.current.xpAvailable}: $xpAvailable XP",
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.tertiary,
                )
            }

            topic.description?.let { description ->
                Spacer(Modifier.height(8.dp))
                Text(
                    text = description,
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun LabCoursesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    TrainerScaffold(
        title = strings.labs,
        onBack = controller::back,
    ) { modifier ->
        if (controller.labCourseCodes.isEmpty()) {
            EmptyMessage(
                text = strings.labsEmpty,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                Text(
                    text = strings.chooseLabCourse,
                    style = MaterialTheme.typography.titleLarge,
                )
            }

            items(
                items = controller.labCourseCodes,
                key = { courseCode -> courseCode },
            ) { courseCode ->
                Card(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { controller.chooseLabCourse(courseCode) },
                ) {
                    Column(Modifier.padding(LayoutSpacing.card)) {
                        Text(
                            text = courseCode.uppercase(),
                            style = MaterialTheme.typography.titleLarge,
                            fontWeight = FontWeight.SemiBold,
                        )

                        Spacer(Modifier.height(4.dp))

                        Text(
                            text = strings.labCount(
                                controller.labs.count { lab -> lab.courseCode == courseCode },
                            ),
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun LabsScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val courseCode = controller.selectedLabCourse
    val labs = controller.labsForSelectedCourse

    TrainerScaffold(
        title = if (courseCode == null) {
            strings.labs
        } else {
            "${strings.labs} → ${courseCode.uppercase()}"
        },
        onBack = controller::back,
    ) { modifier ->
        if (labs.isEmpty()) {
            EmptyMessage(
                text = strings.labsEmpty,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            items(
                items = labs,
                key = { lab -> lab.id },
            ) { lab ->
                LabCard(
                    lab = lab,
                    onClick = { controller.chooseLab(lab) },
                )
            }
        }
    }
}

@Composable
private fun LabCard(
    lab: MobileLab,
    onClick: () -> Unit,
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Column(Modifier.padding(LayoutSpacing.card)) {
            Text(
                text = "${lab.labNumber}. ${lab.title}",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )

            lab.description?.let { description ->
                Spacer(Modifier.height(8.dp))
                Text(
                    text = description,
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun LabDetailScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val lab = controller.selectedLab
    val scrollState = rememberScrollState()

    TrainerScaffold(
        title = lab?.let { "${it.labNumber}. ${it.title}" } ?: strings.labs,
        onBack = controller::back,
    ) { modifier ->
        if (lab == null) {
            EmptyMessage(
                text = strings.labsEmpty,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        Box(modifier = modifier) {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .verticalScroll(scrollState)
                    .padding(end = LayoutSpacing.section),
            ) {
                LabMarkdownContent(
                    markdown = lab.body,
                    modifier = Modifier.fillMaxWidth(),
                )
            }

            LabScrollPositionIndicator(
                scrollState = scrollState,
                modifier = Modifier
                    .align(Alignment.CenterEnd)
                    .fillMaxHeight()
                    .padding(vertical = 4.dp),
            )
        }
    }
}

@Composable
private fun LabScrollPositionIndicator(
    scrollState: ScrollState,
    modifier: Modifier = Modifier,
) {
    if (scrollState.maxValue <= 0) {
        return
    }

    BoxWithConstraints(
        modifier = modifier.width(5.dp),
    ) {
        val density = LocalDensity.current
        val viewportPixels = with(density) { maxHeight.toPx() }
        val contentPixels = viewportPixels + scrollState.maxValue.toFloat()
        val visibleFraction =
            if (contentPixels <= 0f) 1f else (viewportPixels / contentPixels).coerceIn(0f, 1f)
        val thumbHeight = minOf(maxHeight, maxOf(32.dp, maxHeight * visibleFraction))
        val travel = maxHeight - thumbHeight
        val progress =
            (scrollState.value.toFloat() / scrollState.maxValue.toFloat()).coerceIn(0f, 1f)

        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(
                    color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.12f),
                    shape = RoundedCornerShape(4.dp),
                ),
        )

        Box(
            modifier = Modifier
                .offset(y = travel * progress)
                .fillMaxWidth()
                .height(thumbHeight)
                .background(
                    color = MaterialTheme.colorScheme.primary.copy(alpha = 0.75f),
                    shape = RoundedCornerShape(4.dp),
                ),
        )
    }
}

@Composable
private fun LimitScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val title = when {
        controller.pendingMistakeReview && controller.pendingTopic != null -> {
            "${strings.reviewTopicMistakes}: ${controller.pendingTopic!!.title}"
        }
        controller.pendingMistakeReview && controller.pendingCourse != null -> {
            "${strings.reviewCourseMistakes}: ${controller.pendingCourse!!.uppercase()}"
        }
        controller.pendingMistakeReview -> strings.workOnMistakes
        controller.pendingTopic != null -> {
            val topic = controller.pendingTopic!!
            "${topic.topicNumber}. ${topic.title}"
        }
        controller.pendingCourse != null -> controller.pendingCourse!!.uppercase()
        else -> strings.generalQuiz
    }

    val xpOpportunity = controller.pendingQuizXpOpportunity

    TrainerScaffold(
        title = title,
        onBack = controller::back,
    ) { modifier ->
        Column(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            controller.unfinishedSessionsForPendingQuiz.forEach { session ->
                UnfinishedQuizCard(
                    controller = controller,
                    session = session,
                    onContinue = { controller.continueSavedSession(session) },
                    onCancel = { controller.requestDiscardSavedSession(session) },
                )
            }

            Text(
                text = strings.questionCount,
                style = MaterialTheme.typography.titleLarge,
            )

            LimitButton(
                text = quizLimitLabel(
                    strings = strings,
                    base = strings.twentyQuestions,
                    bonusXp = xpOpportunity?.twentyBonusXp,
                ),
                onClick = {
                    controller.startQuiz(
                        MobileQuestionLimit.TWENTY,
                    )
                },
            )

            LimitButton(
                text = quizLimitLabel(
                    strings = strings,
                    base = strings.fiftyQuestions,
                    bonusXp = xpOpportunity?.fiftyBonusXp,
                ),
                onClick = {
                    controller.startQuiz(
                        MobileQuestionLimit.FIFTY,
                    )
                },
            )

            LimitButton(
                text = quizLimitLabel(
                    strings = strings,
                    base = strings.allQuestions,
                    bonusXp = xpOpportunity?.allBonusXp,
                ),
                onClick = {
                    controller.startQuiz(
                        MobileQuestionLimit.ALL,
                    )
                },
            )

            Spacer(Modifier.height(4.dp))

            InfoCard(
                title = strings.passCriteria,
                text = buildString {
                    append("${strings.twentyQuestions}: 85% (17/20)\n")
                    append("${strings.fiftyQuestions}: 70% (35/50)\n")
                    append("${strings.allQuestions}: 60%")
                },
            )

            xpOpportunity
                ?.let { opportunity -> quizXpDetails(strings, opportunity) }
                ?.takeIf { details -> details.isNotEmpty() }
                ?.let { details ->
                    InfoCard(
                        title = strings.xpAvailable,
                        text = details,
                    )
                }
        }
    }
}

private fun quizLimitLabel(
    strings: UiStrings,
    base: String,
    bonusXp: ULong?,
): String = when {
    bonusXp == null -> base
    bonusXp > 0UL -> "$base · +$bonusXp XP ${strings.xpPassBonus}"
    else -> "$base · ${strings.xpReceived}"
}

private fun quizXpDetails(
    strings: UiStrings,
    opportunity: MobileQuizXpOpportunity,
): String {
    val lines = buildList {
        if (opportunity.uniqueQuestionXp > 0UL) {
            add("${strings.xpQuestions}: +${opportunity.uniqueQuestionXp} XP")
        }
        if (opportunity.masteryXp > 0UL) {
            add("${strings.xpTopics}: +${opportunity.masteryXp} XP")
        }
        if (opportunity.courseCompletionXp > 0UL) {
            add("${strings.xpCourses}: +${opportunity.courseCompletionXp} XP")
        }
    }

    return lines.joinToString("\n")
}

@Composable
private fun LimitButton(
    text: String,
    onClick: () -> Unit,
) {
    OutlinedButton(
        onClick = onClick,
        modifier = Modifier.fillMaxWidth(),
    ) {
        Text(text)
    }
}

@Composable
private fun QuizScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val question = controller.currentQuestion
        ?: return

    val currentNumber = (controller.answeredQuestions + 1)
        .coerceAtMost(controller.totalQuestions.coerceAtLeast(1))

    val progress = if (controller.totalQuestions == 0) {
        0f
    } else {
        controller.answeredQuestions.toFloat() /
            controller.totalQuestions.toFloat()
    }

    TrainerScaffold(
        title = strings.questionProgress(currentNumber, controller.totalQuestions),
        onBack = controller::requestCancelQuiz,
    ) { modifier ->
        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                LinearProgressIndicator(
                    progress = { progress },
                    modifier = Modifier.fillMaxWidth(),
                )
            }

            if (controller.questionXpAvailable(question.id)) {
                item {
                    Text(
                        text = "+1 XP · ${strings.xpQuestions}",
                        style = MaterialTheme.typography.labelLarge,
                        color = MaterialTheme.colorScheme.tertiary,
                    )
                }
            }

            item {
                Text(
                    text = question.text,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.SemiBold,
                )
            }

            item {
                Text(
                    text = if (
                        question.questionType ==
                        MobileQuestionType.MULTIPLE_CHOICE
                    ) {
                        strings.chooseAllCorrect
                    } else {
                        strings.chooseOne
                    },
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }

            items(
                items = question.options,
                key = { option -> option.id },
            ) { option ->
                AnswerRow(
                    text = option.text,
                    selected = controller.selectedOptionIds
                        .contains(option.id),
                    multiple = question.questionType ==
                        MobileQuestionType.MULTIPLE_CHOICE,
                    onClick = {
                        controller.selectOption(option.id)
                    },
                )
            }

            item {
                Spacer(Modifier.height(8.dp))

                Button(
                    onClick = controller::submitAnswer,
                    enabled = controller.selectedOptionIds.isNotEmpty(),
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(strings.answer)
                }
            }
        }
    }
}

@Composable
private fun AnswerRow(
    text: String,
    selected: Boolean,
    multiple: Boolean,
    onClick: () -> Unit,
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            if (multiple) {
                Checkbox(
                    checked = selected,
                    onCheckedChange = {
                        onClick()
                    },
                )
            } else {
                RadioButton(
                    selected = selected,
                    onClick = onClick,
                )
            }

            Spacer(Modifier.width(8.dp))

            Text(
                text = text,
                style = MaterialTheme.typography.bodyLarge,
            )
        }
    }
}

@Composable
private fun FeedbackScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val question = controller.currentQuestion
        ?: return
    val result = controller.feedback
        ?: return

    TrainerScaffold(
        title = if (result.isCorrect) {
            strings.correct
        } else {
            strings.incorrect
        },
        onBack = controller::requestCancelQuiz,
    ) { modifier ->
        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                FeedbackAnswers(
                    question = question,
                    result = result,
                )
            }

            item {
                InfoCard(
                    title = strings.explanation,
                    text = question.explanation,
                )
            }

            item {
                InfoCard(
                    title = strings.source,
                    text = formatSource(
                        question.source,
                        strings,
                    ),
                )
            }

            item {
                Button(
                    onClick = controller::nextQuestion,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(strings.nextQuestion)
                }
            }
        }
    }
}

@Composable
private fun FeedbackAnswers(
    question: MobileQuestion,
    result: MobileAnswerResult,
) {
    val strings = LocalUiStrings.current

    Column(
        verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
    ) {
        Text(
            text = strings.yourAnswer,
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.SemiBold,
        )

        OptionList(
            question = question,
            optionIds = result.selectedOptionIds,
        )

        HorizontalDivider()

        Text(
            text = if (result.correctOptionIds.size > 1) {
                strings.correctAnswers
            } else {
                strings.correctAnswer
            },
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.SemiBold,
        )

        OptionList(
            question = question,
            optionIds = result.correctOptionIds,
        )
    }
}

@Composable
private fun OptionList(
    question: MobileQuestion,
    optionIds: List<Long>,
) {
    val texts = question.options
        .filter { option ->
            optionIds.contains(option.id)
        }
        .map { option -> option.text }

    Column(
        verticalArrangement = Arrangement.spacedBy(6.dp),
    ) {
        texts.forEach { text ->
            Text("• $text")
        }
    }
}

@Composable
private fun InfoCard(
    title: String,
    text: String,
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
    ) {
        Column(
            modifier = Modifier.padding(LayoutSpacing.card),
        ) {
            Text(
                text = title,
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )

            Spacer(Modifier.height(8.dp))

            Text(
                text = text,
                style = MaterialTheme.typography.bodyMedium,
            )
        }
    }
}

@Composable
private fun AchievementCelebrationOverlay(
    achievements: List<MobileAchievementProgress>,
    onDismiss: () -> Unit,
) {
    if (achievements.isEmpty()) {
        return
    }

    val strings = LocalUiStrings.current
    var index by remember(achievements) { mutableStateOf(0) }
    val achievement = achievements[index.coerceIn(0, achievements.lastIndex)]
    val transition = rememberInfiniteTransition(label = "achievement-fireworks")
    val phase by transition.animateFloat(
        initialValue = 0f,
        targetValue = 1f,
        animationSpec = infiniteRepeatable(
            animation = tween(durationMillis = 1_500),
            repeatMode = RepeatMode.Restart,
        ),
        label = "achievement-fireworks-phase",
    )
    val primary = MaterialTheme.colorScheme.primary
    val secondary = MaterialTheme.colorScheme.secondary
    val tertiary = MaterialTheme.colorScheme.tertiary

    Dialog(
        onDismissRequest = {},
        properties = DialogProperties(
            dismissOnBackPress = false,
            dismissOnClickOutside = false,
            usePlatformDefaultWidth = false,
        ),
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(24.dp),
            contentAlignment = Alignment.Center,
        ) {
            Canvas(modifier = Modifier.fillMaxSize()) {
                val centers = listOf(
                    Offset(size.width * 0.5f, size.height * 0.5f),
                    Offset(size.width * 0.24f, size.height * 0.38f),
                    Offset(size.width * 0.76f, size.height * 0.38f),
                )
                val colors = listOf(primary, secondary, tertiary)
                centers.forEachIndexed { burstIndex, center ->
                    val burstPhase = (phase + burstIndex * 0.22f) % 1f
                    val radius = 28f + burstPhase * 150f
                    val alpha = (1f - burstPhase).coerceIn(0f, 1f)
                    repeat(18) { particle ->
                        val angle = (particle / 18f) * (Math.PI * 2.0)
                        val point = Offset(
                            x = center.x + cos(angle).toFloat() * radius,
                            y = center.y + sin(angle).toFloat() * radius,
                        )
                        drawCircle(
                            color = colors[burstIndex].copy(alpha = alpha),
                            radius = 5f,
                            center = point,
                        )
                    }
                }
            }

            Card(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 12.dp),
            ) {
                Column(
                    modifier = Modifier.padding(LayoutSpacing.card),
                    horizontalAlignment = Alignment.CenterHorizontally,
                    verticalArrangement = Arrangement.spacedBy(12.dp),
                ) {
                    Text(
                        text = strings.achievementUnlockedTitle(achievements.size),
                        style = MaterialTheme.typography.headlineSmall,
                        fontWeight = FontWeight.Bold,
                    )
                    Text(
                        text = "🏆",
                        style = MaterialTheme.typography.displayMedium,
                    )
                    Text(
                        text = strings.achievementTitle(achievement.kind),
                        style = MaterialTheme.typography.titleLarge,
                        fontWeight = FontWeight.Bold,
                    )
                    Text(
                        text = strings.achievementDescription(achievement.kind),
                        style = MaterialTheme.typography.bodyMedium,
                    )
                    Text(
                        text = strings.achievementReward(achievement.rewardXp),
                        style = MaterialTheme.typography.titleMedium,
                        color = MaterialTheme.colorScheme.primary,
                        fontWeight = FontWeight.Bold,
                    )

                    if (achievements.size > 1) {
                        Row(
                            modifier = Modifier.fillMaxWidth(),
                            horizontalArrangement = Arrangement.SpaceBetween,
                            verticalAlignment = Alignment.CenterVertically,
                        ) {
                            TextButton(
                                onClick = {
                                    index = if (index == 0) achievements.lastIndex else index - 1
                                },
                            ) {
                                Text("‹")
                            }
                            Text(
                                text = "${index + 1}/${achievements.size}",
                                style = MaterialTheme.typography.labelLarge,
                            )
                            TextButton(
                                onClick = {
                                    index = if (index == achievements.lastIndex) 0 else index + 1
                                },
                            ) {
                                Text("›")
                            }
                        }
                    }

                    Button(
                        onClick = onDismiss,
                        modifier = Modifier.fillMaxWidth(),
                    ) {
                        Text(strings.closeAchievement)
                    }
                }
            }
        }
    }
}

@Composable
private fun ResultsScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val accuracy = if (controller.answeredQuestions == 0) {
        0.0
    } else {
        controller.correctAnswers.toDouble() /
            controller.answeredQuestions.toDouble() * 100.0
    }
    val limit = controller.activeQuestionLimit
    val threshold = limit?.let(controller::passThresholdPercent)

    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(
            horizontal = LayoutSpacing.screenHorizontal,
            vertical = LayoutSpacing.screenVertical,
        ),
        verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        item {
            Text(
                text = strings.quizCompleted,
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold,
            )
        }

        item {
            Text(
                text = strings.resultScore(
                    controller.correctAnswers,
                    controller.answeredQuestions,
                ),
                style = MaterialTheme.typography.displaySmall,
                color = MaterialTheme.colorScheme.primary,
            )
        }

        item {
            Text(
                text = strings.percentCorrect(formatPercent(accuracy)),
                style = MaterialTheme.typography.titleMedium,
            )
        }

        if (threshold != null) {
            item {
                InfoCard(
                    title = strings.passCriteria,
                    text = buildString {
                        append("${strings.passThreshold}: $threshold%\n")
                        append(
                            "${strings.passStatus}: " +
                                if (controller.resultPassed) strings.passed else strings.notPassed,
                        )
                    },
                )
            }
        }

        item {
            Button(
                onClick = controller::finishResults,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(strings.home)
            }
        }
    }
}

@Composable
private fun StatisticsScreen(controller: TrainerController) {
    when (controller.statisticsPage) {
        StatisticsPage.Root -> StatisticsRootScreen(controller)
        StatisticsPage.Courses -> StatisticsCoursesScreen(controller)
        StatisticsPage.Topics -> StatisticsTopicsScreen(controller)
        StatisticsPage.GlobalModes -> StatisticsGlobalModesScreen(controller)
        StatisticsPage.ScopeModes -> StatisticsScopeModesScreen(controller)
        StatisticsPage.Mistakes -> StatisticsMistakesScreen(controller)
        StatisticsPage.MistakeTopics -> StatisticsMistakeTopicsScreen(controller)
        StatisticsPage.MistakeQuestions -> StatisticsMistakeQuestionsScreen(controller)
        StatisticsPage.MistakeDetail -> StatisticsMistakeDetailScreen(controller)
        StatisticsPage.Activity -> StatisticsActivityScreen(controller)
        StatisticsPage.Practice -> PracticeStatisticsScreen(controller)
        StatisticsPage.Detail -> StatisticsDetailScreen(controller)
    }
}

@Composable
private fun StatisticsRootScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val overall = controller.overallStatistics

    TrainerScaffold(
        title = strings.statistics,
        onBack = controller::back,
    ) { modifier ->
        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                StatisticsNavigationCard(
                    title = strings.overall,
                    stats = overall,
                    subtitle = strings.allCoursesTopicsModes,
                    onClick = controller::openOverallStatistics,
                )
            }

            item {
                StatisticsNavigationCard(
                    title = strings.byCoursesAndTopics,
                    stats = overall,
                    subtitle = strings.courseTopicMode,
                    onClick = controller::openStatisticsCourses,
                )
            }

            item {
                StatisticsNavigationCard(
                    title = strings.byModes,
                    stats = overall,
                    subtitle = strings.modesDescription,
                    onClick = controller::openStatisticsGlobalModes,
                )
            }

            item {
                StatisticsNavigationCard(
                    title = strings.mistakes,
                    stats = null,
                    subtitle = strings.workOnMistakes,
                    onClick = controller::openMistakes,
                )
            }

            item {
                StatisticsNavigationCard(
                    title = strings.activity,
                    stats = null,
                    subtitle = strings.studyDayTarget,
                    onClick = controller::openActivity,
                )
            }

            item {
                StatisticsNavigationCard(
                    title = strings.practice,
                    stats = null,
                    subtitle = strings.practiceProgress,
                    onClick = controller::openPracticeStatistics,
                )
            }

            item {
                OutlinedButton(
                    onClick = controller::requestClearStatistics,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(strings.clearStatistics)
                }
            }
        }
    }
}

@Composable
private fun StatisticsCoursesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    TrainerScaffold(
        title = strings.statisticsCourses,
        onBack = controller::back,
    ) { modifier ->
        if (controller.courseStatistics.isEmpty()) {
            EmptyMessage(
                text = strings.coursesEmpty,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            items(
                items = controller.courseStatistics,
                key = { row -> row.courseCode },
            ) { row ->
                CourseStatisticsCard(
                    row = row,
                    onClick = {
                        controller.chooseStatisticsCourse(row.courseCode)
                    },
                )
            }
        }
    }
}

@Composable
private fun CourseStatisticsCard(
    row: CourseStatisticsRow,
    onClick: () -> Unit,
) {
    val strings = LocalUiStrings.current

    StatisticsNavigationCard(
        title = row.courseCode.uppercase(),
        stats = row.stats,
        subtitle = strings.courseStatistics,
        onClick = onClick,
    )
}

@Composable
private fun StatisticsTopicsScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val courseCode = controller.selectedStatisticsCourse
        ?: return
    val courseStats = controller.courseStatistics
        .firstOrNull { row -> row.courseCode == courseCode }
        ?.stats

    TrainerScaffold(
        title = "${strings.statistics} → ${courseCode.uppercase()}",
        onBack = controller::back,
    ) { modifier ->
        val rows = controller.topicStatistics
        val topics = rows.map { row -> row.topic }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                StatisticsNavigationCard(
                    title = strings.allTopics,
                    stats = courseStats,
                    subtitle = courseCode.uppercase(),
                    onClick = controller::openCourseAllTopicsModes,
                )
            }

            itemsIndexed(
                items = rows,
                key = { _, row -> row.topic.id },
            ) { index, row ->
                TopicStatisticsCard(
                    row = row,
                    displayNumber = topicDisplayNumber(topics, index),
                    onClick = {
                        controller.openTopicModes(row.topic)
                    },
                )
            }

            item {
                OutlinedButton(
                    onClick = controller::requestClearCurrentStatisticsScope,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(strings.clearScopeStatistics)
                }
            }
        }
    }
}

@Composable
private fun TopicStatisticsCard(
    row: TopicStatisticsRow,
    displayNumber: String,
    onClick: () -> Unit,
) {
    StatisticsNavigationCard(
        title = "$displayNumber. ${row.topic.title}",
        stats = row.stats,
        subtitle = row.topic.courseCode.uppercase(),
        onClick = onClick,
    )
}

@Composable
private fun StatisticsGlobalModesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    TrainerScaffold(
        title = strings.statisticsModes,
        onBack = controller::back,
    ) { modifier ->
        StatisticsModesList(
            rows = controller.globalModeStatistics,
            modifier = modifier,
            onClick = controller::openGlobalModeDetail,
        )
    }
}

@Composable
private fun StatisticsScopeModesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    TrainerScaffold(
        title = controller.statisticsScopeTitle,
        onBack = controller::back,
    ) { modifier ->
        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            items(
                items = controller.scopeModeStatistics,
                key = { row -> row.title },
            ) { row ->
                StatisticsNavigationCard(
                    title = row.title,
                    stats = row.stats,
                    onClick = { controller.openScopeModeDetail(row) },
                )
            }

            item {
                OutlinedButton(
                    onClick = controller::requestClearCurrentStatisticsScope,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(strings.clearScopeStatistics)
                }
            }
        }
    }
}

@Composable
private fun StatisticsModesList(
    rows: List<ModeStatisticsRow>,
    modifier: Modifier,
    onClick: (ModeStatisticsRow) -> Unit,
) {
    LazyColumn(
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
    ) {
        items(
            items = rows,
            key = { row -> row.title },
        ) { row ->
            StatisticsNavigationCard(
                title = row.title,
                stats = row.stats,
                onClick = {
                    onClick(row)
                },
            )
        }
    }
}

@Composable
private fun StatisticsMistakesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val mistakes = controller.mistakeSummaries

    TrainerScaffold(
        title = strings.mistakes,
        onBack = controller::back,
    ) { modifier ->
        if (mistakes.isEmpty()) {
            EmptyMessage(
                text = strings.noMistakes,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                Button(
                    onClick = controller::openMistakeReview,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text("${strings.workOnMistakes} (${mistakes.size})")
                }
            }

            items(
                items = controller.mistakeCourseCodes,
                key = { courseCode -> courseCode },
            ) { courseCode ->
                MistakeGroupCard(
                    title = courseCode.uppercase(),
                    count = controller.mistakeCountForCourse(courseCode),
                    onClick = { controller.openMistakeCourse(courseCode) },
                )
            }
        }
    }
}

@Composable
private fun StatisticsMistakeTopicsScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val courseCode = controller.selectedMistakeCourse

    TrainerScaffold(
        title = buildString {
            append(strings.mistakes)
            courseCode?.let { append(" → ${it.uppercase()}") }
        },
        onBack = controller::back,
    ) { modifier ->
        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                Button(
                    onClick = controller::openMistakeCourseReview,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(strings.reviewCourseMistakes)
                }
            }

            items(
                items = controller.mistakeTopicsForSelectedCourse,
                key = { topic -> topic.id },
            ) { topic ->
                MistakeGroupCard(
                    title = "${topic.topicNumber}. ${topic.title}",
                    count = controller.mistakeCountForTopic(topic.id),
                    onClick = { controller.openMistakeTopic(topic) },
                )
            }
        }
    }
}

@Composable
private fun StatisticsMistakeQuestionsScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val topic = controller.selectedMistakeTopic

    TrainerScaffold(
        title = buildString {
            append(strings.mistakes)
            controller.selectedMistakeCourse?.let { append(" → ${it.uppercase()}") }
            topic?.let { append(" → ${it.title}") }
        },
        onBack = controller::back,
    ) { modifier ->
        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                Button(
                    onClick = controller::openMistakeTopicReview,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(strings.reviewTopicMistakes)
                }
            }

            items(
                items = controller.mistakesForSelectedTopic,
                key = { summary -> summary.questionId },
            ) { summary ->
                MistakeSummaryCard(
                    summary = summary,
                    onClick = { controller.openMistakeDetail(summary) },
                )
            }
        }
    }
}

@Composable
private fun MistakeGroupCard(
    title: String,
    count: Int,
    onClick: () -> Unit,
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Row(
            modifier = Modifier.padding(LayoutSpacing.card),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text(
                text = title,
                modifier = Modifier.weight(1f),
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
            Text(
                text = count.toString(),
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.primary,
            )
        }
    }
}

@Composable
private fun MistakeSummaryCard(
    summary: MobileQuestionMistakeSummary,
    onClick: () -> Unit,
) {
    val strings = LocalUiStrings.current

    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Column(
            modifier = Modifier.padding(LayoutSpacing.card),
            verticalArrangement = Arrangement.spacedBy(6.dp),
        ) {
            Text(
                text = summary.questionText,
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
            Text(
                text = "${summary.courseCode.uppercase()} · ${summary.topicTitle}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            Text(
                text = "${strings.incorrectAnswersCount}: ${summary.incorrectAttempts}/${summary.totalAttempts} · ${formatPercent(summary.errorRatePercent)}",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.primary,
            )
            Text(
                text = "${strings.lastError}: ${summary.lastIncorrectAt}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
private fun StatisticsMistakeDetailScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val detail = controller.mistakeDetail

    TrainerScaffold(
        title = strings.mistakes,
        onBack = controller::back,
    ) { modifier ->
        if (detail == null) {
            EmptyMessage(
                text = strings.statisticsUnavailable,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = detail.question.text,
                            style = MaterialTheme.typography.titleLarge,
                            fontWeight = FontWeight.SemiBold,
                        )
                        Text(
                            text = "${detail.summary.courseCode.uppercase()} · ${detail.summary.topicTitle}",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                        Text(
                            text = "${strings.incorrectAnswersCount}: ${detail.summary.incorrectAttempts}/${detail.summary.totalAttempts} · ${formatPercent(detail.summary.errorRatePercent)}",
                            style = MaterialTheme.typography.bodyLarge,
                            color = MaterialTheme.colorScheme.primary,
                        )
                        Text(
                            text = "${strings.lastError}: ${detail.summary.lastIncorrectAt}",
                            style = MaterialTheme.typography.bodySmall,
                        )
                    }
                }
            }

            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = strings.correctAnswers,
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        detail.correctAnswers.forEach { answer ->
                            Text("• $answer")
                        }
                        HorizontalDivider()
                        Text(
                            text = strings.explanation,
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        Text(detail.question.explanation)
                        Text(
                            text = "${strings.source}: ${formatSource(detail.question.source, strings)}",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }

            item {
                Text(
                    text = strings.attemptHistory,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold,
                )
            }

            itemsIndexed(
                items = detail.attempts,
            ) { index, attempt ->
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(6.dp),
                    ) {
                        Text(
                            text = "${index + 1}. ${if (attempt.isCorrect) strings.correct else strings.incorrect} · ${attempt.answeredAt}",
                            fontWeight = FontWeight.SemiBold,
                            color = if (attempt.isCorrect) {
                                MaterialTheme.colorScheme.primary
                            } else {
                                MaterialTheme.colorScheme.error
                            },
                        )
                        Text("${strings.yourAnswer}:")
                        if (attempt.selectedAnswers.isEmpty()) {
                            Text("—")
                        } else {
                            attempt.selectedAnswers.forEach { answer ->
                                Text("• $answer")
                            }
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun StatisticsActivityScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val stats = controller.activityStats

    TrainerScaffold(
        title = strings.activity,
        onBack = controller::back,
    ) { modifier ->
        if (stats == null) {
            EmptyMessage(
                text = strings.statisticsUnavailable,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        val todayAccuracy = if (stats.todayAnsweredQuestions == 0UL) {
            0.0
        } else {
            stats.todayCorrectAnswers.toDouble() /
                stats.todayAnsweredQuestions.toDouble() * 100.0
        }
        val recentPracticeAttempts = stats.recentDays.fold(0UL) { total, day ->
            total + day.practiceAttempts
        }
        val recentPracticeSolved = stats.recentDays.fold(0UL) { total, day ->
            total + day.practiceSolved
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                RankSummaryCard(stats)
            }

            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = strings.rankScale,
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        stats.rankDefinitions.forEach { definition ->
                            val rank = definition.rank
                            val threshold = definition.minimumXp
                            val marker = when {
                                rank == stats.rank -> "›"
                                stats.xp >= threshold -> "✓"
                                else -> "·"
                            }
                            Text(
                                text = "$marker ${strings.progressRank(rank)} — $threshold XP",
                                fontWeight = if (rank == stats.rank) FontWeight.SemiBold else FontWeight.Normal,
                                color = if (rank == stats.rank) {
                                    MaterialTheme.colorScheme.primary
                                } else {
                                    MaterialTheme.colorScheme.onSurface
                                },
                            )
                        }
                    }
                }
            }

            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = strings.xpRules,
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        stats.xpRules.forEach { rule ->
                            Text("+${rule.points} XP · ${strings.xpSource(rule.source)}")
                        }
                    }
                }
            }

            item {
                val opportunities = stats.xpOpportunitySummary
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = "${strings.xpOpportunities}: ${opportunities.totalAvailableXp} XP",
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        Text("${strings.xpQuestions}: ${opportunities.questionXp} XP")
                        Text("${strings.xpQuizzes}: ${opportunities.quizXp} XP")
                        Text("${strings.xpTopics}: ${opportunities.masteryXp} XP")
                        Text("${strings.xpCourses}: ${opportunities.courseXp} XP")
                        Text("${strings.xpPractice}: ${opportunities.practiceXp} XP")
                    }
                }
            }

            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = strings.mastery,
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        Text(
                            text = "${stats.passedTopics}/${stats.totalTopics} · ${formatPercent(stats.overallMasteryPercent)}",
                            fontWeight = FontWeight.SemiBold,
                        )
                        LinearProgressIndicator(
                            progress = {
                                (stats.overallMasteryPercent / 100.0)
                                    .toFloat()
                                    .coerceIn(0f, 1f)
                            },
                            modifier = Modifier.fillMaxWidth(),
                        )
                        HorizontalDivider()
                        Text("${strings.today}: ${stats.todayAnsweredQuestions} · ${stats.todayCorrectAnswers}/${stats.todayAnsweredQuestions} · ${formatPercent(todayAccuracy)}")
                        Text("${strings.activeDays}: ${stats.activeDays}")
                        Text("${strings.studyDays}: ${stats.studyDays}")
                        Text("${strings.currentStreak}: ${stats.currentStreak}")
                        Text("${strings.bestStreak}: ${stats.bestStreak}")
                        Text("${strings.answeredQuestions}: ${stats.totalAnsweredQuestions}")
                        Text("${strings.last7Days}: ${stats.last7DaysAnsweredQuestions}")
                        Text(
                            text = strings.studyDayTarget,
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }

            item {
                Text(
                    text = strings.courseProgress,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold,
                )
            }

            items(
                items = stats.courseProgress,
                key = { progress -> progress.courseCode },
            ) { progress ->
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(6.dp),
                    ) {
                        Text(
                            text = progress.courseCode.uppercase(),
                            fontWeight = FontWeight.SemiBold,
                        )
                        Text("${progress.passedTopics}/${progress.totalTopics} · ${formatPercent(progress.completionPercent)}")
                        LinearProgressIndicator(
                            progress = {
                                (progress.completionPercent / 100.0)
                                    .toFloat()
                                    .coerceIn(0f, 1f)
                            },
                            modifier = Modifier.fillMaxWidth(),
                        )
                    }
                }
            }

            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        val unlocked = stats.achievements.count { achievement -> achievement.unlocked }
                        Text(
                            text = "${strings.achievements}: $unlocked/${stats.achievements.size}",
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        stats.achievements.forEach { achievement ->
                            val state = if (achievement.unlocked) "✓" else "·"
                            Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
                                Text(
                                    text = "$state ${strings.achievementTitle(achievement.kind)} · ${strings.achievementRarity(achievement.rarity)}",
                                    fontWeight = if (achievement.unlocked) FontWeight.SemiBold else FontWeight.Normal,
                                )
                                Text(
                                    text = strings.achievementDescription(achievement.kind),
                                    style = MaterialTheme.typography.bodySmall,
                                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                                )
                                Text(
                                    text = strings.achievementReward(achievement.rewardXp),
                                    style = MaterialTheme.typography.labelMedium,
                                    color = MaterialTheme.colorScheme.primary,
                                    fontWeight = FontWeight.SemiBold,
                                )
                                if (!achievement.unlocked) {
                                    Text(
                                        text = "${achievement.current}/${achievement.target}",
                                        style = MaterialTheme.typography.labelSmall,
                                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                                    )
                                }
                            }
                        }
                    }
                }
            }

            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(LayoutSpacing.card),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = strings.recentActivity,
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        Text("${strings.activeDays28}: ${stats.recentDays.size}")
                        if (recentPracticeAttempts > 0UL) {
                            Text("${strings.sqlAttempts}: $recentPracticeAttempts")
                        }
                        if (recentPracticeSolved > 0UL) {
                            Text("${strings.sqlSolved}: $recentPracticeSolved")
                        }

                        if (stats.recentDays.isEmpty()) {
                            Text(strings.noRecentActivity)
                        } else {
                            stats.recentDays.asReversed().forEach { day ->
                                val details = buildList {
                                    if (day.answeredQuestions > 0UL) {
                                        add("${strings.answeredQuestions}: ${day.answeredQuestions}")
                                    }
                                    if (day.practiceAttempts > 0UL) {
                                        add("${strings.sqlAttempts}: ${day.practiceAttempts}")
                                    }
                                    if (day.practiceSolved > 0UL) {
                                        add("${strings.sqlSolved}: ${day.practiceSolved}")
                                    }
                                }.joinToString(" · ")
                                Text(
                                    text = "${day.date} — $details",
                                    style = MaterialTheme.typography.bodySmall,
                                )
                            }
                        }

                        stats.lastActivityDate?.let { date ->
                            HorizontalDivider()
                            val ago = stats.daysSinceLastActivity?.let(strings::daysAgo)
                            Text(
                                text = buildString {
                                    append("${strings.lastActivity}: $date")
                                    if (ago != null) append(" · $ago")
                                },
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun RankSummaryCard(stats: MobileActivityStats) {
    val strings = LocalUiStrings.current
    val nextRank = stats.nextRankXp

    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(LayoutSpacing.card),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Text(
                text = "${strings.rank}: ${strings.progressRank(stats.rank)}",
                style = MaterialTheme.typography.headlineSmall,
                fontWeight = FontWeight.Bold,
                color = MaterialTheme.colorScheme.primary,
            )
            Text(strings.rankDescription(stats.rank))
            Text(
                text = "${strings.xp}: ${stats.xp}",
                fontWeight = FontWeight.SemiBold,
            )
            LinearProgressIndicator(
                progress = {
                    (stats.rankProgressPercent / 100.0)
                        .toFloat()
                        .coerceIn(0f, 1f)
                },
                modifier = Modifier.fillMaxWidth(),
            )
            if (nextRank != null) {
                Text(
                    text = "${strings.nextRank}: ${nextRank - stats.xp} XP",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun StatisticsDetailScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val stats = controller.statisticsDetail

    TrainerScaffold(
        title = controller.statisticsDetailTitle,
        onBack = controller::back,
    ) { modifier ->
        if (stats == null) {
            EmptyMessage(
                text = strings.statisticsUnavailable,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(LayoutSpacing.section),
        ) {
            item {
                StatisticsCard(
                    stats = stats,
                )
            }

            if (controller.statisticsDetailLimit != null &&
                controller.statisticsDetailLimit != MobileStatisticsLimit.ANY
            ) {
                item {
                    OutlinedButton(
                        onClick = controller::requestClearCurrentQuestionModeStatistics,
                        modifier = Modifier.fillMaxWidth(),
                    ) {
                        Text(strings.clearQuestionModeStatistics)
                    }
                }
            }
        }
    }
}

@Composable
private fun StatisticsNavigationCard(
    title: String,
    stats: MobileTrainingStats?,
    subtitle: String? = null,
    onClick: () -> Unit,
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(LayoutSpacing.card),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(
                modifier = Modifier.weight(1f),
            ) {
                Text(
                    text = title,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                )

                subtitle?.let {
                    Spacer(Modifier.height(2.dp))
                    Text(
                        text = it,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }

            Spacer(Modifier.width(16.dp))

            Text(
                text = statsPercent(stats),
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold,
                color = MaterialTheme.colorScheme.primary,
            )
        }
    }
}

@Composable
private fun StatisticsCard(
    stats: MobileTrainingStats,
) {
    val strings = LocalUiStrings.current

    Card(
        modifier = Modifier.fillMaxWidth(),
    ) {
        Column(
            modifier = Modifier.padding(LayoutSpacing.card),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Text(
                text = strings.completedTests(stats.completedSessions),
                style = MaterialTheme.typography.bodyLarge,
            )
            Text(
                text = strings.cancelledTests(stats.cancelledSessions),
                style = MaterialTheme.typography.bodyLarge,
            )

            HorizontalDivider(
                modifier = Modifier.padding(vertical = 4.dp),
            )

            Text(
                text = strings.answeredQuestions(stats.answeredQuestions),
                style = MaterialTheme.typography.bodyLarge,
            )
            Text(
                text = strings.correctAnswers(stats.correctAnswers),
                style = MaterialTheme.typography.bodyLarge,
            )
            Text(
                text = strings.incorrectAnswers(stats.incorrectAnswers),
                style = MaterialTheme.typography.bodyLarge,
            )

            Spacer(Modifier.height(6.dp))

            Text(
                text = strings.accuracy(formatPercent(stats.accuracyPercent)),
                style = MaterialTheme.typography.headlineSmall,
                fontWeight = FontWeight.Bold,
                color = MaterialTheme.colorScheme.primary,
            )
        }
    }
}

@Composable
private fun TrainerScaffold(
    title: String,
    onBack: () -> Unit,
    content: @Composable (Modifier) -> Unit,
) {
    val strings = LocalUiStrings.current

    Scaffold(
        topBar = {
            Surface(
                tonalElevation = 2.dp,
            ) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 8.dp, vertical = 6.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    TextButton(
                        onClick = onBack,
                    ) {
                        Text(strings.back)
                    }

                    Spacer(Modifier.width(8.dp))

                    Text(
                        text = title,
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.SemiBold,
                        maxLines = 2,
                        overflow = TextOverflow.Ellipsis,
                    )
                }
            }
        },
    ) { padding ->
        content(
            Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(
                    horizontal = LayoutSpacing.screenHorizontal,
                    vertical = LayoutSpacing.screenVertical,
                ),
        )
    }
}

@Composable
private fun EmptyMessage(
    text: String,
    modifier: Modifier,
) {
    Column(
        modifier = modifier,
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text(
            text = text,
            style = MaterialTheme.typography.bodyLarge,
        )
    }
}

private fun topicDisplayNumber(
    topics: List<MobileTopic>,
    index: Int,
): String {
    val topic = topics[index]
    val groupSize = topics.count { candidate ->
        candidate.notesPart == topic.notesPart
    }

    if (groupSize == 1) {
        return topic.notesPart.toString()
    }

    val subtopicNumber = topics
        .take(index + 1)
        .count { candidate ->
            candidate.notesPart == topic.notesPart
        }

    return "${topic.notesPart}.$subtopicNumber"
}

private fun statsPercent(stats: MobileTrainingStats?): String =
    if (stats == null || stats.answeredQuestions == 0UL) {
        "—"
    } else {
        formatPercent(stats.accuracyPercent)
    }

private fun formatSource(
    source: MobileSource,
    strings: UiStrings,
): String {
    val parts = mutableListOf<String>()

    parts += if (source.url == null) {
        strings.courseMaterial
    } else {
        strings.postgresqlDocumentation
    }

    parts += "${source.module} / ${source.section}"
    parts += source.locator
    source.url?.let(parts::add)

    return parts.joinToString("\n")
}

private fun formatPercent(value: Double): String =
    "%.1f%%".format(value)
