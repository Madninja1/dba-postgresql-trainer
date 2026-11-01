extends "res://scripts/world/world_scene.gd"

const OFFICE_SCENE := "res://scenes/world/office.tscn"

var _office_door: Node3D
var _db_primary: Node3D
var _switch: Node3D


func _scene_title() -> String:
    return "DBA World Labs · Server Room"


func _scene_objective() -> String:
    return "Locate db-primary and inspect the rack"


func _player_spawn() -> Vector3:
    return Vector3(0.0, 1.0, 4.6)


func _background_color() -> Color:
    return Color("#111820")


func _build_world() -> void:
    add_box("ServerFloor", Vector3(16.0, 0.25, 13.0), Vector3(0.0, -0.15, 0.0), Color("#252a2f"), true)
    add_box("BackWall", Vector3(16.0, 4.5, 0.3), Vector3(0.0, 2.25, -6.35), Color("#353d43"), true)
    add_box("LeftWall", Vector3(0.3, 4.5, 13.0), Vector3(-7.85, 2.25, 0.0), Color("#32393f"), true)
    add_box("RightWall", Vector3(0.3, 4.5, 13.0), Vector3(7.85, 2.25, 0.0), Color("#32393f"), true)
    add_box("FrontWallLeft", Vector3(6.6, 4.5, 0.3), Vector3(-4.7, 2.25, 6.35), Color("#353d43"), true)
    add_box("FrontWallRight", Vector3(6.6, 4.5, 0.3), Vector3(4.7, 2.25, 6.35), Color("#353d43"), true)

    _build_rack("RACK A", Vector3(-2.4, 0.0, -1.8), true)
    _build_rack("RACK B", Vector3(2.4, 0.0, -1.8), false)

    add_box("OfficeDoor", Vector3(2.1, 3.2, 0.18), Vector3(0.0, 1.6, 6.12), Color("#20272d"), false)
    add_label_3d("OFFICE", Vector3(0.0, 2.85, 5.95), Color("#d5edf7"), 28)
    _office_door = add_marker("OfficeDoorMarker", Vector3(0.0, 1.0, 5.2))

    _switch = add_marker("SwitchMarker", Vector3(5.8, 1.0, -4.8))
    add_box("NetworkSwitch", Vector3(2.2, 0.45, 0.8), Vector3(5.8, 1.05, -5.4), Color("#273841"), true)
    add_label_3d("SW-01", Vector3(5.8, 1.65, -5.0), Color("#9fdcff"), 24)

    add_box("UPS", Vector3(1.5, 2.2, 1.2), Vector3(-6.1, 1.1, -4.8), Color("#292e33"), true)
    add_label_3d("UPS-01", Vector3(-6.1, 2.55, -4.2), Color("#b9c6cf"), 22)

    for x in [-5.5, -1.8, 1.8, 5.5]:
        add_omni_light(Vector3(x, 3.8, 0.0), Color("#dff4ff"), 0.75, 6.5)


func _register_world_interactions() -> void:
    register_interaction(
        _office_door,
        "Return to office",
        func(): change_world(OFFICE_SCENE),
        2.4
    )
    register_interaction(
        _db_primary,
        "Inspect db-primary",
        func(): show_message("db-primary · PostgreSQL · WARNING · CPU 84% · Disk 91%", 5.0),
        2.3
    )
    register_interaction(
        _switch,
        "Inspect SW-01",
        func(): show_message("SW-01 · network uplink healthy", 3.0),
        2.2
    )


func _build_rack(title: String, origin: Vector3, populated: bool) -> void:
    add_box("%sFrame" % title, Vector3(2.2, 4.2, 1.4), origin + Vector3(0.0, 2.1, 0.0), Color("#161b1f"), true)
    add_label_3d(title, origin + Vector3(0.0, 4.55, 0.7), Color("#cfe9f4"), 28)

    for slot in range(6):
        var y := 0.55 + slot * 0.57
        var color := Color("#343b42")
        var server_name := "FREE"
        if populated:
            match slot:
                5:
                    server_name = "app-01"
                    color = Color("#31566b")
                4:
                    server_name = "app-02"
                    color = Color("#31566b")
                3:
                    server_name = "db-primary"
                    color = Color("#6d4a36")
                2:
                    server_name = "db-replica"
                    color = Color("#3b5865")
        var server := add_box(
            "%s_%s" % [title, slot],
            Vector3(1.82, 0.42, 1.5),
            origin + Vector3(0.0, y, 0.04),
            color,
            false
        )
        if server_name != "FREE":
            add_label_3d(server_name, origin + Vector3(0.0, y + 0.05, 0.82), Color("#e6f4f9"), 19)
            _add_server_leds(origin + Vector3(0.0, y, 0.83), server_name == "db-primary")
        if server_name == "db-primary":
            _db_primary = add_marker("DbPrimaryMarker", origin + Vector3(0.0, y, 1.3))
            add_label_3d("!", origin + Vector3(0.72, y + 0.42, 0.92), Color("#ff6f61"), 34)


func _add_server_leds(origin: Vector3, warning: bool) -> void:
    var colors := [Color("#53d681"), Color("#53d681"), Color("#53d681")]
    if warning:
        colors[1] = Color("#ffbf47")
        colors[2] = Color("#ff6f61")
    for i in range(3):
        var led := add_box(
            "Led",
            Vector3(0.08, 0.08, 0.03),
            origin + Vector3(-0.72 + i * 0.12, 0.0, 0.0),
            colors[i],
            false
        )
        led.material_override = material(colors[i], 0.0, colors[i])
