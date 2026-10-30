package dev.dbatrainer.app.game.engine

import dev.dbatrainer.app.game.domain.IncidentKind
import dev.dbatrainer.app.game.domain.IncidentProgress
import dev.dbatrainer.app.game.domain.IncidentStatus
import dev.dbatrainer.app.game.domain.InfrastructureNodeKind
import dev.dbatrainer.app.game.domain.InfrastructureStatus
import dev.dbatrainer.app.game.domain.WorldState

data class TerminalExecution(
    val world: WorldState,
    val output: String,
)

object TerminalEngine {
    private const val HELP = "help"
    private const val SYSTEMCTL_STATUS = "systemctl status postgresql"
    private const val JOURNALCTL = "journalctl -xeu postgresql"
    private const val DF = "df -h"
    private const val DU = "du -sh /var/log/postgresql/*"
    private const val REMOVE_OLD_LOG = "sudo rm /var/log/postgresql/postgresql-old.log"
    private const val START_POSTGRES = "sudo systemctl start postgresql"
    private const val SHOW_CONFIG = "grep shared_buffers /etc/postgresql/16/main/postgresql.conf"
    private const val FIX_CONFIG = "sudo sed -i \"s/4ПИ/4GB/\" /etc/postgresql/16/main/postgresql.conf"
    private const val TOP = "top -b -n1 | head"
    private const val PG_ACTIVITY = "psql -c \"SELECT pid, state, query FROM pg_stat_activity ORDER BY query_start;\""
    private const val CANCEL_QUERY = "psql -c \"SELECT pg_cancel_backend(4242);\""
    private const val RESTART_POSTGRES = "sudo systemctl restart postgresql"

    fun suggestedCommands(world: WorldState): List<String> = when (world.incident?.kind) {
        IncidentKind.DISK_FULL -> listOf(
            SYSTEMCTL_STATUS,
            JOURNALCTL,
            DF,
            DU,
            REMOVE_OLD_LOG,
            START_POSTGRES,
        )

        IncidentKind.SERVICE_DOWN -> listOf(
            SYSTEMCTL_STATUS,
            JOURNALCTL,
            SHOW_CONFIG,
            FIX_CONFIG,
            START_POSTGRES,
        )

        IncidentKind.HIGH_CPU_BAD_QUERY -> listOf(
            TOP,
            PG_ACTIVITY,
            CANCEL_QUERY,
            RESTART_POSTGRES,
        )

        null -> emptyList()
    }

    fun execute(world: WorldState, rawCommand: String): TerminalExecution {
        val command = normalize(rawCommand)
        if (command.isEmpty()) {
            return TerminalExecution(world, "")
        }

        if (command == HELP) {
            val commands = suggestedCommands(world)
            return TerminalExecution(
                world,
                if (commands.isEmpty()) {
                    "No active incident."
                } else {
                    "Available commands:\n${commands.joinToString("\n")}"
                },
            )
        }

        val incident = world.incident
            ?: return TerminalExecution(world, "No active incident. Type 'help' for available commands.")

        val execution = when (incident.kind) {
            IncidentKind.DISK_FULL -> executeDiskFull(world, command)
            IncidentKind.SERVICE_DOWN -> executeServiceDown(world, command)
            IncidentKind.HIGH_CPU_BAD_QUERY -> executeHighCpu(world, command)
        }

        return if (command in suggestedCommands(world)) {
            execution.copy(world = countAction(execution.world))
        } else {
            execution
        }
    }

