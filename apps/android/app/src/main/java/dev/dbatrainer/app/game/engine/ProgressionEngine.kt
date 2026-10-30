package dev.dbatrainer.app.game.engine

import dev.dbatrainer.app.game.domain.DISK_FULL_COMPANY_REWARD
import dev.dbatrainer.app.game.domain.DISK_FULL_PLAYER_CREDITS_REWARD
import dev.dbatrainer.app.game.domain.DISK_FULL_XP_REWARD
import dev.dbatrainer.app.game.domain.HIGH_CPU_COMPANY_REWARD
import dev.dbatrainer.app.game.domain.HIGH_CPU_PLAYER_CREDITS_REWARD
import dev.dbatrainer.app.game.domain.HIGH_CPU_XP_REWARD
import dev.dbatrainer.app.game.domain.IncidentKind
import dev.dbatrainer.app.game.domain.IncidentStatus
import dev.dbatrainer.app.game.domain.SERVICE_DOWN_COMPANY_REWARD
import dev.dbatrainer.app.game.domain.SERVICE_DOWN_PLAYER_CREDITS_REWARD
import dev.dbatrainer.app.game.domain.SERVICE_DOWN_XP_REWARD
import dev.dbatrainer.app.game.domain.WorldState
import dev.dbatrainer.app.game.domain.worldRankForXp

data class IncidentReward(
    val companyBalance: Long,
    val playerCredits: Int,
    val xp: Int,
)

object ProgressionEngine {
    fun rewardFor(kind: IncidentKind): IncidentReward = when (kind) {
        IncidentKind.DISK_FULL -> IncidentReward(
            companyBalance = DISK_FULL_COMPANY_REWARD,
            playerCredits = DISK_FULL_PLAYER_CREDITS_REWARD,
            xp = DISK_FULL_XP_REWARD,
        )

        IncidentKind.SERVICE_DOWN -> IncidentReward(
            companyBalance = SERVICE_DOWN_COMPANY_REWARD,
            playerCredits = SERVICE_DOWN_PLAYER_CREDITS_REWARD,
            xp = SERVICE_DOWN_XP_REWARD,
        )

        IncidentKind.HIGH_CPU_BAD_QUERY -> IncidentReward(
            companyBalance = HIGH_CPU_COMPANY_REWARD,
            playerCredits = HIGH_CPU_PLAYER_CREDITS_REWARD,
            xp = HIGH_CPU_XP_REWARD,
        )
    }

    fun applyIncidentResolution(
        previous: WorldState,
        updated: WorldState,
    ): WorldState {
        val previousIncident = previous.incident ?: return updated
        val updatedIncident = updated.incident ?: return updated

        if (
            previousIncident.id != updatedIncident.id ||
            previousIncident.status != IncidentStatus.ACTIVE ||
            updatedIncident.status != IncidentStatus.RESOLVED
        ) {
            return updated
        }

        val reward = rewardFor(updatedIncident.kind)
        val updatedXp = updated.player.xp + reward.xp

        return updated.copy(
            company = updated.company.copy(
                balance = updated.company.balance + reward.companyBalance,
            ),
            player = updated.player.copy(
                credits = updated.player.credits + reward.playerCredits,
                xp = updatedXp,
                worldRank = worldRankForXp(updatedXp),
            ),
        )
    }
}
