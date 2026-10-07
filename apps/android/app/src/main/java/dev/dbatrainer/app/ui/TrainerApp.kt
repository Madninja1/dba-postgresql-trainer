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
import dev.dbatrainer.ffi.MobileAnswerResult
import dev.dbatrainer.ffi.MobileQuestion
import dev.dbatrainer.ffi.MobileQuestionLimit
import dev.dbatrainer.ffi.MobileQuestionType
import dev.dbatrainer.ffi.MobileSource
import dev.dbatrainer.ffi.MobileTopic
import dev.dbatrainer.ffi.MobileTrainingStats
import dev.dbatrainer.ffi.coreVersion

@Composable
fun TrainerApp(controller: TrainerController) {
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
                Text("Незавершённый тест")
            },
            text = {
                Text(
                    "Отвечено ${progress.answeredQuestions} " +
                        "из ${progress.totalQuestions} вопросов.\n\n" +
                        "Продолжить с сохранённого места?",
                )
            },
            confirmButton = {
                Button(
                    onClick = controller::continueSavedSession,
                ) {
                    Text("Продолжить")
                }
            },
            dismissButton = {
                TextButton(
                    onClick = controller::discardSavedSession,
                ) {
                    Text("Отменить тест")
                }
            },
        )
    }

    if (controller.showCancelDialog) {
        AlertDialog(
            onDismissRequest = controller::dismissCancelDialog,
            title = {
                Text("Отменить тест?")
            },
            text = {
                Text(
                    "Прогресс этого теста останется в истории как отменённый. " +
                        "В основную статистику ответы не попадут.",
                )
            },
            confirmButton = {
                Button(
                    onClick = controller::confirmCancelQuiz,
                ) {
                    Text("Отменить тест")
                }
            },
            dismissButton = {
                TextButton(
                    onClick = controller::dismissCancelDialog,
                ) {
                    Text("Продолжить")
                }
            },
        )
    }

    controller.errorMessage?.let { message ->
        AlertDialog(
            onDismissRequest = controller::dismissError,
            title = {
                Text("Ошибка")
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

@Composable
private fun HomeScreen(controller: TrainerController) {
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
            text = "Офлайн-тренажёр администратора PostgreSQL",
            style = MaterialTheme.typography.bodyLarge,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )

        Spacer(Modifier.height(32.dp))

        Button(
            onClick = controller::openTopics,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text("Тест по теме")
        }

        Spacer(Modifier.height(12.dp))

        OutlinedButton(
            onClick = controller::openGeneralQuiz,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text("Общий тест")
        }

        Spacer(Modifier.height(12.dp))

        OutlinedButton(
            onClick = controller::openStatistics,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text("Статистика")
        }

        Spacer(Modifier.height(32.dp))

        Text(
            text = "Rust core ${coreVersion()}",
            style = MaterialTheme.typography.labelMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

@Composable
private fun TopicsScreen(controller: TrainerController) {
    TrainerScaffold(
        title = "Темы",
        onBack = controller::back,
    ) { modifier ->
        if (controller.topics.isEmpty()) {
            EmptyMessage(
                text = "Темы пока не загружены.",
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
    val title = controller.pendingTopic?.title
        ?: "Общий тест"

    TrainerScaffold(
        title = title,
        onBack = controller::back,
    ) { modifier ->
        Column(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text(
                text = "Количество вопросов",
                style = MaterialTheme.typography.titleLarge,
            )

            LimitButton(
                text = "20 вопросов",
                onClick = {
                    controller.startQuiz(
                        MobileQuestionLimit.TWENTY,
                    )
                },
            )

            LimitButton(
                text = "50 вопросов",
                onClick = {
                    controller.startQuiz(
                        MobileQuestionLimit.FIFTY,
                    )
                },
            )

            LimitButton(
                text = "Все вопросы",
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
        title = "Вопрос $currentNumber из ${controller.totalQuestions}",
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
                        "Выберите все правильные варианты"
                    } else {
                        "Выберите один вариант"
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
                    Text("Ответить")
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
    val question = controller.currentQuestion
        ?: return
    val result = controller.feedback
        ?: return

    TrainerScaffold(
        title = if (result.isCorrect) {
            "Верно"
        } else {
            "Неверно"
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
                    title = "Объяснение",
                    text = question.explanation,
                )
            }

            item {
                InfoCard(
                    title = "Источник",
                    text = formatSource(
                        question.source,
                    ),
                )
            }

            item {
                Button(
                    onClick = controller::nextQuestion,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text("Следующий вопрос")
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
    Column(
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text(
            text = "Ваш ответ",
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
                "Правильные ответы"
            } else {
                "Правильный ответ"
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
            text = "Тест завершён",
            style = MaterialTheme.typography.headlineMedium,
            fontWeight = FontWeight.Bold,
        )

        Spacer(Modifier.height(24.dp))

        Text(
            text = "${controller.correctAnswers} из " +
                "${controller.answeredQuestions}",
            style = MaterialTheme.typography.displaySmall,
            color = MaterialTheme.colorScheme.primary,
        )

        Text(
            text = "${formatPercent(accuracy)} правильных ответов",
            style = MaterialTheme.typography.titleMedium,
        )

        Spacer(Modifier.height(32.dp))

        Button(
            onClick = controller::finishResults,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text("На главную")
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
    val overall = controller.overallStatistics

    TrainerScaffold(
        title = "Статистика",
        onBack = controller::back,
    ) { modifier ->
        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                StatisticsNavigationCard(
                    title = "Общая",
                    stats = overall,
                    subtitle = "Все курсы, темы и режимы",
                    onClick = controller::openOverallStatistics,
                )
            }

            item {
                StatisticsNavigationCard(
                    title = "По курсам и темам",
                    stats = overall,
                    subtitle = "DBA-1 → тема → режим",
                    onClick = controller::openStatisticsCourses,
                )
            }

            item {
                StatisticsNavigationCard(
                    title = "По режимам",
                    stats = overall,
                    subtitle = "20 / 50 / все вопросы",
                    onClick = controller::openStatisticsGlobalModes,
                )
            }

            item {
                Spacer(Modifier.height(8.dp))
                OutlinedButton(
                    onClick = controller::clearStatistics,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text("Очистить статистику")
                }
            }
        }
    }
}

@Composable
private fun StatisticsCoursesScreen(controller: TrainerController) {
    TrainerScaffold(
        title = "Статистика → Курсы",
        onBack = controller::back,
    ) { modifier ->
        if (controller.courseStatistics.isEmpty()) {
            EmptyMessage(
                text = "Курсы пока не загружены.",
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
    StatisticsNavigationCard(
        title = row.courseCode.uppercase(),
        stats = row.stats,
        subtitle = "Статистика курса",
        onClick = onClick,
    )
}

@Composable
private fun StatisticsTopicsScreen(controller: TrainerController) {
    val courseCode = controller.selectedStatisticsCourse
        ?: return
    val courseStats = controller.courseStatistics
        .firstOrNull { row -> row.courseCode == courseCode }
        ?.stats

    TrainerScaffold(
        title = "Статистика → ${courseCode.uppercase()}",
        onBack = controller::back,
    ) { modifier ->
        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(10.dp),
        ) {
            item {
                StatisticsNavigationCard(
                    title = "Все темы",
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
    TrainerScaffold(
        title = "Статистика → Режимы",
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
    val stats = controller.statisticsDetail

    TrainerScaffold(
        title = controller.statisticsDetailTitle,
        onBack = controller::back,
    ) { modifier ->
        if (stats == null) {
            EmptyMessage(
                text = "Статистика пока недоступна.",
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
    Card(
        modifier = Modifier.fillMaxWidth(),
    ) {
        Column(
            modifier = Modifier.padding(20.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Text(
                text = "Завершено тестов: ${stats.completedSessions}",
                style = MaterialTheme.typography.bodyLarge,
            )
            Text(
                text = "Отменено тестов: ${stats.cancelledSessions}",
                style = MaterialTheme.typography.bodyLarge,
            )

            HorizontalDivider(
                modifier = Modifier.padding(vertical = 4.dp),
            )

            Text(
                text = "Отвечено вопросов: ${stats.answeredQuestions}",
                style = MaterialTheme.typography.bodyLarge,
            )
            Text(
                text = "Правильных ответов: ${stats.correctAnswers}",
                style = MaterialTheme.typography.bodyLarge,
            )
            Text(
                text = "Неправильных ответов: ${stats.incorrectAnswers}",
                style = MaterialTheme.typography.bodyLarge,
            )

            Spacer(Modifier.height(6.dp))

            Text(
                text = "Точность: ${formatPercent(stats.accuracyPercent)}",
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
                        Text("Назад")
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

private fun formatSource(source: MobileSource): String {
    val parts = mutableListOf<String>()

    parts += if (source.url == null) {
        "Материал курса"
    } else {
        "Документация PostgreSQL"
    }

    parts += "${source.module} / ${source.section}"
    parts += source.locator
    source.url?.let(parts::add)

    return parts.joinToString("\n")
}

private fun formatPercent(value: Double): String =
    "%.1f%%".format(value)
