package dev.dbatrainer.app.game

import dev.dbatrainer.app.game.domain.WorldClock
import dev.dbatrainer.app.game.engine.WorldDayPhase
import dev.dbatrainer.app.game.engine.WorldEnvironmentEngine
import dev.dbatrainer.app.game.engine.WorldSeason
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class WorldEnvironmentEngineTest {
    @Test
    fun `winter morning is derived from world clock`() {
        val environment = WorldEnvironmentEngine.environmentFor(
            WorldClock(day = 1, minuteOfDay = 8 * 60),
        )

        assertEquals(WorldSeason.WINTER, environment.season)
        assertEquals(WorldDayPhase.MORNING, environment.dayPhase)
    }

    @Test
    fun `new year decorations are calendar driven`() {
        val januaryFirst = WorldEnvironmentEngine.environmentFor(WorldClock(day = 1))
        assertTrue(januaryFirst.newYearDecorations)

        val februaryFirst = WorldEnvironmentEngine.environmentFor(WorldClock(day = 32))
        assertFalse(februaryFirst.newYearDecorations)
    }

    @Test
    fun `weather is stable for the same game day`() {
        val morning = WorldEnvironmentEngine.environmentFor(WorldClock(day = 120, minuteOfDay = 8 * 60))
        val evening = WorldEnvironmentEngine.environmentFor(WorldClock(day = 120, minuteOfDay = 20 * 60))

        assertEquals(morning.season, evening.season)
        assertEquals(morning.weather, evening.weather)
    }
}
