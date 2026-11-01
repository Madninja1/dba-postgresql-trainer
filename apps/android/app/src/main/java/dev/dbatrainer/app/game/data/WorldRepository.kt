package dev.dbatrainer.app.game.data

import android.content.Context
import android.content.SharedPreferences
import dev.dbatrainer.app.game.domain.CompanyState
import dev.dbatrainer.app.game.domain.DailyShiftReport
import dev.dbatrainer.app.game.domain.DailyTaskKind
import dev.dbatrainer.app.game.domain.DBA_WORLD_TIME_ZONE_ID
import dev.dbatrainer.app.game.domain.DISK_FULL_COMPANY_REWARD
import dev.dbatrainer.app.game.domain.DISK_FULL_PLAYER_CREDITS_REWARD
import dev.dbatrainer.app.game.domain.DISK_FULL_XP_REWARD
import dev.dbatrainer.app.game.domain.INITIAL_COMPANY_BALANCE
import dev.dbatrainer.app.game.domain.IncidentEvaluation
import dev.dbatrainer.app.game.domain.IncidentGrade
import dev.dbatrainer.app.game.domain.IncidentKind
import dev.dbatrainer.app.game.domain.IncidentProgress
import dev.dbatrainer.app.game.domain.IncidentSeverity
import dev.dbatrainer.app.game.domain.IncidentState
import dev.dbatrainer.app.game.domain.IncidentStatus
import dev.dbatrainer.app.game.domain.InfrastructureNode
import dev.dbatrainer.app.game.domain.InfrastructureNodeKind
import dev.dbatrainer.app.game.domain.InfrastructureStatus
import dev.dbatrainer.app.game.domain.LearningProductType
import dev.dbatrainer.app.game.domain.LearningTransaction
import dev.dbatrainer.app.game.domain.MonthlyCompanyReport
import dev.dbatrainer.app.game.domain.NodeMetrics
import dev.dbatrainer.app.game.domain.PlayerProgressState
import dev.dbatrainer.app.game.domain.PostgresqlConfigState
import dev.dbatrainer.app.game.domain.WorldClock
import dev.dbatrainer.app.game.domain.WorldRank
import dev.dbatrainer.app.game.domain.WorkdayState
import dev.dbatrainer.app.game.domain.WorldState
import dev.dbatrainer.app.game.domain.newDbaWorldState
import dev.dbatrainer.app.game.domain.learningProductKey
import dev.dbatrainer.app.game.domain.worldRankForXp

class WorldRepository(context: Context) {
    private val preferences = context.getSharedPreferences(
        "dba_world_state",
        Context.MODE_PRIVATE,
    )

    fun hasSavedWorld(): Boolean = preferences.getBoolean(KEY_WORLD_EXISTS, false)

