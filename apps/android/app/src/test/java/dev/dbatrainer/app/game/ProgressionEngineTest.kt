package dev.dbatrainer.app.game

import dev.dbatrainer.app.game.domain.IncidentKind
import dev.dbatrainer.app.game.domain.IncidentStatus
import dev.dbatrainer.app.game.domain.WorldRank
import dev.dbatrainer.app.game.domain.newDbaWorldState
import dev.dbatrainer.app.game.engine.IncidentEngine
import dev.dbatrainer.app.game.engine.ProgressionEngine
import dev.dbatrainer.app.game.engine.TerminalEngine
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Test

class ProgressionEngineTest {
    @Test
    fun resolvingDiskFullRewardsCompanyAndPlayer() {
        val before = newDbaWorldState()
        val resolvedIncident = requireNotNull(before.incident).copy(
            status = IncidentStatus.RESOLVED,
            progress = requireNotNull(before.incident).progress.copy(diskSpaceFreed = true),
        )
        val resolved = before.copy(incident = resolvedIncident)

        val rewarded = ProgressionEngine.applyIncidentResolution(before, resolved)

        assertEquals(26_500L, rewarded.company.balance)
        assertEquals(150, rewarded.player.credits)
        assertEquals(120, rewarded.player.xp)
        assertEquals(WorldRank.JUNIOR_OPERATOR, rewarded.player.worldRank)
    }

    @Test
    fun resolvedIncidentIsNotRewardedTwice() {
        val initial = newDbaWorldState()
        val resolved = initial.copy(
            incident = requireNotNull(initial.incident).copy(
                status = IncidentStatus.RESOLVED,
                progress = requireNotNull(initial.incident).progress.copy(diskSpaceFreed = true),
            ),
        )
        val rewarded = ProgressionEngine.applyIncidentResolution(initial, resolved)

        val secondPass = ProgressionEngine.applyIncidentResolution(rewarded, rewarded)

        assertEquals(rewarded, secondPass)
    }

    @Test
    fun incidentEngineMovesThroughThreeIncidents() {
        var world = newDbaWorldState()
        assertEquals(IncidentKind.DISK_FULL, world.incident?.kind)

        world = resolveCurrent(world)
        assertEquals(IncidentKind.SERVICE_DOWN, world.incident?.kind)
        assertEquals(setOf(IncidentKind.DISK_FULL), world.completedIncidents)

        world = resolveCurrent(world)
        assertEquals(IncidentKind.HIGH_CPU_BAD_QUERY, world.incident?.kind)

        world = resolveCurrent(world)
        assertNull(world.incident)
        assertEquals(3, world.completedIncidents.size)
        assertNotNull(world.lastIncidentEvaluation)
    }

    @Test
    fun hintDoesNotAdvanceClockAndStillCountsAgainstScore() {
        val initial = newDbaWorldState()
        val hinted = IncidentEngine.useHint(initial).world

        assertEquals(initial.clock, hinted.clock)
        assertEquals(1, hinted.incident?.hintsUsed)
    }

    private fun resolveCurrent(initial: dev.dbatrainer.app.game.domain.WorldState): dev.dbatrainer.app.game.domain.WorldState {
        val current = requireNotNull(initial.incident)
        val resolved = initial.copy(
            incident = current.copy(status = IncidentStatus.RESOLVED),
        )
        val rewarded = ProgressionEngine.applyIncidentResolution(initial, resolved)
        return IncidentEngine.afterExecution(initial, rewarded)
    }
}
