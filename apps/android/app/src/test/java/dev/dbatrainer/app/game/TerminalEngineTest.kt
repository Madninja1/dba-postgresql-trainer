package dev.dbatrainer.app.game

import dev.dbatrainer.app.game.domain.IncidentKind
import dev.dbatrainer.app.game.domain.IncidentStatus
import dev.dbatrainer.app.game.domain.newDbaWorldState
import dev.dbatrainer.app.game.engine.IncidentEngine
import dev.dbatrainer.app.game.engine.TerminalEngine
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TerminalEngineTest {
    @Test
    fun diskFullCanBeDiagnosedAndResolved() {
        var world = newDbaWorldState()
        world = TerminalEngine.execute(world, "systemctl status postgresql").world
        world = TerminalEngine.execute(world, "journalctl -xeu postgresql").world
        world = TerminalEngine.execute(world, "df -h").world
        world = TerminalEngine.execute(world, "du -sh /var/log/postgresql/*").world
        world = TerminalEngine.execute(
            world,
            "sudo rm /var/log/postgresql/postgresql-old.log",
        ).world
        world = TerminalEngine.execute(world, "sudo systemctl start postgresql").world

        assertEquals(IncidentStatus.RESOLVED, world.incident?.status)
        assertEquals(0, world.incident?.progress?.unsafeAttempts)
    }

    @Test
    fun serviceDownConfigIsStatefulAfterFix() {
        val base = newDbaWorldState().copy(
            completedIncidents = setOf(IncidentKind.DISK_FULL),
            incident = null,
        )
        var world = IncidentEngine.ensureActiveIncident(base)

        val before = TerminalEngine.execute(
            world,
            "grep shared_buffers /etc/postgresql/16/main/postgresql.conf",
        )
        assertTrue(before.output.contains("4ПИ"))

        world = TerminalEngine.execute(
            before.world,
            "sudo sed -i \"s/4ПИ/4GB/\" /etc/postgresql/16/main/postgresql.conf",
        ).world

        val after = TerminalEngine.execute(
            world,
            "grep shared_buffers /etc/postgresql/16/main/postgresql.conf",
        )
        assertTrue(after.output.contains("4GB"))
        assertEquals("4GB", after.world.postgresqlConfig.sharedBuffers)
    }

    @Test
    fun highCpuRestartIsAcceptedButMarkedUnsafe() {
        val base = newDbaWorldState().copy(
            completedIncidents = setOf(IncidentKind.DISK_FULL, IncidentKind.SERVICE_DOWN),
            incident = null,
        )
        val highCpu = IncidentEngine.ensureActiveIncident(base)
        val restarted = TerminalEngine.execute(
            highCpu,
            "sudo systemctl restart postgresql",
        ).world

        assertEquals(IncidentStatus.RESOLVED, restarted.incident?.status)
        assertTrue((restarted.incident?.progress?.unsafeAttempts ?: 0) > 0)
    }
}