    fun load(): WorldState? {
        if (!hasSavedWorld()) {
            return null
        }

        val saveVersion = preferences.getInt(KEY_SAVE_VERSION, 0)
        val clock = WorldClock(
            day = preferences.getInt(KEY_CLOCK_DAY, 1).coerceAtLeast(1),
            minuteOfDay = preferences.getInt(KEY_CLOCK_MINUTE, 9 * 60).coerceIn(0, 1439),
        )
        val incident = loadIncident(clock)
        val legacyResolvedIncident = saveVersion == 0 && incident?.status == IncidentStatus.RESOLVED
        val fallbackCompanyBalance = INITIAL_COMPANY_BALANCE +
            if (legacyResolvedIncident) DISK_FULL_COMPANY_REWARD else 0L
        val fallbackCredits = if (legacyResolvedIncident) DISK_FULL_PLAYER_CREDITS_REWARD else 0
        val fallbackXp = if (legacyResolvedIncident) DISK_FULL_XP_REWARD else 0

        val company = CompanyState(
            id = preferences.getString(KEY_COMPANY_ID, null) ?: "company-main",
            name = preferences.getString(KEY_COMPANY_NAME, null) ?: "DBA World Labs",
            balance = preferences.getLong(KEY_COMPANY_BALANCE, fallbackCompanyBalance),
        )
        val playerXp = preferences.getInt(KEY_PLAYER_XP, fallbackXp)
        val player = PlayerProgressState(
            credits = preferences.getInt(KEY_PLAYER_CREDITS, fallbackCredits),
            xp = playerXp,
            worldRank = preferences.getString(KEY_WORLD_RANK, null)
                ?.let { stored -> WorldRank.entries.firstOrNull { it.name == stored } }
                ?: worldRankForXp(playerXp),
        )
        val timeZoneId = preferences.getString(KEY_TIME_ZONE_ID, null)
            ?: DBA_WORLD_TIME_ZONE_ID
        val completed = loadCompletedIncidents().toMutableSet().apply {
            if (incident?.status == IncidentStatus.RESOLVED) {
                add(incident.kind)
            }
        }
        val purchasedTopicBundles = preferences
            .getStringSet(KEY_PURCHASED_TOPIC_BUNDLES, emptySet())
            .orEmpty()
            .toSet()
        val purchasedLearningProducts = preferences
            .getStringSet(KEY_PURCHASED_LEARNING_PRODUCTS, emptySet())
            .orEmpty()
            .toMutableSet()
            .apply {
                purchasedTopicBundles.forEach { topicKey ->
                    add(learningProductKey(topicKey, LearningProductType.BUNDLE))
                }
            }
            .toSet()
        val learningTransactions = loadLearningTransactions()
        val legacySharedBuffers = if (
            incident?.kind == IncidentKind.SERVICE_DOWN &&
            incident.status == IncidentStatus.ACTIVE &&
            !incident.progress.configFixed
        ) {
            "4ПИ"
        } else {
            "4GB"
        }
        val postgresqlConfig = PostgresqlConfigState(
            sharedBuffers = preferences.getString(KEY_POSTGRES_SHARED_BUFFERS, null)
                ?: legacySharedBuffers,
        )
        val activeTimeRemainderMillis = preferences
            .getLong(KEY_ACTIVE_TIME_REMAINDER_MILLIS, 0L)
            .coerceAtLeast(0L)
        val totalActiveRealMillis = preferences
            .getLong(KEY_TOTAL_ACTIVE_REAL_MILLIS, 0L)
            .coerceAtLeast(0L)
        val workday = WorkdayState(
            day = preferences.getInt(KEY_WORKDAY_DAY, clock.day).coerceAtLeast(1),
            completedTasks = preferences
                .getStringSet(KEY_WORKDAY_COMPLETED_TASKS, emptySet())
                .orEmpty()
                .mapNotNullTo(mutableSetOf()) { stored ->
                    DailyTaskKind.entries.firstOrNull { it.name == stored }
                },
            incidentsResolved = preferences.getInt(KEY_WORKDAY_INCIDENTS_RESOLVED, 0).coerceAtLeast(0),
            shiftClosed = preferences.getBoolean(KEY_WORKDAY_SHIFT_CLOSED, false),
        )
        val lastShiftReport = loadShiftReport()
        val lastMonthlyReport = loadMonthlyReport()
        val currentSalaryPeriodKey = clock.date.year * 12 + clock.date.month
        val lastSalaryPeriodKey = if (saveVersion < SAVE_VERSION_2045_EPOCH) {
            // 0.7.7 moves the fictional DBA World epoch from 2026 to 2045.
            // Clock day/minute stay intact, but carrying the old absolute
            // salary period into the new epoch would award ~19 years of
            // catch-up salaries on the next active-time tick.
            currentSalaryPeriodKey
        } else {
            preferences.getInt(KEY_LAST_SALARY_PERIOD, currentSalaryPeriodKey)
        }

        // MVP 0.1 points 1-7 had no incident/save schema. Upgrade only those old saves.
        if (incident == null && saveVersion < 2 && completed.isEmpty()) {
            return newDbaWorldState().copy(
                company = company,
                player = player,
                timeZoneId = timeZoneId,
                clock = clock,
                activeTimeRemainderMillis = activeTimeRemainderMillis,
                totalActiveRealMillis = totalActiveRealMillis,
                postgresqlConfig = postgresqlConfig,
                purchasedTopicBundles = purchasedTopicBundles,
                purchasedLearningProducts = purchasedLearningProducts,
                learningTransactions = learningTransactions,
                workday = workday,
                lastShiftReport = lastShiftReport,
                lastMonthlyReport = lastMonthlyReport,
                lastSalaryPeriodKey = lastSalaryPeriodKey,
            )
        }

        return WorldState(
            company = company,
            player = player,
            nodes = listOf(
                loadNode(
                    prefix = "app",
                    fallbackId = "app-01",
                    fallbackName = "Application",
                    fallbackKind = InfrastructureNodeKind.APPLICATION,
                ),
                loadNode(
                    prefix = "db",
                    fallbackId = "db-primary",
                    fallbackName = "PostgreSQL",
                    fallbackKind = InfrastructureNodeKind.POSTGRESQL,
                ),
            ),
            incident = incident,
            completedIncidents = completed,
            lastIncidentEvaluation = loadEvaluation(),
            clock = clock,
            activeTimeRemainderMillis = activeTimeRemainderMillis,
            totalActiveRealMillis = totalActiveRealMillis,
            postgresqlConfig = postgresqlConfig,
            purchasedTopicBundles = purchasedTopicBundles,
            purchasedLearningProducts = purchasedLearningProducts,
            learningTransactions = learningTransactions,
            workday = workday,
            lastShiftReport = lastShiftReport,
            lastMonthlyReport = lastMonthlyReport,
            lastSalaryPeriodKey = lastSalaryPeriodKey,
            timeZoneId = timeZoneId,
        )
    }

