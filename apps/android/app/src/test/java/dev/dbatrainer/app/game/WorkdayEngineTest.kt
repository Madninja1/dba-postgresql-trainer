package dev.dbatrainer.app.game

import dev.dbatrainer.app.game.domain.DailyTaskKind
import dev.dbatrainer.app.game.domain.WorldClock
import dev.dbatrainer.app.game.domain.newDbaWorldState
import dev.dbatrainer.app.game.engine.WorkdayEngine
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test

class WorkdayEngineTest {
    @Test
    fun shiftCanBeClosedAfterEighteenHundred() {
        var world = newDbaWorldState().copy(clock = WorldClock(day = 1, minuteOfDay = 18 * 60))
        world = WorkdayEngine.completeTask(world, DailyTaskKind.REVIEW_MONITORING)
        world = WorkdayEngine.completeTask(world, DailyTaskKind.INSPECT_SERVER_ROOM)
        world = WorkdayEngine.recordIncidentResolution(world)

        assertTrue(WorkdayEngine.canCloseShift(world))
        val closed = WorkdayEngine.closeShift(world)

        assertTrue(closed.workday.shiftClosed)
        assertEquals(2, closed.lastShiftReport?.tasksCompleted)
        assertEquals(1, closed.lastShiftReport?.incidentsResolved)
    }

    @Test
    fun monthChangePaysSalaryAndCreatesCompanyReport() {
        val initial = newDbaWorldState().copy(
            clock = WorldClock(day = 31, minuteOfDay = 23 * 60 + 59),
        )
        val advancedClock = initial.copy(clock = WorldClock(day = 32, minuteOfDay = 0))
        val updated = WorkdayEngine.applyTimeTransitions(initial, advancedClock)

        assertEquals(1_200, updated.player.credits)
        assertNotNull(updated.lastMonthlyReport)
        assertEquals(2, updated.lastMonthlyReport?.month)
        assertTrue(updated.company.balance > initial.company.balance)
        assertFalse(updated.workday.shiftClosed)
        assertEquals(32, updated.workday.day)
    }
}
