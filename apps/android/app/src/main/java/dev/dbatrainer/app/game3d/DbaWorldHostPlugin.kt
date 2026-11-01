package dev.dbatrainer.app.game3d

import org.godotengine.godot.Godot
import org.godotengine.godot.plugin.GodotPlugin
import org.godotengine.godot.plugin.UsedByGodot

/**
 * Tiny runtime bridge exposed to the shared Godot project.
 *
 * Keep the contract JSON/string based so iOS can provide the same payload later
 * without coupling gameplay scenes to Android APIs or Kotlin classes.
 */
class DbaWorldHostPlugin(
    godot: Godot,
    private val environmentJsonProvider: () -> String,
) : GodotPlugin(godot) {
    override fun getPluginName() = PLUGIN_NAME

    @UsedByGodot
    fun getWorldEnvironmentJson(): String = environmentJsonProvider()

    companion object {
        const val PLUGIN_NAME = "DbaWorldHost"
    }
}