    fun save(world: WorldState) {
        val application = world.nodes.first { it.kind == InfrastructureNodeKind.APPLICATION }
        val database = world.nodes.first { it.kind == InfrastructureNodeKind.POSTGRESQL }

        preferences.edit()
            .putBoolean(KEY_WORLD_EXISTS, true)
            .putInt(KEY_SAVE_VERSION, CURRENT_SAVE_VERSION)
            .putString(KEY_COMPANY_ID, world.company.id)
            .putString(KEY_COMPANY_NAME, world.company.name)
            .putLong(KEY_COMPANY_BALANCE, world.company.balance)
            .putInt(KEY_PLAYER_CREDITS, world.player.credits)
            .putInt(KEY_PLAYER_XP, world.player.xp)
            .putString(KEY_WORLD_RANK, world.player.worldRank.name)
            .putString(KEY_TIME_ZONE_ID, world.timeZoneId)
            .putInt(KEY_CLOCK_DAY, world.clock.day)
            .putInt(KEY_CLOCK_MINUTE, world.clock.minuteOfDay)
            .putLong(KEY_ACTIVE_TIME_REMAINDER_MILLIS, world.activeTimeRemainderMillis)
            .putLong(KEY_TOTAL_ACTIVE_REAL_MILLIS, world.totalActiveRealMillis)
            .putString(KEY_POSTGRES_SHARED_BUFFERS, world.postgresqlConfig.sharedBuffers)
            .putStringSet(
                KEY_COMPLETED_INCIDENTS,
                world.completedIncidents.mapTo(mutableSetOf()) { it.name },
            )
            .putStringSet(
                KEY_PURCHASED_TOPIC_BUNDLES,
                world.purchasedTopicBundles.toMutableSet(),
            )
            .putStringSet(
                KEY_PURCHASED_LEARNING_PRODUCTS,
                world.purchasedLearningProducts.toMutableSet(),
            )
            .putString(
                KEY_LEARNING_TRANSACTIONS,
                encodeLearningTransactions(world.learningTransactions),
            )
            .putInt(KEY_WORKDAY_DAY, world.workday.day)
            .putStringSet(
                KEY_WORKDAY_COMPLETED_TASKS,
                world.workday.completedTasks.mapTo(mutableSetOf()) { it.name },
            )
            .putInt(KEY_WORKDAY_INCIDENTS_RESOLVED, world.workday.incidentsResolved)
            .putBoolean(KEY_WORKDAY_SHIFT_CLOSED, world.workday.shiftClosed)
            .putInt(KEY_LAST_SALARY_PERIOD, world.lastSalaryPeriodKey)
            .also { editor ->
                saveShiftReport(editor, world.lastShiftReport)
                saveMonthlyReport(editor, world.lastMonthlyReport)
                saveNode(editor, "app", application)
                saveNode(editor, "db", database)
                saveIncident(editor, world.incident)
                saveEvaluation(editor, world.lastIncidentEvaluation)
            }
            .apply()
    }

