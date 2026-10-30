package dev.dbatrainer.app

import android.os.Bundle
import android.os.Handler
import android.os.Looper
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import dev.dbatrainer.app.ui.TrainerApp
import dev.dbatrainer.app.ui.theme.DBATrainerTheme

class MainActivity : ComponentActivity() {
    private lateinit var controller: TrainerController
    private val activeTimeHandler = Handler(Looper.getMainLooper())
    private val activeTimeTicker = object : Runnable {
        override fun run() {
            if (::controller.isInitialized) {
                controller.dbaWorld.syncActiveTime()
                activeTimeHandler.postDelayed(this, ACTIVE_TIME_TICK_MILLIS)
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        controller = TrainerController(applicationContext)

        setContent {
            DBATrainerTheme {
                TrainerApp(controller)
            }
        }
    }

    override fun onResume() {
        super.onResume()
        controller.dbaWorld.onAppResumed()
        activeTimeHandler.removeCallbacks(activeTimeTicker)
        activeTimeHandler.postDelayed(activeTimeTicker, ACTIVE_TIME_TICK_MILLIS)
    }

    override fun onPause() {
        activeTimeHandler.removeCallbacks(activeTimeTicker)
        controller.dbaWorld.onAppPaused()
        super.onPause()
    }

    private companion object {
        const val ACTIVE_TIME_TICK_MILLIS = 1_000L
    }
}
