package dev.dbatrainer.app.game.engine

import dev.dbatrainer.app.game.domain.DailyTaskKind
import dev.dbatrainer.app.game.domain.IncidentSeverity
import dev.dbatrainer.app.game.domain.InfrastructureNodeKind
import dev.dbatrainer.app.game.domain.InfrastructureStatus
import dev.dbatrainer.app.game.domain.WorkEventSource
import dev.dbatrainer.app.game.domain.WorkEventType
import dev.dbatrainer.app.game.domain.WorldEvent
import dev.dbatrainer.app.game.domain.WorldState

object WorldEventEngine {
    fun eventsFor(world: WorldState): List<WorldEvent> = buildList {
        if (DailyTaskKind.REVIEW_MONITORING !in world.workday.completedTasks) {
            add(
                WorldEvent(
                    id = "task-review-monitoring-${world.clock.day}",
                    type = WorkEventType.TASK,
                    source = WorkEventSource.MONITORING,
                    titleRu = "Проверить мониторинг",
                    titleEn = "Review monitoring",
                    descriptionRu = "Откройте Monitoring и проверьте состояние инфраструктуры.",
                    descriptionEn = "Open Monitoring and review infrastructure health.",
                ),
            )
        }
        if (DailyTaskKind.INSPECT_SERVER_ROOM !in world.workday.completedTasks) {
            add(
                WorldEvent(
                    id = "task-server-room-${world.clock.day}",
                    type = WorkEventType.TASK,
                    source = WorkEventSource.WORKSTATION,
                    titleRu = "Осмотреть серверную",
                    titleEn = "Inspect server room",
                    descriptionRu = "Зайдите в серверную и проверьте физические узлы.",
                    descriptionEn = "Enter the server room and inspect physical nodes.",
                ),
            )
        }

        val incident = world.incident
        if (incident != null) {
            add(
                WorldEvent(
                    id = "incident-${incident.id}",
                    type = WorkEventType.INCIDENT,
                    source = WorkEventSource.POSTGRESQL,
                    targetNodeId = "db-primary",
                    titleRu = IncidentEngine.summaryRu(incident.kind),
                    titleEn = IncidentEngine.summary(incident.kind),
                    descriptionRu = "Источник: PostgreSQL / db-primary. Откройте узел и начните диагностику.",
                    descriptionEn = "Source: PostgreSQL / db-primary. Open the node and begin diagnosis.",
                    critical = incident.severity == IncidentSeverity.CRITICAL,
                ),
            )
        }

        world.nodes.firstOrNull { it.kind == InfrastructureNodeKind.APPLICATION }
            ?.takeIf { it.status != InfrastructureStatus.HEALTHY }
            ?.let { app ->
                add(
                    WorldEvent(
                        id = "warning-${app.id}-${world.clock.day}",
                        type = WorkEventType.WARNING,
                        source = WorkEventSource.APPLICATION,
                        targetNodeId = app.id,
                        titleRu = "Приложение теряет соединение с базой",
                        titleEn = "Application database connectivity degraded",
                        descriptionRu = "Это вторичный симптом: состояние app-01 ухудшилось из-за проблемы на db-primary.",
                        descriptionEn = "This is a propagated symptom: app-01 is degraded because db-primary is unhealthy.",
                    ),
                )
            }
    }

    fun unresolvedAlertCount(world: WorldState): Int = eventsFor(world).count {
        it.type != WorkEventType.TASK
    }
}