    private fun executeDiskFull(world: WorldState, command: String): TerminalExecution = when (command) {
        SYSTEMCTL_STATUS -> TerminalExecution(
            updateProgress(world) { it.copy(serviceInspected = true) },
            "× postgresql.service - PostgreSQL\n   Active: failed\n   Result: exit-code\n   See: journalctl -xeu postgresql",
        )

        JOURNALCTL -> TerminalExecution(
            updateProgress(world) { it.copy(journalInspected = true) },
            "postgresql: PANIC: could not write file \"pg_wal/xlogtemp\": No space left on device\n" +
                "postgresql.service: Main process exited, status=2/INVALIDARGUMENT",
        )

        DF -> TerminalExecution(
            updateProgress(world) { it.copy(diskInspected = true) },
            if (world.incident?.progress?.diskSpaceFreed == true) {
                "Filesystem      Size  Used Avail Use% Mounted on\n/dev/vda1        8.0G  5.4G  2.6G  68% /var"
            } else {
                "Filesystem      Size  Used Avail Use% Mounted on\n/dev/vda1        8.0G  8.0G     0 100% /var"
            },
        )

        DU -> TerminalExecution(
            updateProgress(world) { it.copy(logsInspected = true) },
            if (world.incident?.progress?.diskSpaceFreed == true) {
                "84M\t/var/log/postgresql/postgresql.log"
            } else {
                "84M\t/var/log/postgresql/postgresql.log\n" +
                    "2.6G\t/var/log/postgresql/postgresql-old.log"
            },
        )

        REMOVE_OLD_LOG -> freeDisk(world)
        START_POSTGRES -> startPostgresAfterDiskFull(world)
        else -> unknown(world, command)
    }

    private fun executeServiceDown(world: WorldState, command: String): TerminalExecution = when (command) {
        SYSTEMCTL_STATUS -> TerminalExecution(
            updateProgress(world) { it.copy(serviceInspected = true) },
            "× postgresql.service - PostgreSQL\n   Active: failed\n   Process: ExecStart=/usr/lib/postgresql/16/bin/postgres\n   Result: exit-code",
        )

        JOURNALCTL -> TerminalExecution(
            updateProgress(world) { it.copy(journalInspected = true) },
            "postgres: invalid value for parameter \"shared_buffers\": \"4ПИ\"\n" +
                "postgresql.service: Failed with result 'exit-code'.",
        )

        SHOW_CONFIG -> TerminalExecution(
            updateProgress(world) { it.copy(configInspected = true) },
            "shared_buffers = '${world.postgresqlConfig.sharedBuffers}'",
        )

        FIX_CONFIG -> {
            val updated = updateProgress(world) { it.copy(configFixed = true) }.copy(
                postgresqlConfig = world.postgresqlConfig.copy(sharedBuffers = "4GB"),
            )
            TerminalExecution(
                updated,
                "Updated postgresql.conf: shared_buffers = '4GB'",
            )
        }

        START_POSTGRES -> startPostgresAfterConfig(world)
        else -> unknown(world, command)
    }

    private fun executeHighCpu(world: WorldState, command: String): TerminalExecution = when (command) {
        TOP -> TerminalExecution(
            updateProgress(world) { it.copy(processesInspected = true) },
            "PID   USER      %CPU  %MEM  COMMAND\n4242  postgres  92.4   8.1  postgres: app analytics SELECT\n1180  postgres   2.1   4.0  postgres: checkpointer",
        )

        PG_ACTIVITY -> TerminalExecution(
            updateProgress(world) { it.copy(activityInspected = true) },
            " pid  | state  | query\n------+--------+------------------------------------------------\n" +
                " 4242 | active | SELECT generate_series(1,1000000000), count(*) ...\n" +
                " 3110 | idle   | COMMIT",
        )

        CANCEL_QUERY -> cancelBadQuery(world)
        RESTART_POSTGRES -> restartDuringHighCpu(world)
        else -> unknown(world, command)
    }

