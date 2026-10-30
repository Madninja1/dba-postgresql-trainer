package dev.dbatrainer.app.game.domain

enum class KnowledgeGateState {
    LOCKED,
    DISCOVERED,
    MASTERED,
}

enum class KnowledgeMode {
    GUIDED,
    NORMAL,
}

data class KnowledgeRequirement(
    val courseCode: String,
    val topicSlug: String,
    val fallbackTitleRu: String,
    val fallbackTitleEn: String,
)

data class KnowledgeTopicProgress(
    val courseCode: String,
    val topicSlug: String,
    val topicNumber: Long,
    val title: String,
    val started: Boolean,
    val passed: Boolean,
    val completedAttempts: Long,
    val bestAccuracyPercent: Double,
    val hasLab: Boolean = false,
)

data class KnowledgeSnapshot(
    val topics: Map<String, KnowledgeTopicProgress> = emptyMap(),
) {
    fun progressFor(requirement: KnowledgeRequirement): KnowledgeTopicProgress? =
        topics[topicKey(requirement.courseCode, requirement.topicSlug)]

    companion object {
        val EMPTY = KnowledgeSnapshot()

        fun topicKey(courseCode: String, topicSlug: String): String = "$courseCode::$topicSlug"
    }
}

data class KnowledgeRequirementAccess(
    val requirement: KnowledgeRequirement,
    val progress: KnowledgeTopicProgress?,
    val purchased: Boolean = false,
    val notesAccessible: Boolean = false,
    val testAccessible: Boolean = false,
) {
    val passed: Boolean
        get() = progress?.passed == true

    val started: Boolean
        get() = progress?.started == true || passed || purchased
}

data class IncidentKnowledgeAccess(
    val incidentKind: IncidentKind,
    val requirements: List<KnowledgeRequirementAccess>,
) {
    val gateState: KnowledgeGateState
        get() = when {
            requirements.isNotEmpty() && requirements.all { it.passed } -> KnowledgeGateState.MASTERED
            requirements.any { it.started } -> KnowledgeGateState.DISCOVERED
            else -> KnowledgeGateState.LOCKED
        }

    val mode: KnowledgeMode
        get() = if (gateState == KnowledgeGateState.MASTERED) {
            KnowledgeMode.NORMAL
        } else {
            KnowledgeMode.GUIDED
        }
}
