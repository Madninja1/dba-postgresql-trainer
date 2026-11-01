package dev.dbatrainer.app.game3d

import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberUpdatedState
import androidx.compose.ui.platform.LocalContext

internal object DbaWorldReturnGate {
    private const val PREFERENCES_NAME = "dba_trainer_settings"
    const val FORCE_MODE_SELECTION_KEY = "dba_world_force_mode_selection_once"

    fun mark(context: android.content.Context) {
        context.getSharedPreferences(PREFERENCES_NAME, android.content.Context.MODE_PRIVATE)
            .edit()
            .putBoolean(FORCE_MODE_SELECTION_KEY, true)
            .commit()
    }

    fun clear(context: android.content.Context) {
        context.getSharedPreferences(PREFERENCES_NAME, android.content.Context.MODE_PRIVATE)
            .edit()
            .remove(FORCE_MODE_SELECTION_KEY)
            .apply()
    }
}

/**
 * Platform-facing launch handle used by the Compose shell.
 *
 * Trainer UI only depends on this tiny contract. Android-specific Activity,
 * Intent and ActivityResult APIs stay in this file so a future iOS host can
 * implement the same behavior without changing the learning UI or Godot world.
 */
class DbaWorldLaunchHandle internal constructor(
    private val launchAction: () -> Unit,
    private val runningState: State<Boolean>,
) {
    val isRunning: Boolean
        get() = runningState.value

    fun launch() = launchAction()
}

@Composable
fun rememberDbaWorldLauncher(
    onReturned: () -> Unit,
): DbaWorldLaunchHandle {
    val context = LocalContext.current
    val runningState = remember { mutableStateOf(false) }
    val currentOnReturned by rememberUpdatedState(onReturned)
    val activityLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.StartActivityForResult(),
    ) {
        runningState.value = false
        DbaWorldReturnGate.clear(context)
        currentOnReturned()
    }

    return remember(activityLauncher, context) {
        DbaWorldLaunchHandle(
            launchAction = {
                if (!runningState.value) {
                    runningState.value = true
                    DbaWorldReturnGate.mark(context)
                    activityLauncher.launch(DbaWorldGodotActivity.createIntent(context))
                }
            },
            runningState = runningState,
        )
    }
}
