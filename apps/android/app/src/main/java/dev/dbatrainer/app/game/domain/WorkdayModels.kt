package dev.dbatrainer.app.game.domain

enum class WorkEventType {
    TASK,
    WARNING,
    INCIDENT,
}

enum class WorkEventSource {
    WORKSTATION,
    MONITORING,
    APPLICATION,
    SERVER,
    POSTGRESQL,
    NETWORK,
    BACKUP,
}

enum class DailyTaskKind {
    REVIEW_MONITORING,
    INSPECT_SERVER_ROOM,
}

data class WorldEvent(
    val id: String,
    val type: WorkEventType,
    val source: WorkEventSource,
    val targetNodeId: String? = null,
    val titleRu: String,
    val titleEn: String,
    val descriptionRu: String,
    val descriptionEn: String,
    val critical: Boolean = false,
)

data class WorkdayState(
    val day: Int = 1,
    val completedTasks: Set<DailyTaskKind> = emptySet(),
    val incidentsResolved: Int = 0,
    val shiftClosed: Boolean = false,
)

data class DailyShiftReport(
    val day: Int,
    val tasksCompleted: Int,
    val tasksTotal: Int,
    val incidentsResolved: Int,
    val companyBalance: Long,
    val playerCredits: Int,
)

data class MonthlyCompanyReport(
    val year: Int,
    val month: Int,
    val revenue: Long,
    val infrastructureCosts: Long,
    val payrollCosts: Long,
    val incidentLosses: Long,
    val netChange: Long,
    val playerSalaryCredits: Int,
    val closingBalance: Long,
)
