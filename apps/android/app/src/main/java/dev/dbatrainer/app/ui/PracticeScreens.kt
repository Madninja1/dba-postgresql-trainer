package dev.dbatrainer.app.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import dev.dbatrainer.app.PracticePane
import dev.dbatrainer.app.TrainerController
import dev.dbatrainer.app.UiStrings
import dev.dbatrainer.ffi.MobilePracticeExecutionResult
import dev.dbatrainer.ffi.MobilePracticeRule
import dev.dbatrainer.ffi.MobilePracticeRuleKind
import dev.dbatrainer.ffi.MobilePracticeTable
import kotlinx.coroutines.delay

@Composable
fun PracticeCategoriesScreen(controller: TrainerController) {
    val strings = controller.strings

    PracticeScaffold(
        title = strings.practice,
        backLabel = strings.back,
        onBack = controller::back,
    ) { modifier ->
        if (controller.practiceTasks.isEmpty()) {
            PracticeEmpty(
                text = strings.practiceEmpty,
                modifier = modifier,
            )
            return@PracticeScaffold
        }

        val stats = controller.practiceStats
        LazyColumn(
            modifier = modifier,
            contentPadding = PaddingValues(vertical = 4.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                Text(
                    text = strings.practiceSubtitle,
                    style = MaterialTheme.typography.bodyLarge,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }

            stats?.let { practiceStats ->
                item {
                    Card(
                        modifier = Modifier.fillMaxWidth(),
                    ) {
                        Column(
                            modifier = Modifier.padding(16.dp),
                            verticalArrangement = Arrangement.spacedBy(8.dp),
                        ) {
                            Text(
                                text = strings.practiceProgress,
                                style = MaterialTheme.typography.titleMedium,
                                fontWeight = FontWeight.SemiBold,
                            )
                            LinearProgressIndicator(
                                progress = {
                                    (practiceStats.solvedPercent / 100.0)
                                        .toFloat()
                                        .coerceIn(0f, 1f)
                                },
                                modifier = Modifier.fillMaxWidth(),
                            )
                            Text(
                                "${strings.practiceSolved}: ${practiceStats.solvedTasks}/${practiceStats.totalTasks} " +
                                    "(${"%.1f".format(practiceStats.solvedPercent)}%)",
                            )
                            Text(
                                "${strings.practiceStarted}: ${practiceStats.startedTasks} · " +
                                    "${strings.practiceAttempts}: ${practiceStats.totalAttempts} · " +
                                    "${strings.practiceFirstTry}: ${practiceStats.solvedFirstTry}",
                                style = MaterialTheme.typography.bodyMedium,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                        }
                    }
                }
            }

            controller.resumablePracticeTask?.let { task ->
                item {
                    Button(
                        onClick = controller::resumePractice,
                        modifier = Modifier.fillMaxWidth(),
                    ) {
                        Text(
                            text = "${strings.practiceContinue}: [${task.category}] ${task.title}",
                            maxLines = 2,
                            overflow = TextOverflow.Ellipsis,
                        )
                    }
                }
            }

            item {
                Text(
                    text = strings.practiceCategories,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.SemiBold,
                )
            }

            items(
                items = controller.practiceCategories,
                key = { category -> category },
            ) { category ->
                val categoryStats = controller.practiceCategoryStats(category)
                Card(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { controller.choosePracticeCategory(category) },
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(4.dp),
                    ) {
                        Text(
                            text = category,
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        if (categoryStats != null) {
                            Text(
                                text = "${strings.practiceSolved}: ${categoryStats.solvedTasks}/${categoryStats.totalTasks} · " +
                                    "${strings.practiceAttempts}: ${categoryStats.totalAttempts}",
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
fun PracticeTasksScreen(controller: TrainerController) {
    val strings = controller.strings
    val category = controller.selectedPracticeCategory.orEmpty()
    val tasks = controller.practiceTasksForSelectedCategory

    PracticeScaffold(
        title = "${strings.practice} → $category",
        backLabel = strings.back,
        onBack = controller::back,
    ) { modifier ->
        if (tasks.isEmpty()) {
            PracticeEmpty(
                text = strings.practiceEmpty,
                modifier = modifier,
            )
            return@PracticeScaffold
        }

        LazyColumn(
            modifier = modifier,
            contentPadding = PaddingValues(vertical = 4.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                Text(
                    text = strings.practiceTasks,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.SemiBold,
                )
            }

            items(
                items = tasks,
                key = { task -> task.id },
            ) { task ->
                val progress = controller.practiceProgressFor(task.id)
                val marker = when {
                    progress?.solved == true -> "✓"
                    progress?.started == true -> "•"
                    else -> "○"
                }
                Card(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { controller.openPracticeTask(task) },
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(6.dp),
                    ) {
                        Text(
                            text = "$marker ${task.title}",
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        Text(
                            text = "${strings.practiceDifficulty(task.difficulty)} · " +
                                "${strings.practiceAttempts}: ${progress?.attempts ?: 0UL}",
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                        Text(
                            text = task.description,
                            maxLines = 3,
                            overflow = TextOverflow.Ellipsis,
                            style = MaterialTheme.typography.bodyMedium,
                        )
                    }
                }
            }
        }
    }
}

@Composable
fun PracticeWorkbenchScreen(controller: TrainerController) {
    val task = controller.currentPracticeTask
    val strings = controller.strings

    if (task == null) {
        PracticeScaffold(
            title = strings.practice,
            backLabel = strings.back,
            onBack = controller::back,
        ) { modifier ->
            PracticeEmpty(
                text = strings.practiceEmpty,
                modifier = modifier,
            )
        }
        return
    }

    LaunchedEffect(task.id, controller.practiceSql) {
        delay(700)
        controller.persistPracticeDraft()
    }

    PracticeScaffold(
        title = task.title,
        backLabel = strings.back,
        onBack = controller::back,
    ) { modifier ->
        LazyColumn(
            modifier = modifier,
            contentPadding = PaddingValues(bottom = 24.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                Card(
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(6.dp),
                    ) {
                        Text(
                            text = task.description,
                            style = MaterialTheme.typography.bodyLarge,
                        )
                        Text(
                            text = "${task.category} · ${strings.practiceDifficulty(task.difficulty)} · " +
                                "${strings.practiceAttempts}: ${controller.practiceProgressFor(task.id)?.attempts ?: 0UL}",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }

            item {
                PracticePaneTabs(controller)
            }

            item {
                when (controller.practicePane) {
                    PracticePane.Editor -> PracticeEditorPane(controller)
                    PracticePane.Schema -> PracticeSchemaPane(controller)
                    PracticePane.Data -> PracticeDataPane(controller)
                    PracticePane.Result -> PracticeResultPane(controller)
                    PracticePane.Learning -> PracticeLearningPane(controller)
                }
            }

            item {
                PracticeActions(controller)
            }
        }
    }
}

@Composable
private fun PracticePaneTabs(controller: TrainerController) {
    val strings = controller.strings
    val panes = listOf(
        PracticePane.Editor to strings.practiceEditor,
        PracticePane.Schema to strings.practiceSchema,
        PracticePane.Data to strings.practiceData,
        PracticePane.Result to strings.practiceResult,
        PracticePane.Learning to strings.practiceLearning,
    )

    LazyRow(
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        items(
            items = panes,
            key = { item -> item.first.name },
        ) { (pane, label) ->
            if (controller.practicePane == pane) {
                Button(
                    onClick = { controller.selectPracticePane(pane) },
                ) {
                    Text(label)
                }
            } else {
                OutlinedButton(
                    onClick = { controller.selectPracticePane(pane) },
                ) {
                    Text(label)
                }
            }
        }
    }
}

@Composable
private fun PracticeEditorPane(controller: TrainerController) {
    val strings = controller.strings

    OutlinedTextField(
        value = controller.practiceSql,
        onValueChange = controller::updatePracticeSql,
        modifier = Modifier
            .fillMaxWidth()
            .height(280.dp),
        label = { Text(strings.practiceEditor) },
        textStyle = MaterialTheme.typography.bodyMedium.copy(
            fontFamily = FontFamily.Monospace,
        ),
        minLines = 8,
    )
}

@Composable
private fun PracticeSchemaPane(controller: TrainerController) {
    val strings = controller.strings
    val tables = controller.practiceSchema?.tables.orEmpty()

    if (tables.isEmpty()) {
        Text(
            text = "(empty schema)",
            fontFamily = FontFamily.Monospace,
        )
        return
    }

    Column(
        verticalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        tables.forEach { table ->
            PracticeTableCard(
                table = table,
                strings = strings,
                onOpen = { controller.openPracticeTable(table.name) },
            )
        }
    }
}

@Composable
private fun PracticeTableCard(
    table: MobilePracticeTable,
    strings: UiStrings,
    onOpen: () -> Unit,
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
    ) {
        Column(
            modifier = Modifier.padding(14.dp),
            verticalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            Text(
                text = table.name,
                style = MaterialTheme.typography.titleMedium,
                fontFamily = FontFamily.Monospace,
                fontWeight = FontWeight.SemiBold,
            )
            table.columns.forEach { column ->
                val flags = buildList {
                    if (column.primaryKey) add("PK")
                    if (column.notNull) add("NOT NULL")
                }
                val suffix = buildString {
                    if (flags.isNotEmpty()) append(" [${flags.joinToString()}]")
                    column.defaultValue?.let { value -> append(" DEFAULT $value") }
                }
                Text(
                    text = "${column.name}  ${column.dataType}$suffix",
                    fontFamily = FontFamily.Monospace,
                    style = MaterialTheme.typography.bodySmall,
                )
            }
            Spacer(Modifier.height(4.dp))
            OutlinedButton(onClick = onOpen) {
                Text(strings.practiceOpenTable)
            }
        }
    }
}

@Composable
private fun PracticeDataPane(controller: TrainerController) {
    val strings = controller.strings
    val data = controller.practiceTableData

    if (data == null) {
        Text(strings.practiceOpenTable)
        return
    }

    Column(
        verticalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        Text(
            text = "${strings.practiceData}: ${data.table}",
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.SemiBold,
        )
        PracticeResultTable(
            result = data.result,
            strings = strings,
        )
        Row(
            horizontalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            OutlinedButton(
                onClick = controller::previousPracticeTablePage,
                enabled = data.offset > 0UL,
            ) {
                Text(strings.practicePreviousPage)
            }
            OutlinedButton(
                onClick = controller::nextPracticeTablePage,
                enabled = data.hasMore,
            ) {
                Text(strings.practiceNextPage)
            }
        }
    }
}

@Composable
private fun PracticeResultPane(controller: TrainerController) {
    val strings = controller.strings

    Column(
        verticalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        controller.practiceCheck?.let { check ->
            Card(
                modifier = Modifier.fillMaxWidth(),
            ) {
                Column(
                    modifier = Modifier.padding(14.dp),
                    verticalArrangement = Arrangement.spacedBy(6.dp),
                ) {
                    Text(
                        text = if (check.isCorrect) {
                            strings.practiceCorrect
                        } else {
                            strings.practiceIncorrect
                        },
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.Bold,
                    )
                    Text(
                        "${strings.practiceActualRows}: ${check.actualRows} · " +
                            "${strings.practiceExpectedRows}: ${check.expectedRows}",
                    )
                    if (check.failedChecks.isNotEmpty()) {
                        Text(
                            text = strings.practiceFailedChecks,
                            fontWeight = FontWeight.SemiBold,
                        )
                        check.failedChecks.forEach { failed ->
                            Text("• $failed")
                        }
                    }
                }
            }
        }

        val output = controller.practiceOutput
        if (output != null) {
            PracticeResultTable(
                result = output,
                strings = strings,
            )
        } else if (controller.practiceCheck == null) {
            Text(strings.practiceNoOutput)
        }
    }
}

@Composable
private fun PracticeLearningPane(controller: TrainerController) {
    val strings = controller.strings
    val task = controller.currentPracticeTask ?: return

    Column(
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        if (task.rules.isNotEmpty()) {
            Card(
                modifier = Modifier.fillMaxWidth(),
            ) {
                Column(
                    modifier = Modifier.padding(14.dp),
                    verticalArrangement = Arrangement.spacedBy(4.dp),
                ) {
                    Text(
                        text = strings.practiceRules,
                        fontWeight = FontWeight.SemiBold,
                    )
                    task.rules.forEach { rule ->
                        Text("• ${practiceRuleText(rule, strings)}")
                    }
                }
            }
        }

        if (controller.practiceHintCount > 0) {
            Card(
                modifier = Modifier.fillMaxWidth(),
            ) {
                Column(
                    modifier = Modifier.padding(14.dp),
                    verticalArrangement = Arrangement.spacedBy(6.dp),
                ) {
                    Text(
                        text = strings.practiceHint,
                        fontWeight = FontWeight.SemiBold,
                    )
                    task.hints
                        .take(controller.practiceHintCount)
                        .forEachIndexed { index, hint ->
                            Text("${index + 1}. $hint")
                        }
                }
            }
        }

        if (controller.practiceShowSolution) {
            Card(
                modifier = Modifier.fillMaxWidth(),
            ) {
                Column(
                    modifier = Modifier.padding(14.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    Text(
                        text = strings.practiceSolution,
                        fontWeight = FontWeight.SemiBold,
                    )
                    Text(
                        text = task.solutionSql,
                        fontFamily = FontFamily.Monospace,
                    )
                    task.solutionExplanation?.let { explanation ->
                        HorizontalDivider()
                        Text(explanation)
                    }
                }
            }
        } else if (!controller.practiceSolutionAvailable) {
            Text(
                text = strings.practiceSolutionLocked,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }

        controller.practiceMessage?.let { message ->
            Text(
                text = message,
                color = MaterialTheme.colorScheme.primary,
            )
        }

        controller.practiceError?.let { error ->
            Card(
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(
                    text = error,
                    modifier = Modifier.padding(14.dp),
                    fontFamily = FontFamily.Monospace,
                    color = MaterialTheme.colorScheme.error,
                )
            }
        }
    }
}

@Composable
private fun PracticeActions(controller: TrainerController) {
    val strings = controller.strings

    LazyRow(
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        item {
            Button(onClick = controller::runPracticeSql) {
                Text(strings.practiceRun)
            }
        }
        item {
            Button(onClick = controller::checkPracticeSql) {
                Text(strings.practiceCheck)
            }
        }
        item {
            OutlinedButton(onClick = controller::resetPracticeSandbox) {
                Text(strings.practiceReset)
            }
        }
        item {
            OutlinedButton(onClick = controller::showPracticeHint) {
                Text(strings.practiceHint)
            }
        }
        item {
            OutlinedButton(onClick = controller::togglePracticeSolution) {
                Text(strings.practiceSolution)
            }
        }
        item {
            OutlinedButton(onClick = controller::nextPracticeTask) {
                Text(strings.practiceNext)
            }
        }
    }
}

@Composable
private fun PracticeResultTable(
    result: MobilePracticeExecutionResult,
    strings: UiStrings,
) {
    val affectedRows = result.affectedRows
    if (affectedRows != null) {
        Text("${strings.practiceAffectedRows}: $affectedRows")
        return
    }

    if (result.columns.isEmpty()) {
        Text(strings.practiceNoOutput)
        return
    }

    val scrollState = rememberScrollState()
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .horizontalScroll(scrollState),
    ) {
        PracticeResultRow(
            values = result.columns,
            header = true,
        )
        HorizontalDivider()
        result.rows.forEach { row ->
            PracticeResultRow(
                values = row.values,
                header = false,
            )
        }
        Spacer(Modifier.height(4.dp))
        Text(
            text = "${strings.practiceRows}: ${result.rows.size}",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

@Composable
private fun PracticeResultRow(
    values: List<String>,
    header: Boolean,
) {
    Row {
        values.forEach { value ->
            Text(
                text = value,
                modifier = Modifier
                    .widthIn(min = 110.dp, max = 240.dp)
                    .padding(horizontal = 8.dp, vertical = 5.dp),
                fontFamily = FontFamily.Monospace,
                fontWeight = if (header) FontWeight.SemiBold else FontWeight.Normal,
                maxLines = 4,
                overflow = TextOverflow.Ellipsis,
            )
        }
    }
}

private fun practiceRuleText(
    rule: MobilePracticeRule,
    strings: UiStrings,
): String = when (rule.kind) {
    MobilePracticeRuleKind.REQUIRE_KEYWORD ->
        "${strings.practiceRequireKeyword}: ${rule.value.orEmpty()}"
    MobilePracticeRuleKind.FORBID_KEYWORD ->
        "${strings.practiceForbidKeyword}: ${rule.value.orEmpty()}"
    MobilePracticeRuleKind.FORBID_SELECT_STAR -> strings.practiceForbidSelectStar
}

@Composable
private fun PracticeScaffold(
    title: String,
    backLabel: String,
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
                    TextButton(onClick = onBack) {
                        Text(backLabel)
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
                .padding(horizontal = 16.dp, vertical = 12.dp),
        )
    }
}

@Composable
private fun PracticeEmpty(
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

@Composable
fun PracticeStatisticsScreen(controller: TrainerController) {
    val strings = controller.strings
    val stats = controller.practiceStats

    PracticeScaffold(
        title = "${strings.statistics} → ${strings.practice}",
        backLabel = strings.back,
        onBack = controller::back,
    ) { modifier ->
        if (stats == null || stats.totalTasks == 0UL) {
            PracticeEmpty(
                text = strings.practiceEmpty,
                modifier = modifier,
            )
            return@PracticeScaffold
        }

        LazyColumn(
            modifier = modifier,
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                Card(
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = strings.practiceProgress,
                            style = MaterialTheme.typography.titleLarge,
                            fontWeight = FontWeight.SemiBold,
                        )
                        LinearProgressIndicator(
                            progress = {
                                (stats.solvedPercent / 100.0)
                                    .toFloat()
                                    .coerceIn(0f, 1f)
                            },
                            modifier = Modifier.fillMaxWidth(),
                        )
                        Text("${strings.practiceSolved}: ${stats.solvedTasks}/${stats.totalTasks} (${"%.1f".format(stats.solvedPercent)}%)")
                        Text("${strings.practiceStarted}: ${stats.startedTasks}")
                        Text("${strings.practiceAttempts}: ${stats.totalAttempts}")
                        Text("${strings.practiceFirstTry}: ${stats.solvedFirstTry}")
                    }
                }
            }

            items(
                items = stats.categories,
                key = { category -> category.category },
            ) { category ->
                Card(
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(5.dp),
                    ) {
                        Text(
                            text = category.category,
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold,
                        )
                        Text("${strings.practiceSolved}: ${category.solvedTasks}/${category.totalTasks}")
                        Text("${strings.practiceStarted}: ${category.startedTasks}")
                        Text("${strings.practiceAttempts}: ${category.totalAttempts}")
                        Text("${strings.practiceFirstTry}: ${category.solvedFirstTry}")
                    }
                }
            }
        }
    }
}
