package dev.dbatrainer.app.game.engine

import dev.dbatrainer.app.game.domain.IncidentKind
import dev.dbatrainer.app.game.domain.IncidentKnowledgeAccess
import dev.dbatrainer.app.game.domain.KnowledgeRequirement
import dev.dbatrainer.app.game.domain.KnowledgeRequirementAccess
import dev.dbatrainer.app.game.domain.KnowledgeSnapshot
import dev.dbatrainer.app.game.domain.LearningProductType
import dev.dbatrainer.app.game.domain.WorldState

object KnowledgeEngine {
    private val requirements = mapOf(
        IncidentKind.DISK_FULL to listOf(
            KnowledgeRequirement(
                courseCode = "linux-dba-1",
                topicSlug = "linuxdba1-filesystems-mounts",
                fallbackTitleRu = "Файловые системы и монтирование",
                fallbackTitleEn = "Filesystems and mounts",
            ),
        ),
        IncidentKind.SERVICE_DOWN to listOf(
            KnowledgeRequirement(
                courseCode = "linux-dba-1",
                topicSlug = "linuxdba1-systemd-journal",
                fallbackTitleRu = "systemd и journalctl",
                fallbackTitleEn = "systemd and journalctl",
            ),
            KnowledgeRequirement(
                courseCode = "dba-1",
                topicSlug = "dba1-tools-configuration",
                fallbackTitleRu = "Конфигурация сервера",
                fallbackTitleEn = "Server configuration",
            ),
        ),
        IncidentKind.HIGH_CPU_BAD_QUERY to listOf(
            KnowledgeRequirement(
                courseCode = "linux-dba-1",
                topicSlug = "linuxdba1-cpu-ram-swap",
                fallbackTitleRu = "CPU, RAM и swap",
                fallbackTitleEn = "CPU, RAM and swap",
            ),
            KnowledgeRequirement(
                courseCode = "dba-1",
                topicSlug = "dba1-admin-monitoring",
                fallbackTitleRu = "Мониторинг PostgreSQL",
                fallbackTitleEn = "PostgreSQL monitoring",
            ),
        ),
    )

    fun allRequirements(): List<KnowledgeRequirement> = requirements
        .values
        .flatten()
        .distinctBy { KnowledgeSnapshot.topicKey(it.courseCode, it.topicSlug) }

    fun accessFor(
        kind: IncidentKind,
        snapshot: KnowledgeSnapshot,
        world: WorldState? = null,
    ): IncidentKnowledgeAccess {
        val incidentRequirements = requirements[kind].orEmpty()
        return IncidentKnowledgeAccess(
            incidentKind = kind,
            requirements = incidentRequirements.map { requirement ->
                val progress = snapshot.progressFor(requirement)
                val purchased = progress != null && world != null &&
                    LearningEconomyEngine.hasPurchasedAccess(world, progress)
                KnowledgeRequirementAccess(
                    requirement = requirement,
                    progress = progress,
                    purchased = purchased,
                    notesAccessible = progress != null && world != null &&
                        LearningEconomyEngine.canAccessProduct(
                            world,
                            progress,
                            LearningProductType.NOTES,
                        ),
                    testAccessible = progress != null && world != null &&
                        LearningEconomyEngine.canAccessProduct(
                            world,
                            progress,
                            LearningProductType.TEST,
                        ),
                )
            },
        )
    }

    fun guidedTip(kind: IncidentKind, russian: Boolean): String = when (kind) {
        IncidentKind.DISK_FULL -> if (russian) {
            "Начните с симптома: проверьте состояние PostgreSQL, затем выясните, какая файловая система заполнена и что заняло место."
        } else {
            "Start from the symptom: inspect PostgreSQL status, then identify the full filesystem and what consumes its space."
        }

        IncidentKind.SERVICE_DOWN -> if (russian) {
            "Сначала подтвердите причину отказа через systemctl и journalctl. После этого проверяйте конфигурацию, не перезапуская сервис вслепую."
        } else {
            "Confirm the failure cause with systemctl and journalctl first. Inspect configuration before attempting blind restarts."
        }

        IncidentKind.HIGH_CPU_BAD_QUERY -> if (russian) {
            "Сначала найдите процесс, который потребляет CPU, затем сопоставьте его с backend PostgreSQL через pg_stat_activity."
        } else {
            "Identify the CPU-consuming process first, then map it to a PostgreSQL backend through pg_stat_activity."
        }
    }
}