    private fun freeDisk(world: WorldState): TerminalExecution {
        val incident = requireNotNull(world.incident)
        if (incident.progress.diskSpaceFreed) {
            return TerminalExecution(world, "rm: cannot remove 'postgresql-old.log': No such file or directory")
        }

        val unsafe = if (incident.progress.logsInspected) 0 else 1
        val updated = updateProgress(world) {
            it.copy(
                diskSpaceFreed = true,
                unsafeAttempts = it.unsafeAttempts + unsafe,
            )
        }.copy(
            nodes = world.nodes.map { node ->
                if (node.kind == InfrastructureNodeKind.POSTGRESQL) {
                    node.copy(metrics = node.metrics.copy(diskPercent = 68))
                } else {
                    node
                }
            },
        )

        return TerminalExecution(
            updated,
            "Removed /var/log/postgresql/postgresql-old.log\n2.6G freed on /var.",
        )
    }

    private fun startPostgresAfterDiskFull(world: WorldState): TerminalExecution {
        val incident = requireNotNull(world.incident)
        if (!incident.progress.diskSpaceFreed) {
            return TerminalExecution(
                updateProgress(world) {
                    it.copy(unsafeAttempts = it.unsafeAttempts + 1)
                },
                "Job for postgresql.service failed because the filesystem is full.\n" +
                    "Check 'df -h' and free disk space first.",
            )
        }
        return resolveWithHealthyNodes(world, "Started postgresql.service.\nPostgreSQL is ready to accept connections.")
    }

    private fun startPostgresAfterConfig(world: WorldState): TerminalExecution {
        val incident = requireNotNull(world.incident)
        if (!incident.progress.configFixed || world.postgresqlConfig.sharedBuffers != "4GB") {
            return TerminalExecution(
                updateProgress(world) {
                    it.copy(unsafeAttempts = it.unsafeAttempts + 1)
                },
                "Job for postgresql.service failed. Configuration still contains an invalid shared_buffers value.",
            )
        }
        return resolveWithHealthyNodes(world, "Started postgresql.service with validated configuration.")
    }

    private fun cancelBadQuery(world: WorldState): TerminalExecution {
        val updated = updateProgress(world) { it.copy(badQueryCancelled = true) }
        return resolveWithHealthyNodes(
            updated,
            " pg_cancel_backend\n-------------------\n t\n\nQuery 4242 cancelled. Database CPU returned to normal.",
        )
    }

    private fun restartDuringHighCpu(world: WorldState): TerminalExecution {
        val updated = updateProgress(world) {
            it.copy(unsafeAttempts = it.unsafeAttempts + 1)
        }
        return resolveWithHealthyNodes(
            updated,
            "PostgreSQL restarted. The runaway query stopped, but the restart caused avoidable application downtime.",
        )
    }

    private fun resolveWithHealthyNodes(
        world: WorldState,
        output: String,
    ): TerminalExecution {
        val incident = requireNotNull(world.incident)
        val nodes = world.nodes.map { node ->
            when (node.kind) {
                InfrastructureNodeKind.APPLICATION -> node.copy(status = InfrastructureStatus.HEALTHY)
                InfrastructureNodeKind.POSTGRESQL -> node.copy(
                    status = InfrastructureStatus.HEALTHY,
                    metrics = node.metrics.copy(cpuPercent = 18, ramPercent = 46),
                )
            }
        }
        return TerminalExecution(
            world.copy(
                nodes = nodes,
                incident = incident.copy(status = IncidentStatus.RESOLVED),
            ),
            "$output\nIncident resolved.",
        )
    }

    private fun updateProgress(
        world: WorldState,
        transform: (IncidentProgress) -> IncidentProgress,
    ): WorldState {
        val incident = world.incident ?: return world
        return world.copy(
            incident = incident.copy(progress = transform(incident.progress)),
        )
    }

    private fun countAction(world: WorldState): WorldState {
        val incident = world.incident ?: return world
        return world.copy(
            incident = incident.copy(actionCount = incident.actionCount + 1),
        )
    }

    private fun unknown(world: WorldState, command: String): TerminalExecution = TerminalExecution(
        world,
        "bash: $command: command not found\nType 'help' to see commands available in this simulation.",
    )

    private fun normalize(command: String): String = command
        .trim()
        .replace(Regex("\\s+"), " ")
}
