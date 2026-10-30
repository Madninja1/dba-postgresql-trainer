package dev.dbatrainer.app.game.engine

import dev.dbatrainer.app.game.domain.REAL_MILLIS_PER_GAME_MINUTE
import dev.dbatrainer.app.game.domain.WorldState

object WorldClockEngine {
    /**
     * Advances DBA World only by time when the Android app is actually active.
     * Five real minutes equal one game hour, so one game minute is five real seconds.
     */
    fun advanceByActiveTime(world: WorldState, elapsedRealMillis: Long): WorldState {
        if (elapsedRealMillis <= 0L) {
            return world
        }

        val accumulated = world.activeTimeRemainderMillis + elapsedRealMillis
        val gameMinutes = (accumulated / REAL_MILLIS_PER_GAME_MINUTE).toInt()
        val remainder = accumulated % REAL_MILLIS_PER_GAME_MINUTE
        return world.copy(
            clock = if (gameMinutes > 0) world.clock.advance(gameMinutes) else world.clock,
            activeTimeRemainderMillis = remainder,
            totalActiveRealMillis = world.totalActiveRealMillis + elapsedRealMillis,
        )
    }
}
