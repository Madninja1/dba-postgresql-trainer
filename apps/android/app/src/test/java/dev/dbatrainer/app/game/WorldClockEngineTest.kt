package dev.dbatrainer.app.game

import dev.dbatrainer.app.game.domain.WorldClock
import dev.dbatrainer.app.game.domain.newDbaWorldState
import dev.dbatrainer.app.game.engine.WorldClockEngine
import org.junit.Assert.assertEquals
import org.junit.Test

class WorldClockEngineTest {
    @Test
    fun fiveRealMinutesAdvanceOneGameHour() {
        val initial = newDbaWorldState()
        val advanced = WorldClockEngine.advanceByActiveTime(initial, 5 * 60 * 1_000L)

        assertEquals(initial.clock.totalMinutes() + 60, advanced.clock.totalMinutes())
        assertEquals(0L, advanced.activeTimeRemainderMillis)
    }

    @Test
    fun partialActiveTimeIsCarriedBetweenTicks() {
        val initial = newDbaWorldState()
        val first = WorldClockEngine.advanceByActiveTime(initial, 2_500L)
        val second = WorldClockEngine.advanceByActiveTime(first, 2_500L)

        assertEquals(initial.clock.totalMinutes(), first.clock.totalMinutes())
        assertEquals(2_500L, first.activeTimeRemainderMillis)
        assertEquals(initial.clock.totalMinutes() + 1, second.clock.totalMinutes())
        assertEquals(0L, second.activeTimeRemainderMillis)
    }

    @Test
    fun calendarYearAdvancesAfterFullYear() {
        val clock = WorldClock(day = 366, minuteOfDay = 0)

        assertEquals(2027, clock.date.year)
        assertEquals(1, clock.date.month)
        assertEquals(1, clock.date.dayOfMonth)
    }
}