    private fun loadIncident(clock: WorldClock): IncidentState? {
        val id = preferences.getString(KEY_INCIDENT_ID, null) ?: return null
        val kind = enumValueOrNull<IncidentKind>(preferences.getString(KEY_INCIDENT_KIND, null))
            ?: IncidentKind.DISK_FULL
        val status = enumValueOrNull<IncidentStatus>(preferences.getString(KEY_INCIDENT_STATUS, null))
            ?: IncidentStatus.ACTIVE
        val legacyDiskFreed = preferences.getBoolean(KEY_INCIDENT_DISK_FREED, false)

        return IncidentState(
            id = id,
            kind = kind,
            title = preferences.getString(KEY_INCIDENT_TITLE, null) ?: incidentTitle(kind),
            severity = enumValueOrNull<IncidentSeverity>(preferences.getString(KEY_INCIDENT_SEVERITY, null))
                ?: incidentSeverity(kind),
            status = status,
            rootCause = preferences.getString(KEY_INCIDENT_ROOT_CAUSE, null) ?: incidentRootCause(kind),
            startedAtTotalMinutes = preferences.getInt(
                KEY_INCIDENT_STARTED_AT,
                clock.totalMinutes(),
            ),
            actionCount = preferences.getInt(KEY_INCIDENT_ACTION_COUNT, 0),
            hintsUsed = preferences.getInt(KEY_INCIDENT_HINTS_USED, 0),
            progress = IncidentProgress(
                serviceInspected = preferences.getBoolean(KEY_PROGRESS_SERVICE, false),
                journalInspected = preferences.getBoolean(KEY_PROGRESS_JOURNAL, false),
                diskInspected = preferences.getBoolean(KEY_PROGRESS_DISK, false),
                logsInspected = preferences.getBoolean(KEY_PROGRESS_LOGS, false),
                diskSpaceFreed = preferences.getBoolean(KEY_PROGRESS_DISK_FREED, legacyDiskFreed),
                configInspected = preferences.getBoolean(KEY_PROGRESS_CONFIG, false),
                configFixed = preferences.getBoolean(KEY_PROGRESS_CONFIG_FIXED, false),
                processesInspected = preferences.getBoolean(KEY_PROGRESS_PROCESSES, false),
                activityInspected = preferences.getBoolean(KEY_PROGRESS_ACTIVITY, false),
                badQueryCancelled = preferences.getBoolean(KEY_PROGRESS_QUERY_CANCELLED, false),
                unsafeAttempts = preferences.getInt(KEY_PROGRESS_UNSAFE_ATTEMPTS, 0),
            ),
        )
    }

    private fun saveIncident(
        editor: SharedPreferences.Editor,
        incident: IncidentState?,
    ) {
        if (incident == null) {
            INCIDENT_KEYS.forEach { editor.remove(it) }
            return
        }

        editor
            .putString(KEY_INCIDENT_ID, incident.id)
            .putString(KEY_INCIDENT_KIND, incident.kind.name)
            .putString(KEY_INCIDENT_TITLE, incident.title)
            .putString(KEY_INCIDENT_SEVERITY, incident.severity.name)
            .putString(KEY_INCIDENT_STATUS, incident.status.name)
            .putString(KEY_INCIDENT_ROOT_CAUSE, incident.rootCause)
            .putInt(KEY_INCIDENT_STARTED_AT, incident.startedAtTotalMinutes)
            .putInt(KEY_INCIDENT_ACTION_COUNT, incident.actionCount)
            .putInt(KEY_INCIDENT_HINTS_USED, incident.hintsUsed)
            .putBoolean(KEY_PROGRESS_SERVICE, incident.progress.serviceInspected)
            .putBoolean(KEY_PROGRESS_JOURNAL, incident.progress.journalInspected)
            .putBoolean(KEY_PROGRESS_DISK, incident.progress.diskInspected)
            .putBoolean(KEY_PROGRESS_LOGS, incident.progress.logsInspected)
            .putBoolean(KEY_PROGRESS_DISK_FREED, incident.progress.diskSpaceFreed)
            .putBoolean(KEY_PROGRESS_CONFIG, incident.progress.configInspected)
            .putBoolean(KEY_PROGRESS_CONFIG_FIXED, incident.progress.configFixed)
            .putBoolean(KEY_PROGRESS_PROCESSES, incident.progress.processesInspected)
            .putBoolean(KEY_PROGRESS_ACTIVITY, incident.progress.activityInspected)
            .putBoolean(KEY_PROGRESS_QUERY_CANCELLED, incident.progress.badQueryCancelled)
            .putInt(KEY_PROGRESS_UNSAFE_ATTEMPTS, incident.progress.unsafeAttempts)
            .remove(KEY_INCIDENT_DISK_FREED)
    }

    private fun loadCompletedIncidents(): Set<IncidentKind> = preferences
        .getStringSet(KEY_COMPLETED_INCIDENTS, emptySet())
        .orEmpty()
        .mapNotNullTo(mutableSetOf()) { stored ->
            IncidentKind.entries.firstOrNull { it.name == stored }
        }

