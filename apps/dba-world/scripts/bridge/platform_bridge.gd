extends Node

## Cross-platform boundary for DBA World.
##
## Scenes only consume dictionaries/strings from this singleton. Android uses
## a runtime GodotPlugin today; iOS can expose the same contract later without
## changing scenes, gameplay UI, models or materials.

signal bootstrap_changed(payload: Dictionary)
signal exit_requested

const DEFAULT_WORLD_YEAR := 2045
const DEFAULT_WORLD_MONTH := 1
const DEFAULT_WORLD_DAY := 1
const HOST_PLUGIN_NAME := "DbaWorldHost"

var _bootstrap: Dictionary = {}


func platform_name() -> String:
    return OS.get_name()


func set_bootstrap(payload: Dictionary) -> void:
    _bootstrap = payload.duplicate(true)
    bootstrap_changed.emit(_bootstrap.duplicate(true))


func bootstrap() -> Dictionary:
    return _bootstrap.duplicate(true)


func world_environment() -> Dictionary:
    if Engine.has_singleton(HOST_PLUGIN_NAME):
        var host = Engine.get_singleton(HOST_PLUGIN_NAME)
        var raw = host.call("getWorldEnvironmentJson")
        if raw is String and not raw.is_empty():
            var parsed = JSON.parse_string(raw)
            if parsed is Dictionary and not parsed.is_empty():
                return parsed

    return _fallback_environment()


func request_exit() -> void:
    exit_requested.emit()
    get_tree().quit()


func _fallback_environment() -> Dictionary:
    var now := Time.get_datetime_dict_from_system()
    var hour := int(now.get("hour", 12))
    return {
        "year": DEFAULT_WORLD_YEAR,
        "month": DEFAULT_WORLD_MONTH,
        "day": DEFAULT_WORLD_DAY,
        "world_day": 1,
        "season": "winter",
        "day_phase": _phase_for_hour(hour),
        "weather": "clear",
        "new_year_decorations": true,
    }


func _phase_for_hour(hour: int) -> String:
    if hour >= 6 and hour <= 9:
        return "morning"
    if hour >= 10 and hour <= 17:
        return "day"
    if hour >= 18 and hour <= 21:
        return "evening"
    return "night"
