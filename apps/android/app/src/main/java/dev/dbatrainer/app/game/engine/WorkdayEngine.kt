package dev.dbatrainer.app.game.engine

import dev.dbatrainer.app.game.domain.DailyShiftReport
import dev.dbatrainer.app.game.domain.DailyTaskKind
import dev.dbatrainer.app.game.domain.InfrastructureStatus
import dev.dbatrainer.app.game.domain.MonthlyCompanyReport
import dev.dbatrainer.app.game.domain.WorkdayState
import dev.dbatrainer.app.game.domain.WorldRank
import dev.dbatrainer.app.game.domain.WorldState
import dev.dbatrainer.app.game.domain.worldRankForXp

object WorkdayEngine {
    const val SHIFT_START_MINUTE = 9 * 60
    const val SHIFT_END_MINUTE = 18 * 60
    const val DAILY_TASK_COUNT = 2

    fun applyTimeTransitions(previous: WorldState, advanced: WorldState): WorldState {
        var result = advanced

        if (advanced.clock.day != previous.clock.day) {
            val report = buildDailyReport(previous)
            result = result.copy(
                workday = WorkdayState(day = advanced.clock.day),
                lastShiftReport = report,
            )
        } else if (result.workday.day != result.clock.day) {
            result = result.copy(workday = WorkdayState(day = result.clock.day))
        }

        result = applyMonthlyTransitions(result)
        return result
    }

    fun completeTask(world: WorldState, task: DailyTaskKind): WorldState {
        val normalized = ensureCurrentDay(world)
        if (task in normalized.workday.completedTasks) {
            return normalized
        }
        return normalized.copy(
            workday = normalized.workday.copy(
                completedTasks = normalized.workday.completedTasks + task,
            ),
        )
    }

    fun recordIncidentResolution(world: WorldState): WorldState {
        val normalized = ensureCurrentDay(world)
        return normalized.copy(
            workday = normalized.workday.copy(
                incidentsResolved = normalized.workday.incidentsResolved + 1,
            ),
        )
    }

    fun closeShift(world: WorldState): WorldState {
        val normalized = ensureCurrentDay(world)
        if (!canCloseShift(normalized) || normalized.workday.shiftClosed) {
            return normalized
        }
        return normalized.copy(
            workday = normalized.workday.copy(shiftClosed = true),
            lastShiftReport = buildDailyReport(normalized),
        )
    }

    fun canCloseShift(world: WorldState): Boolean =
        world.clock.minuteOfDay >= SHIFT_END_MINUTE && !world.workday.shiftClosed

    fun shiftLabelRu(world: WorldState): String = when {
        world.workday.shiftClosed -> "Смена закрыта"
        world.clock.minuteOfDay < SHIFT_START_MINUTE -> "До начала смены"
        world.clock.minuteOfDay < SHIFT_END_MINUTE -> "Рабочая смена"
        else -> "Смена завершена — можно закрыть отчёт"
    }

    fun shiftLabelEn(world: WorldState): String = when {
        world.workday.shiftClosed -> "Shift closed"
        world.clock.minuteOfDay < SHIFT_START_MINUTE -> "Before shift"
        world.clock.minuteOfDay < SHIFT_END_MINUTE -> "On shift"
        else -> "Shift ended — report can be closed"
    }

    fun salaryFor(rank: WorldRank): Int = when (rank) {
        WorldRank.TRAINEE_OPERATOR -> 1_200
        WorldRank.JUNIOR_OPERATOR -> 1_600
        WorldRank.DB_ENGINEER -> 2_300
        WorldRank.PRODUCTION_DBA -> 3_500
        WorldRank.SENIOR_DBA -> 5_000
        WorldRank.INCIDENT_COMMANDER -> 7_500
        WorldRank.INFRASTRUCTURE_ARCHITECT -> 10_000
    }

    private fun ensureCurrentDay(world: WorldState): WorldState = if (world.workday.day == world.clock.day) {
        world
    } else {
        world.copy(workday = WorkdayState(day = world.clock.day))
    }

    private fun buildDailyReport(world: WorldState): DailyShiftReport = DailyShiftReport(
        day = world.workday.day,
        tasksCompleted = world.workday.completedTasks.size,
        tasksTotal = DAILY_TASK_COUNT,
        incidentsResolved = world.workday.incidentsResolved,
        companyBalance = world.company.balance,
        playerCredits = world.player.credits,
    )

    private fun applyMonthlyTransitions(world: WorldState): WorldState {
        val currentPeriod = periodKey(world.clock.date.year, world.clock.date.month)
        if (currentPeriod <= world.lastSalaryPeriodKey) {
            return world
        }

        var result = world
        var period = world.lastSalaryPeriodKey + 1
        while (period <= currentPeriod) {
            val year = (period - 1) / 12
            val month = (period - 1) % 12 + 1
            val salary = salaryFor(result.player.worldRank)
            val revenue = 18_000L + result.completedIncidents.size * 1_000L
            val infrastructureCosts = 4_500L
            val payrollCosts = companyPayrollCost(result.player.worldRank)
            val incidentLosses = if (
                result.incident != null || result.nodes.any { it.status == InfrastructureStatus.DOWN }
            ) {
                2_000L
            } else {
                0L
            }
            val net = revenue - infrastructureCosts - payrollCosts - incidentLosses
            val closingBalance = result.company.balance + net
            val updatedXpRank = worldRankForXp(result.player.xp)

            result = result.copy(
                company = result.company.copy(balance = closingBalance),
                player = result.player.copy(
                    credits = result.player.credits + salary,
                    worldRank = updatedXpRank,
                ),
                lastMonthlyReport = MonthlyCompanyReport(
                    year = year,
                    month = month,
                    revenue = revenue,
                    infrastructureCosts = infrastructureCosts,
                    payrollCosts = payrollCosts,
                    incidentLosses = incidentLosses,
                    netChange = net,
                    playerSalaryCredits = salary,
                    closingBalance = closingBalance,
                ),
                lastSalaryPeriodKey = period,
            )
            period += 1
        }
        return result
    }

    private fun companyPayrollCost(rank: WorldRank): Long = when (rank) {
        WorldRank.TRAINEE_OPERATOR -> 2_500L
        WorldRank.JUNIOR_OPERATOR -> 3_200L
        WorldRank.DB_ENGINEER -> 4_600L
        WorldRank.PRODUCTION_DBA -> 6_000L
        WorldRank.SENIOR_DBA -> 8_000L
        WorldRank.INCIDENT_COMMANDER -> 11_000L
        WorldRank.INFRASTRUCTURE_ARCHITECT -> 15_000L
    }

    fun periodKey(year: Int, month: Int): Int = year * 12 + month
}