    private fun loadEvaluation(): IncidentEvaluation? {
        val kind = enumValueOrNull<IncidentKind>(preferences.getString(KEY_EVAL_KIND, null))
            ?: return null
        return IncidentEvaluation(
            incidentKind = kind,
            title = preferences.getString(KEY_EVAL_TITLE, null) ?: incidentTitle(kind),
            diagnosis = enumValueOrNull<IncidentGrade>(preferences.getString(KEY_EVAL_DIAGNOSIS, null))
                ?: IncidentGrade.D,
            safety = enumValueOrNull<IncidentGrade>(preferences.getString(KEY_EVAL_SAFETY, null))
                ?: IncidentGrade.D,
            time = enumValueOrNull<IncidentGrade>(preferences.getString(KEY_EVAL_TIME, null))
                ?: IncidentGrade.D,
            hintsUsed = preferences.getInt(KEY_EVAL_HINTS, 0),
            elapsedMinutes = preferences.getInt(KEY_EVAL_ELAPSED, 0),
            score = preferences.getInt(KEY_EVAL_SCORE, 0),
        )
    }

    private fun saveEvaluation(
        editor: SharedPreferences.Editor,
        evaluation: IncidentEvaluation?,
    ) {
        if (evaluation == null) {
            EVALUATION_KEYS.forEach { editor.remove(it) }
            return
        }
        editor
            .putString(KEY_EVAL_KIND, evaluation.incidentKind.name)
            .putString(KEY_EVAL_TITLE, evaluation.title)
            .putString(KEY_EVAL_DIAGNOSIS, evaluation.diagnosis.name)
            .putString(KEY_EVAL_SAFETY, evaluation.safety.name)
            .putString(KEY_EVAL_TIME, evaluation.time.name)
            .putInt(KEY_EVAL_HINTS, evaluation.hintsUsed)
            .putInt(KEY_EVAL_ELAPSED, evaluation.elapsedMinutes)
            .putInt(KEY_EVAL_SCORE, evaluation.score)
    }

    private fun loadLearningTransactions(): List<LearningTransaction> = preferences
        .getString(KEY_LEARNING_TRANSACTIONS, null)
        .orEmpty()
        .lineSequence()
        .filter { it.isNotBlank() }
        .mapNotNull { line ->
            val parts = line.split('|')
            if (parts.size != 6) {
                return@mapNotNull null
            }
            val id = parts[0].toLongOrNull() ?: return@mapNotNull null
            val productType = enumValueOrNull<LearningProductType>(parts[2])
                ?: return@mapNotNull null
            val price = parts[3].toIntOrNull() ?: return@mapNotNull null
            val balanceAfter = parts[4].toIntOrNull() ?: return@mapNotNull null
            val worldTotalMinutes = parts[5].toIntOrNull() ?: return@mapNotNull null
            LearningTransaction(
                id = id,
                topicKey = parts[1],
                productType = productType,
                price = price,
                balanceAfter = balanceAfter,
                worldTotalMinutes = worldTotalMinutes,
            )
        }
        .sortedBy { it.id }
        .toList()

    private fun encodeLearningTransactions(transactions: List<LearningTransaction>): String = transactions
        .joinToString("\n") { transaction ->
            listOf(
                transaction.id,
                transaction.topicKey,
                transaction.productType.name,
                transaction.price,
                transaction.balanceAfter,
                transaction.worldTotalMinutes,
            ).joinToString("|")
        }

    private fun loadShiftReport(): DailyShiftReport? {
        val day = preferences.getInt(KEY_SHIFT_REPORT_DAY, 0)
        if (day <= 0) {
            return null
        }
        return DailyShiftReport(
            day = day,
            tasksCompleted = preferences.getInt(KEY_SHIFT_REPORT_TASKS_COMPLETED, 0),
            tasksTotal = preferences.getInt(KEY_SHIFT_REPORT_TASKS_TOTAL, 0),
            incidentsResolved = preferences.getInt(KEY_SHIFT_REPORT_INCIDENTS, 0),
            companyBalance = preferences.getLong(KEY_SHIFT_REPORT_COMPANY_BALANCE, INITIAL_COMPANY_BALANCE),
            playerCredits = preferences.getInt(KEY_SHIFT_REPORT_PLAYER_CREDITS, 0),
        )
    }

    private fun saveShiftReport(
        editor: SharedPreferences.Editor,
        report: DailyShiftReport?,
    ) {
        if (report == null) {
            SHIFT_REPORT_KEYS.forEach(editor::remove)
            return
        }
        editor
            .putInt(KEY_SHIFT_REPORT_DAY, report.day)
            .putInt(KEY_SHIFT_REPORT_TASKS_COMPLETED, report.tasksCompleted)
            .putInt(KEY_SHIFT_REPORT_TASKS_TOTAL, report.tasksTotal)
            .putInt(KEY_SHIFT_REPORT_INCIDENTS, report.incidentsResolved)
            .putLong(KEY_SHIFT_REPORT_COMPANY_BALANCE, report.companyBalance)
            .putInt(KEY_SHIFT_REPORT_PLAYER_CREDITS, report.playerCredits)
    }

