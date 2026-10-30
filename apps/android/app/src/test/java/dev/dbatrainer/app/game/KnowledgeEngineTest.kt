package dev.dbatrainer.app.game

import dev.dbatrainer.app.game.domain.IncidentKind
import dev.dbatrainer.app.game.domain.KnowledgeGateState
import dev.dbatrainer.app.game.domain.KnowledgeMode
import dev.dbatrainer.app.game.domain.KnowledgeSnapshot
import dev.dbatrainer.app.game.domain.KnowledgeTopicProgress
import dev.dbatrainer.app.game.domain.LearningProductType
import dev.dbatrainer.app.game.domain.newDbaWorldState
import dev.dbatrainer.app.game.engine.KnowledgeEngine
import dev.dbatrainer.app.game.engine.LearningEconomyEngine
import org.junit.Assert.assertEquals
import org.junit.Test

class KnowledgeEngineTest {
    @Test
    fun untouchedRequirementsUseGuidedMode() {
        val access = KnowledgeEngine.accessFor(
            IncidentKind.DISK_FULL,
            KnowledgeSnapshot.EMPTY,
        )

        assertEquals(KnowledgeGateState.LOCKED, access.gateState)
        assertEquals(KnowledgeMode.GUIDED, access.mode)
    }

    @Test
    fun partiallyStartedRequirementsStayGuided() {
        val requirement = KnowledgeEngine
            .allRequirements()
            .first { it.topicSlug == "linuxdba1-systemd-journal" }
        val progress = KnowledgeTopicProgress(
            courseCode = requirement.courseCode,
            topicSlug = requirement.topicSlug,
            topicNumber = 1L,
            title = "systemd и journalctl",
            started = true,
            passed = true,
            completedAttempts = 1,
            bestAccuracyPercent = 90.0,
        )
        val snapshot = KnowledgeSnapshot(
            mapOf(
                KnowledgeSnapshot.topicKey(requirement.courseCode, requirement.topicSlug) to progress,
            ),
        )

        val access = KnowledgeEngine.accessFor(IncidentKind.SERVICE_DOWN, snapshot)

        assertEquals(KnowledgeGateState.DISCOVERED, access.gateState)
        assertEquals(KnowledgeMode.GUIDED, access.mode)
    }


    @Test
    fun purchasedRequirementBecomesDiscoveredButStaysGuided() {
        val requirement = KnowledgeEngine
            .allRequirements()
            .first { it.topicSlug == "dba1-admin-monitoring" }
        val progress = KnowledgeTopicProgress(
            courseCode = requirement.courseCode,
            topicSlug = requirement.topicSlug,
            topicNumber = 12L,
            title = requirement.fallbackTitleRu,
            started = false,
            passed = false,
            completedAttempts = 0,
            bestAccuracyPercent = 0.0,
        )
        val snapshot = KnowledgeSnapshot(
            mapOf(KnowledgeSnapshot.topicKey(progress.courseCode, progress.topicSlug) to progress),
        )
        val funded = newDbaWorldState().copy(
            player = newDbaWorldState().player.copy(credits = 2_000),
        )
        val purchased = LearningEconomyEngine.purchaseProduct(
            funded,
            progress,
            LearningProductType.NOTES,
        ).world

        val access = KnowledgeEngine.accessFor(
            IncidentKind.HIGH_CPU_BAD_QUERY,
            snapshot,
            purchased,
        )
        val requirementAccess = access.requirements.first {
            it.requirement.topicSlug == requirement.topicSlug
        }

        assertEquals(true, requirementAccess.purchased)
        assertEquals(true, requirementAccess.notesAccessible)
        assertEquals(false, requirementAccess.testAccessible)
        assertEquals(KnowledgeGateState.DISCOVERED, access.gateState)
        assertEquals(KnowledgeMode.GUIDED, access.mode)
    }

    @Test
    fun allRequiredTopicsPassedEnableNormalMode() {
        val serviceRequirements = KnowledgeEngine
            .accessFor(IncidentKind.SERVICE_DOWN, KnowledgeSnapshot.EMPTY)
            .requirements
            .map { it.requirement }
        val snapshot = KnowledgeSnapshot(
            serviceRequirements.associate { requirement ->
                KnowledgeSnapshot.topicKey(requirement.courseCode, requirement.topicSlug) to
                    KnowledgeTopicProgress(
                        courseCode = requirement.courseCode,
                        topicSlug = requirement.topicSlug,
                        topicNumber = 1L,
                        title = requirement.fallbackTitleRu,
                        started = true,
                        passed = true,
                        completedAttempts = 2,
                        bestAccuracyPercent = 92.0,
                    )
            },
        )

        val access = KnowledgeEngine.accessFor(IncidentKind.SERVICE_DOWN, snapshot)

        assertEquals(KnowledgeGateState.MASTERED, access.gateState)
        assertEquals(KnowledgeMode.NORMAL, access.mode)
    }
}
