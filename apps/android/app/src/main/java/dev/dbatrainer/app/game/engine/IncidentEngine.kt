package dev.dbatrainer.app.game.engine

import dev.dbatrainer.app.game.domain.IncidentEvaluation
import dev.dbatrainer.app.game.domain.IncidentGrade
import dev.dbatrainer.app.game.domain.IncidentKind
import dev.dbatrainer.app.game.domain.IncidentSeverity
import dev.dbatrainer.app.game.domain.IncidentState
import dev.dbatrainer.app.game.domain.IncidentStatus
import dev.dbatrainer.app.game.domain.InfrastructureNodeKind
import dev.dbatrainer.app.game.domain.InfrastructureStatus
import dev.dbatrainer.app.game.domain.WorldState

object IncidentEngine {
    private val sequence = listOf(
        IncidentKind.DISK_FULL,
        IncidentKind.SERVICE_DOWN,
        IncidentKind.HIGH_CPU_BAD_QUERY,
    )

    fun ensureActiveIncident(world: WorldState): WorldState {
        val current = world.incident
        if (current != null && current.status == IncidentStatus.ACTIVE) {
            return world
        }

        val completed = buildSet {
            addAll(world.completedIncidents)
            if (current?.status == IncidentStatus.RESOLVED) {
                add(current.kind)
            }
        }
        val next = sequence.firstOrNull { it !in completed } ?: return world.copy(
            incident = null,
            completedIncidents = completed,
        )

        val prepared = prepareWorldFor(next, world.copy(completedIncidents = completed))
        return prepared.copy(incident = createIncident(next, prepared))
    }

    fun afterExecution(previous: WorldState, updated: WorldState): WorldState {
        val before = previous.incident ?: return updated
        val after = updated.incident ?: return updated
        if (
            before.id != after.id ||
            before.status != IncidentStatus.ACTIVE ||
            after.status != IncidentStatus.RESOLVED
        ) {
            return updated
        }

        val evaluation = evaluate(after, updated)
        val completed = updated.completedIncidents + after.kind
        val resolvedWorld = updated.copy(
            completedIncidents = completed,
            lastIncidentEvaluation = evaluation,
        )
        return ensureActiveIncident(resolvedWorld)
    }

    fun useHint(world: WorldState, russian: Boolean = false): HintResult {
        val incident = world.incident
            ?: return HintResult(world, "No active incident.")
        if (incident.status != IncidentStatus.ACTIVE) {
            return HintResult(world, "Incident is already resolved.")
        }

        val hints = hintsFor(incident.kind, russian)
        val hintIndex = incident.hintsUsed.coerceAtMost(hints.lastIndex)
        val hint = hints[hintIndex]
        val updated = world.copy(
            incident = incident.copy(hintsUsed = incident.hintsUsed + 1),
        )
        return HintResult(updated, hint)
    }

    fun title(kind: IncidentKind): String = when (kind) {
        IncidentKind.DISK_FULL -> "Disk Full"
        IncidentKind.SERVICE_DOWN -> "PostgreSQL Service Down"
        IncidentKind.HIGH_CPU_BAD_QUERY -> "High CPU / Bad Query"
    }

    fun summary(kind: IncidentKind): String = when (kind) {
        IncidentKind.DISK_FULL -> "PostgreSQL is down and /var is full. Diagnose the filesystem and restore the service."
        IncidentKind.SERVICE_DOWN -> "PostgreSQL failed after a configuration change. Find the invalid setting and start the service safely."
        IncidentKind.HIGH_CPU_BAD_QUERY -> "Database CPU is saturated. Find the expensive session and stop the runaway query without restarting PostgreSQL."
    }

    fun summaryRu(kind: IncidentKind): String = when (kind) {
        IncidentKind.DISK_FULL -> "PostgreSQL недоступен, а /var заполнен. Найдите причину и безопасно восстановите сервис."
        IncidentKind.SERVICE_DOWN -> "PostgreSQL не запустился после изменения конфигурации. Найдите неверный параметр и восстановите сервис."
        IncidentKind.HIGH_CPU_BAD_QUERY -> "CPU базы перегружен. Найдите дорогую сессию и остановите проблемный запрос без перезапуска PostgreSQL."
    }

    private fun createIncident(kind: IncidentKind, world: WorldState): IncidentState = IncidentState(
        id = when (kind) {
            IncidentKind.DISK_FULL -> "incident-disk-full-01"
            IncidentKind.SERVICE_DOWN -> "incident-service-down-01"
            IncidentKind.HIGH_CPU_BAD_QUERY -> "incident-high-cpu-01"
        },
        kind = kind,
        title = title(kind),
        severity = when (kind) {
            IncidentKind.DISK_FULL -> IncidentSeverity.CRITICAL
            IncidentKind.SERVICE_DOWN -> IncidentSeverity.HIGH
            IncidentKind.HIGH_CPU_BAD_QUERY -> IncidentSeverity.HIGH
        },
        status = IncidentStatus.ACTIVE,
        rootCause = when (kind) {
            IncidentKind.DISK_FULL -> "Rotated PostgreSQL log filled /var."
            IncidentKind.SERVICE_DOWN -> "shared_buffers was typed as 4ПИ with the Russian keyboard layout instead of 4GB."
            IncidentKind.HIGH_CPU_BAD_QUERY -> "Runaway analytical query is consuming database CPU."
        },
        startedAtTotalMinutes = world.clock.totalMinutes(),
    )

