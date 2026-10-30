package dev.dbatrainer.app.game.domain

const val DBA_WORLD_TIME_ZONE_ID = "Europe/Moscow"
const val INITIAL_COMPANY_BALANCE = 25_000L
const val DBA_WORLD_START_YEAR = 2026
const val REAL_MILLIS_PER_GAME_MINUTE = 5_000L

const val DISK_FULL_COMPANY_REWARD = 1_500L
const val DISK_FULL_PLAYER_CREDITS_REWARD = 150
const val DISK_FULL_XP_REWARD = 120

const val SERVICE_DOWN_COMPANY_REWARD = 1_200L
const val SERVICE_DOWN_PLAYER_CREDITS_REWARD = 120
const val SERVICE_DOWN_XP_REWARD = 100

const val HIGH_CPU_COMPANY_REWARD = 2_000L
const val HIGH_CPU_PLAYER_CREDITS_REWARD = 180
const val HIGH_CPU_XP_REWARD = 180

enum class InfrastructureNodeKind {
    APPLICATION,
    POSTGRESQL,
}

enum class InfrastructureStatus {
    HEALTHY,
    WARNING,
    DOWN,
}

enum class IncidentKind {
    DISK_FULL,
    SERVICE_DOWN,
    HIGH_CPU_BAD_QUERY,
}

enum class IncidentSeverity {
    MEDIUM,
    HIGH,
    CRITICAL,
}

enum class IncidentStatus {
    ACTIVE,
    RESOLVED,
}

enum class IncidentGrade {
    A,
    B,
    C,
    D,
}

enum class WorldRank(val minimumXp: Int) {
    TRAINEE_OPERATOR(0),
    JUNIOR_OPERATOR(100),
    DB_ENGINEER(300),
    PRODUCTION_DBA(700),
    SENIOR_DBA(1_500),
    INCIDENT_COMMANDER(3_000),
    INFRASTRUCTURE_ARCHITECT(6_000),
}

data class NodeMetrics(
    val cpuPercent: Int,
    val ramPercent: Int,
    val diskPercent: Int,
)

data class InfrastructureNode(
    val id: String,
    val name: String,
    val kind: InfrastructureNodeKind,
    val status: InfrastructureStatus,
    val metrics: NodeMetrics,
)

data class CompanyState(
    val id: String,
    val name: String,
    val balance: Long,
)

data class PlayerProgressState(
    val credits: Int,
    val xp: Int,
    val worldRank: WorldRank,
)

data class WorldDate(
    val year: Int,
    val month: Int,
    val dayOfMonth: Int,
)

data class WorldClock(
    val day: Int = 1,
    val minuteOfDay: Int = 9 * 60,
) {
    init {
        require(day >= 1)
        require(minuteOfDay in 0 until MINUTES_PER_DAY)
    }

    val hour: Int
        get() = minuteOfDay / 60

    val minute: Int
        get() = minuteOfDay % 60

    val date: WorldDate
        get() = dateForDay(day)

    fun totalMinutes(): Int = (day - 1) * MINUTES_PER_DAY + minuteOfDay

    fun advance(minutes: Int): WorldClock {
        require(minutes >= 0)
        val total = totalMinutes() + minutes
        return WorldClock(
            day = total / MINUTES_PER_DAY + 1,
            minuteOfDay = total % MINUTES_PER_DAY,
        )
    }

    private fun dateForDay(dayNumber: Int): WorldDate {
        var remainingDays = dayNumber - 1
        var year = DBA_WORLD_START_YEAR
        while (remainingDays >= daysInYear(year)) {
            remainingDays -= daysInYear(year)
            year += 1
        }

        var month = 1
        while (remainingDays >= daysInMonth(year, month)) {
            remainingDays -= daysInMonth(year, month)
            month += 1
        }
        return WorldDate(
            year = year,
            month = month,
            dayOfMonth = remainingDays + 1,
        )
    }

    private fun daysInYear(year: Int): Int = if (isLeapYear(year)) 366 else 365

    private fun daysInMonth(year: Int, month: Int): Int = when (month) {
        1, 3, 5, 7, 8, 10, 12 -> 31
        4, 6, 9, 11 -> 30
        2 -> if (isLeapYear(year)) 29 else 28
        else -> error("Invalid month: $month")
    }

    private fun isLeapYear(year: Int): Boolean =
        year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)

    private companion object {
        const val MINUTES_PER_DAY = 24 * 60
    }
}

