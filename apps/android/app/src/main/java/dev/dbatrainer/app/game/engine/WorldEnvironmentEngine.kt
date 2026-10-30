package dev.dbatrainer.app.game.engine

import dev.dbatrainer.app.game.domain.WorldClock

enum class WorldSeason {
    WINTER,
    SPRING,
    SUMMER,
    AUTUMN,
}

enum class WorldDayPhase {
    MORNING,
    DAY,
    EVENING,
    NIGHT,
}

enum class WorldWeather {
    CLEAR,
    CLOUDY,
    RAIN,
    SNOW,
}

data class WorldEnvironment(
    val season: WorldSeason,
    val dayPhase: WorldDayPhase,
    val weather: WorldWeather,
    val newYearDecorations: Boolean,
)

object WorldEnvironmentEngine {
    fun environmentFor(clock: WorldClock): WorldEnvironment {
        val date = clock.date
        val season = when (date.month) {
            12, 1, 2 -> WorldSeason.WINTER
            3, 4, 5 -> WorldSeason.SPRING
            6, 7, 8 -> WorldSeason.SUMMER
            else -> WorldSeason.AUTUMN
        }
        val dayPhase = when (clock.hour) {
            in 6..9 -> WorldDayPhase.MORNING
            in 10..17 -> WorldDayPhase.DAY
            in 18..21 -> WorldDayPhase.EVENING
            else -> WorldDayPhase.NIGHT
        }
        val roll = deterministicRoll(
            year = date.year,
            month = date.month,
            day = date.dayOfMonth,
        )
        val weather = when (season) {
            WorldSeason.WINTER -> when {
                roll < 52 -> WorldWeather.SNOW
                roll < 78 -> WorldWeather.CLOUDY
                else -> WorldWeather.CLEAR
            }

            WorldSeason.SPRING -> when {
                roll < 38 -> WorldWeather.RAIN
                roll < 68 -> WorldWeather.CLOUDY
                else -> WorldWeather.CLEAR
            }

            WorldSeason.SUMMER -> when {
                roll < 16 -> WorldWeather.RAIN
                roll < 30 -> WorldWeather.CLOUDY
                else -> WorldWeather.CLEAR
            }

            WorldSeason.AUTUMN -> when {
                roll < 52 -> WorldWeather.RAIN
                roll < 82 -> WorldWeather.CLOUDY
                else -> WorldWeather.CLEAR
            }
        }
        val newYearDecorations = (date.month == 12 && date.dayOfMonth >= 20) ||
            (date.month == 1 && date.dayOfMonth <= 10)
        return WorldEnvironment(
            season = season,
            dayPhase = dayPhase,
            weather = weather,
            newYearDecorations = newYearDecorations,
        )
    }

    private fun deterministicRoll(year: Int, month: Int, day: Int): Int {
        val seed = year * 372 + month * 31 + day * 17
        return ((seed xor (seed ushr 5)) and Int.MAX_VALUE) % 100
    }
}
