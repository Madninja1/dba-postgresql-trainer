package dev.dbatrainer.app.game3d

import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import dev.dbatrainer.app.game.DbaWorldController
import dev.dbatrainer.app.game.engine.WorldEnvironmentEngine
import org.godotengine.godot.Godot
import org.godotengine.godot.GodotActivity
import org.godotengine.godot.plugin.GodotPlugin
import org.json.JSONObject
import java.time.LocalTime

/**
 * Android host adapter for the shared DBA World Godot project.
 *
 * Keep platform lifecycle and launch details here. Godot scenes must remain
 * platform-neutral so the same project can be hosted by iOS later. The
 * existing Kotlin simulation remains the source of truth for persisted world
 * time; this adapter advances that clock while the game is foregrounded.
 */
class DbaWorldGodotActivity : GodotActivity() {
    private lateinit var worldController: DbaWorldController
    @Volatile
    private var environmentSnapshotJson: String = "{}"
    private var hostPlugin: DbaWorldHostPlugin? = null

    private val activeTimeHandler = Handler(Looper.getMainLooper())
    private val activeTimeTicker = object : Runnable {
        override fun run() {
            if (::worldController.isInitialized) {
                worldController.syncActiveTime()
                refreshEnvironmentSnapshot()
                activeTimeHandler.postDelayed(this, ACTIVE_TIME_TICK_MILLIS)
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        // Configure edge-to-edge before Godot creates its rendering Surface.
        // Changing decor fitting after super.onCreate() can resize an already
        // initialized embedded Godot surface and leave it rendering only the
        // clear/background color on some devices.
        WindowCompat.setDecorFitsSystemWindows(window, false)

        worldController = DbaWorldController(applicationContext)
        if (worldController.canContinue) {
            worldController.continueGame()
        } else {
            worldController.newGame()
        }
        refreshEnvironmentSnapshot()
        super.onCreate(savedInstanceState)
        hideSystemBars()
    }

    override fun onResume() {
        super.onResume()
        hideSystemBars()
        worldController.onAppResumed()
        refreshEnvironmentSnapshot()
        activeTimeHandler.removeCallbacks(activeTimeTicker)
        activeTimeHandler.postDelayed(activeTimeTicker, ACTIVE_TIME_TICK_MILLIS)
    }

    override fun onPause() {
        activeTimeHandler.removeCallbacks(activeTimeTicker)
        worldController.onAppPaused()
        super.onPause()
    }

    override fun getHostPlugins(godot: Godot): Set<GodotPlugin> {
        val plugin = hostPlugin ?: DbaWorldHostPlugin(godot) { environmentSnapshotJson }
            .also { hostPlugin = it }
        return super.getHostPlugins(godot) + plugin
    }

    private fun refreshEnvironmentSnapshot() {
        val current = worldController.world ?: return
        val date = current.clock.date
        val environment = WorldEnvironmentEngine.environmentFor(
            clock = current.clock,
            visualHour = LocalTime.now().hour,
        )
        environmentSnapshotJson = JSONObject()
            .put("year", date.year)
            .put("month", date.month)
            .put("day", date.dayOfMonth)
            .put("world_day", current.clock.day)
            .put("season", environment.season.name.lowercase())
            .put("day_phase", environment.dayPhase.name.lowercase())
            .put("weather", environment.weather.name.lowercase())
            .put("new_year_decorations", environment.newYearDecorations)
            .toString()
    }

    private fun hideSystemBars() {
        WindowCompat.getInsetsController(window, window.decorView).apply {
            hide(WindowInsetsCompat.Type.systemBars())
            systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
        }
    }

    companion object {
        private const val ACTIVE_TIME_TICK_MILLIS = 1_000L

        fun createIntent(context: Context): Intent =
            Intent(context, DbaWorldGodotActivity::class.java)
    }
}
