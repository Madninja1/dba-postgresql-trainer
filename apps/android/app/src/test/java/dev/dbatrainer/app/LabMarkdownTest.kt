package dev.dbatrainer.app

import dev.dbatrainer.app.ui.LabMarkdownBlock
import dev.dbatrainer.app.ui.parseLabMarkdown
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class LabMarkdownTest {
    @Test
    fun parser_skips_duplicate_h1_and_preserves_structure() {
        val blocks = parseLabMarkdown(
            """
            # Lab title

            ## Step one

            > **Important**

            - run `psql`

            1. First action

            ```bash
            psql -l
            ```

            | Name | Value |
            | --- | ---: |
            | DB | postgres |
            """.trimIndent(),
        )

        assertTrue(blocks.none { block ->
            block is LabMarkdownBlock.Heading && block.text == "Lab title"
        })
        assertTrue(blocks.any { block ->
            block is LabMarkdownBlock.Heading && block.text == "Step one"
        })
        assertTrue(blocks.any { block ->
            block is LabMarkdownBlock.Quote && block.text == "Important"
        })
        assertTrue(blocks.any { block ->
            block is LabMarkdownBlock.Bullet && block.text == "run psql"
        })
        assertTrue(blocks.any { block ->
            block is LabMarkdownBlock.Code && block.lines == listOf("psql -l")
        })

        val table = blocks.filterIsInstance<LabMarkdownBlock.Table>().single()
        assertEquals(listOf("Name", "Value"), table.rows.first())
        assertEquals(listOf("DB", "postgres"), table.rows.last())
    }
}