data class IncidentProgress(
    val serviceInspected: Boolean = false,
    val journalInspected: Boolean = false,
    val diskInspected: Boolean = false,
    val logsInspected: Boolean = false,
    val diskSpaceFreed: Boolean = false,
    val configInspected: Boolean = false,
    val configFixed: Boolean = false,
    val processesInspected: Boolean = false,
    val activityInspected: Boolean = false,
    val badQueryCancelled: Boolean = false,
    val unsafeAttempts: Int = 0,
)

data class IncidentState(
    val id: String,
    val kind: IncidentKind,
    val title: String,
    val severity: IncidentSeverity,
    val status: IncidentStatus,
    val rootCause: String,
    val startedAtTotalMinutes: Int,
    val actionCount: Int = 0,
    val hintsUsed: Int = 0,
    val progress: IncidentProgress = IncidentProgress(),
)

data class IncidentEvaluation(
    val incidentKind: IncidentKind,
    val title: String,
    val diagnosis: IncidentGrade,
    val safety: IncidentGrade,
    val time: IncidentGrade,
    val hintsUsed: Int,
    val elapsedMinutes: Int,
    val score: Int,
)

data class PostgresqlConfigState(
    val sharedBuffers: String = "4GB",
)

data class WorldState(
    val company: CompanyState,
    val player: PlayerProgressState,
    val nodes: List<InfrastructureNode>,
    val incident: IncidentState?,
    val completedIncidents: Set<IncidentKind> = emptySet(),
    val lastIncidentEvaluation: IncidentEvaluation? = null,
    val clock: WorldClock = WorldClock(),
    val activeTimeRemainderMillis: Long = 0L,
    val totalActiveRealMillis: Long = 0L,
    val postgresqlConfig: PostgresqlConfigState = PostgresqlConfigState(),
    val purchasedTopicBundles: Set<String> = emptySet(),
    val purchasedLearningProducts: Set<String> = emptySet(),
    val learningTransactions: List<LearningTransaction> = emptyList(),
    val workday: WorkdayState = WorkdayState(),
    val lastShiftReport: DailyShiftReport? = null,
    val lastMonthlyReport: MonthlyCompanyReport? = null,
    val lastSalaryPeriodKey: Int = DBA_WORLD_START_YEAR * 12 + 1,
    val timeZoneId: String = DBA_WORLD_TIME_ZONE_ID,
)

fun worldRankForXp(xp: Int): WorldRank = WorldRank.entries
    .lastOrNull { xp >= it.minimumXp }
    ?: WorldRank.TRAINEE_OPERATOR

fun nextWorldRank(rank: WorldRank): WorldRank? {
    val currentIndex = WorldRank.entries.indexOf(rank)
    return WorldRank.entries.getOrNull(currentIndex + 1)
}

fun newDbaWorldState(): WorldState {
    val clock = WorldClock()
    return WorldState(
        company = CompanyState(
            id = "company-main",
            name = "DBA World Labs",
            balance = INITIAL_COMPANY_BALANCE,
        ),
        player = PlayerProgressState(
            credits = 0,
            xp = 0,
            worldRank = WorldRank.TRAINEE_OPERATOR,
        ),
        nodes = listOf(
            InfrastructureNode(
                id = "app-01",
                name = "Application",
                kind = InfrastructureNodeKind.APPLICATION,
                status = InfrastructureStatus.WARNING,
                metrics = NodeMetrics(
                    cpuPercent = 24,
                    ramPercent = 38,
                    diskPercent = 31,
                ),
            ),
            InfrastructureNode(
                id = "db-primary",
                name = "PostgreSQL",
                kind = InfrastructureNodeKind.POSTGRESQL,
                status = InfrastructureStatus.DOWN,
                metrics = NodeMetrics(
                    cpuPercent = 3,
                    ramPercent = 34,
                    diskPercent = 100,
                ),
            ),
        ),
        incident = IncidentState(
            id = "incident-disk-full-01",
            kind = IncidentKind.DISK_FULL,
            title = "Disk Full",
            severity = IncidentSeverity.CRITICAL,
            status = IncidentStatus.ACTIVE,
            rootCause = "Rotated PostgreSQL log filled /var.",
            startedAtTotalMinutes = clock.totalMinutes(),
        ),
        clock = clock,
    )
}