    private fun loadMonthlyReport(): MonthlyCompanyReport? {
        val year = preferences.getInt(KEY_MONTHLY_REPORT_YEAR, 0)
        val month = preferences.getInt(KEY_MONTHLY_REPORT_MONTH, 0)
        if (year <= 0 || month !in 1..12) {
            return null
        }
        return MonthlyCompanyReport(
            year = year,
            month = month,
            revenue = preferences.getLong(KEY_MONTHLY_REPORT_REVENUE, 0L),
            infrastructureCosts = preferences.getLong(KEY_MONTHLY_REPORT_INFRA_COSTS, 0L),
            payrollCosts = preferences.getLong(KEY_MONTHLY_REPORT_PAYROLL_COSTS, 0L),
            incidentLosses = preferences.getLong(KEY_MONTHLY_REPORT_INCIDENT_LOSSES, 0L),
            netChange = preferences.getLong(KEY_MONTHLY_REPORT_NET_CHANGE, 0L),
            playerSalaryCredits = preferences.getInt(KEY_MONTHLY_REPORT_SALARY, 0),
            closingBalance = preferences.getLong(KEY_MONTHLY_REPORT_CLOSING_BALANCE, INITIAL_COMPANY_BALANCE),
        )
    }

    private fun saveMonthlyReport(
        editor: SharedPreferences.Editor,
        report: MonthlyCompanyReport?,
    ) {
        if (report == null) {
            MONTHLY_REPORT_KEYS.forEach(editor::remove)
            return
        }
        editor
            .putInt(KEY_MONTHLY_REPORT_YEAR, report.year)
            .putInt(KEY_MONTHLY_REPORT_MONTH, report.month)
            .putLong(KEY_MONTHLY_REPORT_REVENUE, report.revenue)
            .putLong(KEY_MONTHLY_REPORT_INFRA_COSTS, report.infrastructureCosts)
            .putLong(KEY_MONTHLY_REPORT_PAYROLL_COSTS, report.payrollCosts)
            .putLong(KEY_MONTHLY_REPORT_INCIDENT_LOSSES, report.incidentLosses)
            .putLong(KEY_MONTHLY_REPORT_NET_CHANGE, report.netChange)
            .putInt(KEY_MONTHLY_REPORT_SALARY, report.playerSalaryCredits)
            .putLong(KEY_MONTHLY_REPORT_CLOSING_BALANCE, report.closingBalance)
    }

    private fun loadNode(
        prefix: String,
        fallbackId: String,
        fallbackName: String,
        fallbackKind: InfrastructureNodeKind,
    ): InfrastructureNode = InfrastructureNode(
        id = preferences.getString("${prefix}_id", null) ?: fallbackId,
        name = preferences.getString("${prefix}_name", null) ?: fallbackName,
        kind = enumValueOrNull<InfrastructureNodeKind>(preferences.getString("${prefix}_kind", null))
            ?: fallbackKind,
        status = enumValueOrNull<InfrastructureStatus>(preferences.getString("${prefix}_status", null))
            ?: InfrastructureStatus.HEALTHY,
        metrics = NodeMetrics(
            cpuPercent = preferences.getInt("${prefix}_cpu", 0),
            ramPercent = preferences.getInt("${prefix}_ram", 0),
            diskPercent = preferences.getInt("${prefix}_disk", 0),
        ),
    )

    private fun saveNode(
        editor: SharedPreferences.Editor,
        prefix: String,
        node: InfrastructureNode,
    ) {
        editor
            .putString("${prefix}_id", node.id)
            .putString("${prefix}_name", node.name)
            .putString("${prefix}_kind", node.kind.name)
            .putString("${prefix}_status", node.status.name)
            .putInt("${prefix}_cpu", node.metrics.cpuPercent)
            .putInt("${prefix}_ram", node.metrics.ramPercent)
            .putInt("${prefix}_disk", node.metrics.diskPercent)
    }

    private inline fun <reified T : Enum<T>> enumValueOrNull(value: String?): T? =
        value?.let { stored -> enumValues<T>().firstOrNull { it.name == stored } }

    private fun incidentTitle(kind: IncidentKind): String = when (kind) {
        IncidentKind.DISK_FULL -> "Disk Full"
        IncidentKind.SERVICE_DOWN -> "PostgreSQL Service Down"
        IncidentKind.HIGH_CPU_BAD_QUERY -> "High CPU / Bad Query"
    }

