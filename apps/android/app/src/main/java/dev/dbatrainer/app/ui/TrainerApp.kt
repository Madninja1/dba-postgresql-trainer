package dev.dbatrainer.app.ui

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
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
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import dev.dbatrainer.app.AppScreen
import dev.dbatrainer.app.CourseStatisticsRow
import dev.dbatrainer.app.ModeStatisticsRow
import dev.dbatrainer.app.StatisticsPage
import dev.dbatrainer.app.TopicStatisticsRow
import dev.dbatrainer.app.TrainerController
import dev.dbatrainer.app.UiLanguage
import dev.dbatrainer.app.UiStrings
import dev.dbatrainer.app.uiStrings
import dev.dbatrainer.ffi.MobileAnswerResult
import dev.dbatrainer.ffi.MobileQuestion
import dev.dbatrainer.ffi.MobileQuestionLimit
import dev.dbatrainer.ffi.MobileQuestionType
import dev.dbatrainer.ffi.MobileSource
import dev.dbatrainer.ffi.MobileTopic
import dev.dbatrainer.ffi.MobileTrainingStats
import dev.dbatrainer.ffi.coreVersion

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
                AppScreen.Home -> HomeScreen(controller)
                AppScreen.Topics -> TopicsScreen(controller)
                AppScreen.Courses -> CoursesScreen(controller)
                AppScreen.Limit -> LimitScreen(controller)
                AppScreen.Quiz -> QuizScreen(controller)
                AppScreen.Feedback -> FeedbackScreen(controller)
                AppScreen.Results -> ResultsScreen(controller)
                AppScreen.Statistics -> StatisticsScreen(controller)
            }
        }
    }

    controller.resumeSession?.let { progress ->
        AlertDialog(
            onDismissRequest = {},
            title = {
                Text(strings.unfinishedQuiz)
            },
            text = {
                Text(
                    strings.resumeMessage(
                        progress.answeredQuestions,
                        progress.totalQuestions,
                    ),
                )
            },
            confirmButton = {
                Button(
                    onClick = controller::continueSavedSession,
                ) {
                    Text(strings.continueQuiz)
                }
            },
            dismissButton = {
                TextButton(
                    onClick = controller::discardSavedSession,
                ) {
                    Text(strings.cancelQuiz)
                }
            },
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
                Button(
                    onClick = controller::confirmCancelQuiz,
                ) {
                    Text(strings.cancelQuiz)
                }
            },
            dismissButton = {
                TextButton(
                    onClick = controller::dismissCancelDialog,
                ) {
                    Text(strings.continueQuiz)
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
private fun HomeScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(24.dp),
        verticalArrangement = Arrangement.Center,
    ) {
        Text(
            text = "DBA Trainer",
            style = MaterialTheme.typography.headlineLarge,
            fontWeight = FontWeight.Bold,
        )

        Text(
            text = strings.appSubtitle,
            style = MaterialTheme.typography.bodyLarge,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )

        Spacer(Modifier.height(32.dp))

        Button(
            onClick = controller::openTopics,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text(strings.topicQuiz)
        }

        Spacer(Modifier.height(12.dp))

        OutlinedButton(
            onClick = controller::openGeneralQuiz,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text(strings.generalQuiz)
        }

        Spacer(Modifier.height(12.dp))

        OutlinedButton(
            onClick = controller::openStatistics,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text(strings.statistics)
        }

        Spacer(Modifier.height(24.dp))

        Text(
            text = strings.language,
            style = MaterialTheme.typography.labelLarge,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )

        Spacer(Modifier.height(8.dp))

        Row(
            horizontalArrangement = Arrangement.spacedBy(8.dp),
        ) {
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

        Spacer(Modifier.height(24.dp))

        Text(
            text = "${strings.rustCore} ${coreVersion()}",
            style = MaterialTheme.typography.labelMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
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
private fun TopicsScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    TrainerScaffold(
        title = strings.topics,
        onBack = controller::back,
    ) { modifier ->
        if (controller.topics.isEmpty()) {
            EmptyMessage(
                text = strings.topicsEmpty,
                modifier = modifier,
            )
            return@TrainerScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            items(
                items = controller.topics,
                key = { topic -> topic.id },
            ) { topic ->
                TopicCard(
                    topic = topic,
                    onClick = {
                        controller.chooseTopic(topic)
                    },
                )
            }
        }
    }
}

@Composable
private fun CoursesScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current

    TrainerScaffold(
        title = strings.generalQuiz,
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
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                Text(
                    text = strings.chooseCourse,
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
                        .clickable { controller.chooseCourse(courseCode) },
                ) {
                    Column(
                        modifier = Modifier.padding(20.dp),
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
                    }
                }
            }
        }
    }
}

@Composable
private fun TopicCard(
    topic: MobileTopic,
    onClick: () -> Unit,
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Column(
            modifier = Modifier.padding(20.dp),
        ) {
            Text(
                text = topic.courseCode.uppercase(),
                style = MaterialTheme.typography.labelLarge,
                color = MaterialTheme.colorScheme.primary,
            )

            Spacer(Modifier.height(4.dp))

            Text(
                text = topic.title,
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )

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
private fun LimitScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val title = controller.pendingTopic?.title
        ?: controller.pendingCourse?.uppercase()
        ?: strings.generalQuiz

    TrainerScaffold(
        title = title,
        onBack = controller::back,
    ) { modifier ->
        Column(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text(
                text = strings.questionCount,
                style = MaterialTheme.typography.titleLarge,
            )

            LimitButton(
                text = strings.twentyQuestions,
                onClick = {
                    controller.startQuiz(
                        MobileQuestionLimit.TWENTY,
                    )
                },
            )

            LimitButton(
                text = strings.fiftyQuestions,
                onClick = {
                    controller.startQuiz(
                        MobileQuestionLimit.FIFTY,
                    )
                },
            )

            LimitButton(
                text = strings.allQuestions,
                onClick = {
                    controller.startQuiz(
                        MobileQuestionLimit.ALL,
                    )
                },
            )
        }
    }
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
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                LinearProgressIndicator(
                    progress = { progress },
                    modifier = Modifier.fillMaxWidth(),
                )
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
            verticalArrangement = Arrangement.spacedBy(16.dp),
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
        verticalArrangement = Arrangement.spacedBy(12.dp),
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
            modifier = Modifier.padding(16.dp),
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
private fun ResultsScreen(controller: TrainerController) {
    val strings = LocalUiStrings.current
    val accuracy = if (controller.answeredQuestions == 0) {
        0.0
    } else {
        controller.correctAnswers.toDouble() /
            controller.answeredQuestions.toDouble() * 100.0
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(24.dp),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text(
            text = strings.quizCompleted,
            style = MaterialTheme.typography.headlineMedium,
            fontWeight = FontWeight.Bold,
        )

        Spacer(Modifier.height(24.dp))

        Text(
            text = strings.resultScore(
                controller.correctAnswers,
                controller.answeredQuestions,
            ),
            style = MaterialTheme.typography.displaySmall,
            color = MaterialTheme.colorScheme.primary,
        )

        Text(
            text = strings.percentCorrect(formatPercent(accuracy)),
            style = MaterialTheme.typography.titleMedium,
        )

        Spacer(Modifier.height(32.dp))

        Button(
            onClick = controller::finishResults,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text(strings.home)
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
            verticalArrangement = Arrangement.spacedBy(12.dp),
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
                Spacer(Modifier.height(8.dp))
                OutlinedButton(
                    onClick = controller::clearStatistics,
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
            verticalArrangement = Arrangement.spacedBy(10.dp),
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
        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(10.dp),
        ) {
            item {
                StatisticsNavigationCard(
                    title = strings.allTopics,
                    stats = courseStats,
                    subtitle = courseCode.uppercase(),
                    onClick = controller::openCourseAllTopicsModes,
                )
            }

            items(
                items = controller.topicStatistics,
                key = { row -> row.topic.id },
            ) { row ->
                TopicStatisticsCard(
                    row = row,
                    onClick = {
                        controller.openTopicModes(row.topic)
                    },
                )
            }
        }
    }
}

@Composable
private fun TopicStatisticsCard(
    row: TopicStatisticsRow,
    onClick: () -> Unit,
) {
    StatisticsNavigationCard(
        title = row.topic.title,
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
    TrainerScaffold(
        title = controller.statisticsScopeTitle,
        onBack = controller::back,
    ) { modifier ->
        StatisticsModesList(
            rows = controller.scopeModeStatistics,
            modifier = modifier,
            onClick = controller::openScopeModeDetail,
        )
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
        verticalArrangement = Arrangement.spacedBy(10.dp),
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
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                StatisticsCard(
                    stats = stats,
                )
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
                .padding(18.dp),
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
            modifier = Modifier.padding(20.dp),
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
                .padding(horizontal = 16.dp),
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
