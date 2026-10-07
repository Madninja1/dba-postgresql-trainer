package dev.dbatrainer.app

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import dev.dbatrainer.app.ui.TrainerApp
import dev.dbatrainer.app.ui.theme.DBATrainerTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        val controller = TrainerController(applicationContext)

        setContent {
            DBATrainerTheme {
                TrainerApp(controller)
            }
        }
    }
}
