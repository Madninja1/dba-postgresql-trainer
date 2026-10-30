package dev.dbatrainer.app.game

import dev.dbatrainer.app.game.domain.DailyTaskKind
import dev.dbatrainer.app.game.domain.WorkEventType
import dev.dbatrainer.app.game.domain.newDbaWorldState
import dev.dbatrainer.app.game.engine.WorkdayEngine
import dev.dbatrainer.app.game.engine.WorldEventEngine
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class WorldEventEngineTest {
    @Test
    fun incidentPropagatesToPostgresqlApplicationAndWorkTasks() {
        val world = newDbaWorldState()
        val events = WorldEventEngine.eventsFor(world)

        assertEquals(4, events.size)
        assertEquals(2, events.count { it.type == WorkEventType.TASK })
        assertEquals(1, events.count { it.type == WorkEventType.WARNING })
        assertEquals(1, events.count { it.type == WorkEventType.INCIDENT })
        assertTrue(events.any { it.targetNodeId == "db-primary" })
        assertTrue(events.any { it.targetNodeId == "app-01" })
    }

    @Test
    fun dailyTasksDisappearAfterTheirAreasAreInspected() {
        var world = newDbaWorldState()
        world = WorkdayEngine.completeTask(world, DailyTaskKind.REVIEW_MONITORING)
        world = WorkdayEngine.completeTask(world, DailyTaskKind.INSPECT_SERVER_ROOM)

        assertEquals(0, WorldEventEngine.eventsFor(world).count { it.type == WorkEventType.TASK })
    }
}