    private fun prepareWorldFor(kind: IncidentKind, world: WorldState): WorldState {
        val nodes = world.nodes.map { node ->
            when (kind) {
                IncidentKind.DISK_FULL -> when (node.kind) {
                    InfrastructureNodeKind.APPLICATION -> node.copy(status = InfrastructureStatus.WARNING)
                    InfrastructureNodeKind.POSTGRESQL -> node.copy(
                        status = InfrastructureStatus.DOWN,
                        metrics = node.metrics.copy(cpuPercent = 3, ramPercent = 34, diskPercent = 100),
                    )
                }

                IncidentKind.SERVICE_DOWN -> when (node.kind) {
                    InfrastructureNodeKind.APPLICATION -> node.copy(status = InfrastructureStatus.WARNING)
                    InfrastructureNodeKind.POSTGRESQL -> node.copy(
                        status = InfrastructureStatus.DOWN,
                        metrics = node.metrics.copy(cpuPercent = 2, ramPercent = 31, diskPercent = 68),
                    )
                }

                IncidentKind.HIGH_CPU_BAD_QUERY -> when (node.kind) {
                    InfrastructureNodeKind.APPLICATION -> node.copy(status = InfrastructureStatus.WARNING)
                    InfrastructureNodeKind.POSTGRESQL -> node.copy(
                        status = InfrastructureStatus.WARNING,
                        metrics = node.metrics.copy(cpuPercent = 97, ramPercent = 63, diskPercent = 69),
                    )
                }
            }
        }
        return world.copy(
            nodes = nodes,
            postgresqlConfig = if (kind == IncidentKind.SERVICE_DOWN) {
                world.postgresqlConfig.copy(sharedBuffers = "4ПИ")
            } else {
                world.postgresqlConfig
            },
        )
    }

    private fun evaluate(incident: IncidentState, world: WorldState): IncidentEvaluation {
        val elapsed = (world.clock.totalMinutes() - incident.startedAtTotalMinutes).coerceAtLeast(0)
        val progress = incident.progress
        val diagnosisChecks = when (incident.kind) {
            IncidentKind.DISK_FULL -> listOf(
                progress.serviceInspected,
                progress.journalInspected,
                progress.diskInspected,
                progress.logsInspected,
            )

            IncidentKind.SERVICE_DOWN -> listOf(
                progress.serviceInspected,
                progress.journalInspected,
                progress.configInspected,
            )

            IncidentKind.HIGH_CPU_BAD_QUERY -> listOf(
                progress.processesInspected,
                progress.activityInspected,
            )
        }
        val checked = diagnosisChecks.count { it }
        val diagnosis = when {
            checked == diagnosisChecks.size -> IncidentGrade.A
            checked >= diagnosisChecks.size - 1 -> IncidentGrade.B
            checked >= 1 -> IncidentGrade.C
            else -> IncidentGrade.D
        }
        val safety = when (progress.unsafeAttempts) {
            0 -> IncidentGrade.A
            1 -> IncidentGrade.B
            2 -> IncidentGrade.C
            else -> IncidentGrade.D
        }
        val time = when {
            elapsed <= 25 -> IncidentGrade.A
            elapsed <= 40 -> IncidentGrade.B
            elapsed <= 60 -> IncidentGrade.C
            else -> IncidentGrade.D
        }
        val score = (
            100 -
                incident.hintsUsed * 10 -
                progress.unsafeAttempts * 12 -
                (incident.actionCount - diagnosisChecks.size - 1).coerceAtLeast(0) * 2 -
                (elapsed - 25).coerceAtLeast(0) / 3
            ).coerceIn(0, 100)

        return IncidentEvaluation(
            incidentKind = incident.kind,
            title = incident.title,
            diagnosis = diagnosis,
            safety = safety,
            time = time,
            hintsUsed = incident.hintsUsed,
            elapsedMinutes = elapsed,
            score = score,
        )
    }

    private fun hintsFor(kind: IncidentKind, russian: Boolean): List<String> = when (kind) {
        IncidentKind.DISK_FULL -> if (russian) {
            listOf(
                "Начните с состояния сервиса, затем посмотрите журнал PostgreSQL.",
                "Журнал указывает на хранилище. Проверьте заполнение файловой системы через df -h.",
                "Перед удалением сравните размеры файлов в /var/log/postgresql.",
            )
        } else {
            listOf(
                "Start with the service state, then inspect the PostgreSQL journal.",
                "The journal points to storage. Check filesystem usage with df -h.",
                "Compare files under /var/log/postgresql before deleting anything.",
            )
        }

        IncidentKind.SERVICE_DOWN -> if (russian) {
            listOf(
                "Сначала проверьте systemctl status и журнал, не повторяя restart вслепую.",
                "Сбой появился после изменения конфигурации. Проверьте shared_buffers.",
                "Исправьте неверную единицу в postgresql.conf и только затем запускайте PostgreSQL.",
            )
        } else {
            listOf(
                "Inspect systemctl status and the journal before restarting repeatedly.",
                "The failure happened after a configuration change. Inspect shared_buffers.",
                "Fix the invalid unit in postgresql.conf, then start PostgreSQL.",
            )
        }

        IncidentKind.HIGH_CPU_BAD_QUERY -> if (russian) {
            listOf(
                "Высокий CPU не означает, что PostgreSQL нужно сразу перезапускать. Сначала найдите процесс.",
                "Посмотрите pg_stat_activity и найдите долго выполняющийся аналитический запрос.",
                "Отмените проблемный backend вместо перезапуска всей базы.",
            )
        } else {
            listOf(
                "High CPU does not automatically mean PostgreSQL should be restarted. Find the process first.",
                "Inspect pg_stat_activity and identify the long-running analytical query.",
                "Cancel the runaway backend instead of restarting the whole database.",
            )
        }
    }
}

data class HintResult(
    val world: WorldState,
    val hint: String,
)
