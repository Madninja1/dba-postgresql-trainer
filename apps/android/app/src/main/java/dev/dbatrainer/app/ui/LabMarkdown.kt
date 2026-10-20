package dev.dbatrainer.app.ui

import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

internal sealed interface LabMarkdownBlock {
    data class Heading(
        val level: Int,
        val text: String,
    ) : LabMarkdownBlock

    data class Paragraph(
        val text: String,
    ) : LabMarkdownBlock

    data class Bullet(
        val text: String,
    ) : LabMarkdownBlock

    data class Numbered(
        val number: String,
        val text: String,
    ) : LabMarkdownBlock

    data class Quote(
        val text: String,
    ) : LabMarkdownBlock

    data class Code(
        val language: String?,
        val lines: List<String>,
    ) : LabMarkdownBlock

    data class Table(
        val rows: List<List<String>>,
    ) : LabMarkdownBlock

    data object Divider : LabMarkdownBlock
}

internal fun parseLabMarkdown(markdown: String): List<LabMarkdownBlock> {
    val lines = markdown.lines()
    val blocks = mutableListOf<LabMarkdownBlock>()
    var index = 0
    var firstContentSeen = false

    while (index < lines.size) {
        val rawLine = lines[index].trimEnd()
        val line = rawLine.trim()

        if (line.isEmpty()) {
            index += 1
            continue
        }

        if (!firstContentSeen && line.startsWith("# ")) {
            firstContentSeen = true
            index += 1
            continue
        }

        firstContentSeen = true

        if (line.startsWith("```")) {
            val language = line.removePrefix("```").trim().ifEmpty { null }
            val codeLines = mutableListOf<String>()
            index += 1

            while (index < lines.size && !lines[index].trim().startsWith("```")) {
                codeLines += lines[index].trimEnd()
                index += 1
            }

            if (index < lines.size) {
                index += 1
            }

            blocks += LabMarkdownBlock.Code(language, codeLines)
            continue
        }

        val headingLevel = line.takeWhile { character -> character == '#' }.length
        if (headingLevel in 1..6 && line.getOrNull(headingLevel) == ' ') {
            blocks += LabMarkdownBlock.Heading(
                level = headingLevel,
                text = cleanInlineMarkdown(line.drop(headingLevel + 1)),
            )
            index += 1
            continue
        }

        if (
            line.startsWith('|') &&
            index + 1 < lines.size &&
            isMarkdownTableSeparator(lines[index + 1])
        ) {
            val rows = mutableListOf<List<String>>()
            rows += markdownTableCells(line)
            index += 2

            while (index < lines.size && lines[index].trim().startsWith('|')) {
                val candidate = lines[index].trim()
                if (!isMarkdownTableSeparator(candidate)) {
                    rows += markdownTableCells(candidate)
                }
                index += 1
            }

            blocks += LabMarkdownBlock.Table(rows)
            continue
        }

        if (line.startsWith("> ")) {
            blocks += LabMarkdownBlock.Quote(
                cleanInlineMarkdown(line.removePrefix("> ")),
            )
            index += 1
            continue
        }

        if (line.startsWith("- ") || line.startsWith("* ")) {
            blocks += LabMarkdownBlock.Bullet(
                cleanInlineMarkdown(line.drop(2)),
            )
            index += 1
            continue
        }

        val numberedItem = numberedListItem(line)
        if (numberedItem != null) {
            val (number, item) = numberedItem
            blocks += LabMarkdownBlock.Numbered(
                number = number,
                text = cleanInlineMarkdown(item),
            )
            index += 1
            continue
        }

        if (line == "---" || line == "***" || line == "___") {
            blocks += LabMarkdownBlock.Divider
            index += 1
            continue
        }

        blocks += LabMarkdownBlock.Paragraph(cleanInlineMarkdown(line))
        index += 1
    }

    return blocks
}