    private fun incidentSeverity(kind: IncidentKind): IncidentSeverity = when (kind) {
        IncidentKind.DISK_FULL -> IncidentSeverity.CRITICAL
        IncidentKind.SERVICE_DOWN, IncidentKind.HIGH_CPU_BAD_QUERY -> IncidentSeverity.HIGH
    }

    private fun incidentRootCause(kind: IncidentKind): String = when (kind) {
        IncidentKind.DISK_FULL -> "Rotated PostgreSQL log filled /var."
        IncidentKind.SERVICE_DOWN -> "shared_buffers was typed as 4ПИ with the Russian keyboard layout instead of 4GB."
        IncidentKind.HIGH_CPU_BAD_QUERY -> "Runaway analytical query is consuming database CPU."
    }

    private companion object {
        const val CURRENT_SAVE_VERSION = 8
        const val SAVE_VERSION_2045_EPOCH = 8
        const val KEY_WORLD_EXISTS = "world_exists"
        const val KEY_SAVE_VERSION = "save_version"
        const val KEY_COMPANY_ID = "company_id"
        const val KEY_COMPANY_NAME = "company_name"
        const val KEY_COMPANY_BALANCE = "company_balance"
        const val KEY_PLAYER_CREDITS = "player_credits"
        const val KEY_PLAYER_XP = "player_xp"
        const val KEY_WORLD_RANK = "world_rank"
        const val KEY_TIME_ZONE_ID = "time_zone_id"
        const val KEY_CLOCK_DAY = "clock_day"
        const val KEY_CLOCK_MINUTE = "clock_minute"
        const val KEY_ACTIVE_TIME_REMAINDER_MILLIS = "active_time_remainder_millis"
        const val KEY_TOTAL_ACTIVE_REAL_MILLIS = "total_active_real_millis"
        const val KEY_POSTGRES_SHARED_BUFFERS = "postgres_shared_buffers"
        const val KEY_COMPLETED_INCIDENTS = "completed_incidents"
        const val KEY_PURCHASED_TOPIC_BUNDLES = "purchased_topic_bundles"
        const val KEY_PURCHASED_LEARNING_PRODUCTS = "purchased_learning_products"
        const val KEY_LEARNING_TRANSACTIONS = "learning_transactions"
        const val KEY_WORKDAY_DAY = "workday_day"
        const val KEY_WORKDAY_COMPLETED_TASKS = "workday_completed_tasks"
        const val KEY_WORKDAY_INCIDENTS_RESOLVED = "workday_incidents_resolved"
        const val KEY_WORKDAY_SHIFT_CLOSED = "workday_shift_closed"
        const val KEY_LAST_SALARY_PERIOD = "last_salary_period"

        const val KEY_SHIFT_REPORT_DAY = "shift_report_day"
        const val KEY_SHIFT_REPORT_TASKS_COMPLETED = "shift_report_tasks_completed"
        const val KEY_SHIFT_REPORT_TASKS_TOTAL = "shift_report_tasks_total"
        const val KEY_SHIFT_REPORT_INCIDENTS = "shift_report_incidents"
        const val KEY_SHIFT_REPORT_COMPANY_BALANCE = "shift_report_company_balance"
        const val KEY_SHIFT_REPORT_PLAYER_CREDITS = "shift_report_player_credits"

        const val KEY_MONTHLY_REPORT_YEAR = "monthly_report_year"
        const val KEY_MONTHLY_REPORT_MONTH = "monthly_report_month"
        const val KEY_MONTHLY_REPORT_REVENUE = "monthly_report_revenue"
        const val KEY_MONTHLY_REPORT_INFRA_COSTS = "monthly_report_infra_costs"
        const val KEY_MONTHLY_REPORT_PAYROLL_COSTS = "monthly_report_payroll_costs"
        const val KEY_MONTHLY_REPORT_INCIDENT_LOSSES = "monthly_report_incident_losses"
        const val KEY_MONTHLY_REPORT_NET_CHANGE = "monthly_report_net_change"
        const val KEY_MONTHLY_REPORT_SALARY = "monthly_report_salary"
        const val KEY_MONTHLY_REPORT_CLOSING_BALANCE = "monthly_report_closing_balance"

        const val KEY_INCIDENT_ID = "incident_id"
        const val KEY_INCIDENT_KIND = "incident_kind"
        const val KEY_INCIDENT_TITLE = "incident_title"
        const val KEY_INCIDENT_SEVERITY = "incident_severity"
        const val KEY_INCIDENT_STATUS = "incident_status"
        const val KEY_INCIDENT_ROOT_CAUSE = "incident_root_cause"
        const val KEY_INCIDENT_STARTED_AT = "incident_started_at"
        const val KEY_INCIDENT_ACTION_COUNT = "incident_action_count"
        const val KEY_INCIDENT_HINTS_USED = "incident_hints_used"
        const val KEY_INCIDENT_DISK_FREED = "incident_disk_freed"
        const val KEY_PROGRESS_SERVICE = "progress_service"
        const val KEY_PROGRESS_JOURNAL = "progress_journal"
        const val KEY_PROGRESS_DISK = "progress_disk"
        const val KEY_PROGRESS_LOGS = "progress_logs"
        const val KEY_PROGRESS_DISK_FREED = "progress_disk_freed"
        const val KEY_PROGRESS_CONFIG = "progress_config"
        const val KEY_PROGRESS_CONFIG_FIXED = "progress_config_fixed"
        const val KEY_PROGRESS_PROCESSES = "progress_processes"
        const val KEY_PROGRESS_ACTIVITY = "progress_activity"
        const val KEY_PROGRESS_QUERY_CANCELLED = "progress_query_cancelled"
        const val KEY_PROGRESS_UNSAFE_ATTEMPTS = "progress_unsafe_attempts"

        const val KEY_EVAL_KIND = "evaluation_kind"
        const val KEY_EVAL_TITLE = "evaluation_title"
        const val KEY_EVAL_DIAGNOSIS = "evaluation_diagnosis"
        const val KEY_EVAL_SAFETY = "evaluation_safety"
        const val KEY_EVAL_TIME = "evaluation_time"
        const val KEY_EVAL_HINTS = "evaluation_hints"
        const val KEY_EVAL_ELAPSED = "evaluation_elapsed"
        const val KEY_EVAL_SCORE = "evaluation_score"

        val SHIFT_REPORT_KEYS = listOf(
            KEY_SHIFT_REPORT_DAY,
            KEY_SHIFT_REPORT_TASKS_COMPLETED,
            KEY_SHIFT_REPORT_TASKS_TOTAL,
            KEY_SHIFT_REPORT_INCIDENTS,
            KEY_SHIFT_REPORT_COMPANY_BALANCE,
            KEY_SHIFT_REPORT_PLAYER_CREDITS,
        )

        val MONTHLY_REPORT_KEYS = listOf(
            KEY_MONTHLY_REPORT_YEAR,
            KEY_MONTHLY_REPORT_MONTH,
            KEY_MONTHLY_REPORT_REVENUE,
            KEY_MONTHLY_REPORT_INFRA_COSTS,
            KEY_MONTHLY_REPORT_PAYROLL_COSTS,
            KEY_MONTHLY_REPORT_INCIDENT_LOSSES,
            KEY_MONTHLY_REPORT_NET_CHANGE,
            KEY_MONTHLY_REPORT_SALARY,
            KEY_MONTHLY_REPORT_CLOSING_BALANCE,
        )

        val INCIDENT_KEYS = listOf(
            KEY_INCIDENT_ID,
            KEY_INCIDENT_KIND,
            KEY_INCIDENT_TITLE,
            KEY_INCIDENT_SEVERITY,
            KEY_INCIDENT_STATUS,
            KEY_INCIDENT_ROOT_CAUSE,
            KEY_INCIDENT_STARTED_AT,
            KEY_INCIDENT_ACTION_COUNT,
            KEY_INCIDENT_HINTS_USED,
            KEY_INCIDENT_DISK_FREED,
            KEY_PROGRESS_SERVICE,
            KEY_PROGRESS_JOURNAL,
            KEY_PROGRESS_DISK,
            KEY_PROGRESS_LOGS,
            KEY_PROGRESS_DISK_FREED,
            KEY_PROGRESS_CONFIG,
            KEY_PROGRESS_CONFIG_FIXED,
            KEY_PROGRESS_PROCESSES,
            KEY_PROGRESS_ACTIVITY,
            KEY_PROGRESS_QUERY_CANCELLED,
            KEY_PROGRESS_UNSAFE_ATTEMPTS,
        )

        val EVALUATION_KEYS = listOf(
            KEY_EVAL_KIND,
            KEY_EVAL_TITLE,
            KEY_EVAL_DIAGNOSIS,
            KEY_EVAL_SAFETY,
            KEY_EVAL_TIME,
            KEY_EVAL_HINTS,
            KEY_EVAL_ELAPSED,
            KEY_EVAL_SCORE,
        )
    }
}
