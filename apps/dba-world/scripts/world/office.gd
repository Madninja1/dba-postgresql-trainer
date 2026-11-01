extends "res://scripts/world/world_scene.gd"

const OUTDOOR_SCENE := "res://scenes/world/outdoor.tscn"
const SERVER_ROOM_SCENE := "res://scenes/world/server_room.tscn"

var _exit_door: Node3D
var _server_room_door: Node3D
var _workstation: Node3D


func _scene_title() -> String:
    return "DBA World Labs · Office"


func _scene_objective() -> String:
    return "Inspect your workstation, then enter the server room"


func _player_spawn() -> Vector3:
    return Vector3(-3.5, 1.0, 4.2)


func _background_color() -> Color:
    return Color("#29343d")


func _build_world() -> void:
    add_box("OfficeFloor", Vector3(18.0, 0.25, 13.0), Vector3(0.0, -0.15, 0.0), Color("#353b40"), true)
    add_box("BackWall", Vector3(18.0, 4.5, 0.3), Vector3(0.0, 2.25, -6.35), Color("#545d63"), true)
    add_box("LeftWall", Vector3(0.3, 4.5, 13.0), Vector3(-8.85, 2.25, 0.0), Color("#4b545b"), true)
    add_box("RightWall", Vector3(0.3, 4.5, 13.0), Vector3(8.85, 2.25, 0.0), Color("#4b545b"), true)
    add_box("FrontWallLeft", Vector3(7.0, 4.5, 0.3), Vector3(-5.5, 2.25, 6.35), Color("#505960"), true)
    add_box("FrontWallRight", Vector3(7.0, 4.5, 0.3), Vector3(5.5, 2.25, 6.35), Color("#505960"), true)

    _build_desk(Vector3(-1.3, 0.0, -0.5))
    _build_noc_wall(Vector3(2.8, 2.1, -6.0))

    add_box("OutdoorDoor", Vector3(2.3, 3.2, 0.18), Vector3(0.0, 1.6, 6.12), Color("#26343e"), false)
    add_label_3d("OUTSIDE", Vector3(0.0, 2.8, 5.95), Color("#cde9f6"), 28)
    _exit_door = add_marker("OutdoorExit", Vector3(0.0, 1.0, 5.2))

    add_box("ServerRoomDoor", Vector3(2.2, 3.2, 0.18), Vector3(6.2, 1.6, -6.12), Color("#1e272e"), false)
    add_label_3d("SERVER ROOM", Vector3(6.2, 2.85, -5.96), Color("#ffcf7a"), 28)
    _server_room_door = add_marker("ServerRoomDoorMarker", Vector3(6.2, 1.0, -5.2))

    _workstation = add_marker("EngineerWorkstation", Vector3(-1.3, 1.0, 0.6))

    add_box("Sofa", Vector3(3.0, 0.8, 1.2), Vector3(-5.8, 0.4, -3.6), Color("#263d4c"), true)
    add_box("CoffeeTable", Vector3(1.8, 0.45, 1.0), Vector3(-5.8, 0.3, -1.8), Color("#6e5039"), true)


func _register_world_interactions() -> void:
    register_interaction(
        _exit_door,
        "Go outside",
        func(): change_world(OUTDOOR_SCENE),
        2.5
    )
    register_interaction(
        _server_room_door,
        "Enter server room",
        func(): change_world(SERVER_ROOM_SCENE),
        2.5
    )
    register_interaction(
        _workstation,
        "Inspect engineering workstation",
        func(): show_message("Workstation online · Terminal / Monitoring integration comes next", 4.0),
        2.2
    )


func _build_desk(origin: Vector3) -> void:
    add_box("DeskTop", Vector3(4.4, 0.22, 1.7), origin + Vector3(0.0, 1.0, 0.0), Color("#735239"), true)
    add_box("DeskLegL", Vector3(0.22, 1.0, 1.45), origin + Vector3(-1.8, 0.5, 0.0), Color("#3d3027"), true)
    add_box("DeskLegR", Vector3(0.22, 1.0, 1.45), origin + Vector3(1.8, 0.5, 0.0), Color("#3d3027"), true)

    for x in [-0.9, 0.9]:
        add_box("Monitor", Vector3(1.45, 0.9, 0.12), origin + Vector3(x, 1.85, -0.35), Color("#111820"), false)
        var screen := add_box("MonitorGlow", Vector3(1.25, 0.7, 0.04), origin + Vector3(x, 1.85, -0.27), Color("#235673"), false)
        screen.material_override = material(Color("#245b76"), 0.0, Color("#1d5e7e"))
    add_box("Keyboard", Vector3(1.6, 0.08, 0.55), origin + Vector3(0.0, 1.16, 0.42), Color("#24282c"), false)
    add_cylinder("Coffee", 0.18, 0.3, origin + Vector3(1.6, 1.25, 0.3), Color("#d7d7d7"))


func _build_noc_wall(origin: Vector3) -> void:
    add_box("NocPanel", Vector3(4.6, 2.4, 0.12), origin, Color("#12191f"), false)
    var healthy := add_box("NocHealthy", Vector3(1.1, 0.18, 0.05), origin + Vector3(-1.35, 0.55, 0.09), Color("#3fa66a"), false)
    healthy.material_override = material(Color("#3fa66a"), 0.0, Color("#1e6b3a"))
    var warning := add_box("NocWarning", Vector3(0.8, 0.18, 0.05), origin + Vector3(0.1, 0.1, 0.09), Color("#d5a938"), false)
    warning.material_override = material(Color("#d5a938"), 0.0, Color("#7c5b11"))
    add_label_3d("MONITORING", origin + Vector3(0.0, 0.88, 0.1), Color("#bfe9ff"), 26)