@Composable
internal fun LabMarkdownContent(
    markdown: String,
    modifier: Modifier = Modifier,
) {
    Column(
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        parseLabMarkdown(markdown).forEach { block ->
            when (block) {
                is LabMarkdownBlock.Heading -> {
                    Text(
                        text = block.text,
                        style = when (block.level) {
                            1 -> MaterialTheme.typography.headlineSmall
                            2 -> MaterialTheme.typography.titleLarge
                            else -> MaterialTheme.typography.titleMedium
                        },
                        fontWeight = FontWeight.SemiBold,
                    )
                }

                is LabMarkdownBlock.Paragraph -> {
                    Text(
                        text = block.text,
                        style = MaterialTheme.typography.bodyMedium,
                    )
                }

                is LabMarkdownBlock.Bullet -> {
                    Text(
                        text = "• ${block.text}",
                        style = MaterialTheme.typography.bodyMedium,
                    )
                }

                is LabMarkdownBlock.Numbered -> {
                    Text(
                        text = "${block.number}. ${block.text}",
                        style = MaterialTheme.typography.bodyMedium,
                    )
                }

                is LabMarkdownBlock.Quote -> {
                    Surface(
                        modifier = Modifier.fillMaxWidth(),
                        tonalElevation = 1.dp,
                    ) {
                        Text(
                            text = block.text,
                            modifier = Modifier.padding(12.dp),
                            style = MaterialTheme.typography.bodyMedium,
                            fontStyle = FontStyle.Italic,
                        )
                    }
                }

                is LabMarkdownBlock.Code -> {
                    Surface(
                        modifier = Modifier.fillMaxWidth(),
                        tonalElevation = 2.dp,
                    ) {
                        Column(
                            modifier = Modifier
                                .horizontalScroll(rememberScrollState())
                                .padding(12.dp),
                            verticalArrangement = Arrangement.spacedBy(6.dp),
                        ) {
                            block.language?.let { language ->
                                Text(
                                    text = language,
                                    style = MaterialTheme.typography.labelSmall,
                                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                                )
                            }

                            Text(
                                text = block.lines.joinToString("\n"),
                                style = MaterialTheme.typography.bodySmall,
                                fontFamily = FontFamily.Monospace,
                            )
                        }
                    }
                }

                is LabMarkdownBlock.Table -> {
                    Surface(
                        modifier = Modifier.fillMaxWidth(),
                        tonalElevation = 1.dp,
                    ) {
                        Text(
                            text = block.rows.joinToString("\n") { row ->
                                row.joinToString("  │  ")
                            },
                            modifier = Modifier
                                .horizontalScroll(rememberScrollState())
                                .padding(12.dp),
                            style = MaterialTheme.typography.bodySmall,
                            fontFamily = FontFamily.Monospace,
                        )
                    }
                }

                LabMarkdownBlock.Divider -> HorizontalDivider()
            }
        }
    }
}

private fun numberedListItem(line: String): Pair<String, String>? {
    val separator = line.indexOf(". ")
    if (separator <= 0) {
        return null
    }

    val number = line.substring(0, separator)
    if (!number.all { character -> character.isDigit() }) {
        return null
    }

    return number to line.substring(separator + 2)
}

private fun isMarkdownTableSeparator(line: String): Boolean {
    if (!line.trim().startsWith('|')) {
        return false
    }

    val cells = markdownTableCells(line)
    return cells.isNotEmpty() && cells.all { cell ->
        val trimmed = cell.trim(':')
        trimmed.length >= 3 && trimmed.all { character -> character == '-' }
    }
}

private fun markdownTableCells(line: String): List<String> =
    line
        .trim()
        .trim('|')
        .split('|')
        .map { cell -> cleanInlineMarkdown(cell.trim()) }

private fun cleanInlineMarkdown(value: String): String {
    val withoutLinks = MARKDOWN_LINK.replace(value) { match ->
        val label = match.groupValues[1]
        val url = match.groupValues[2]
        "$label ($url)"
    }

    return withoutLinks
        .replace("**", "")
        .replace("__", "")
        .replace("`", "")
}

private val MARKDOWN_LINK = Regex("\\[([^]]+)]\\(([^)]+)\\)")
