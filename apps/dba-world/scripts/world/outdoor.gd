extends "res://scripts/world/world_scene.gd"

const OFFICE_SCENE := "res://scenes/world/office.tscn"

var _entrance: Node3D
var _outdoor_environment: Environment
var _sun: DirectionalLight3D
var _ground: MeshInstance3D
var _road: MeshInstance3D
var _walkway: MeshInstance3D
var _window_meshes: Array[MeshInstance3D] = []
var _lamp_lights: Array[OmniLight3D] = []
var _tree_crowns: Array[MeshInstance3D] = []
var _weather_nodes: Array[MeshInstance3D] = []
var _holiday_nodes: Array[Node3D] = []
var _weather_rng := RandomNumberGenerator.new()
var _active_weather := ""


func _scene_title() -> String:
    return "DBA World · Campus"


func _scene_objective() -> String:
    return "Reach the main entrance of DBA World Labs"


func _player_spawn() -> Vector3:
    return Vector3(0.0, 1.0, 8.5)


func _build_environment() -> void:
    var world_environment := WorldEnvironment.new()
    world_environment.name = "OutdoorEnvironment"
    _outdoor_environment = Environment.new()
    _outdoor_environment.background_mode = Environment.BG_COLOR
    _outdoor_environment.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
    world_environment.environment = _outdoor_environment
    add_child(world_environment)

    _sun = DirectionalLight3D.new()
    _sun.name = "SunMoonLight"
    _sun.shadow_enabled = true
    add_child(_sun)

    _apply_atmosphere(environment_snapshot())


func _build_world() -> void:
    var profile := environment_snapshot()
    _ground = add_box(
        "Ground",
        Vector3(48.0, 0.25, 48.0),
        Vector3(0.0, -0.15, 0.0),
        _ground_color(str(profile.get("season", "winter"))),
        true
    )
    _road = add_box("Road", Vector3(11.0, 0.10, 48.0), Vector3(0.0, 0.02, 7.0), Color("#30343a"), false)
    _walkway = add_box("Walkway", Vector3(4.8, 0.08, 15.0), Vector3(0.0, 0.07, -7.0), Color("#aeb3b7"), false)

    # Road detail and parking bays make the first view read as a place rather
    # than a collection of primitive boxes, while remaining cheap greybox art.
    add_box("RoadCenterLine", Vector3(0.12, 0.025, 42.0), Vector3(0.0, 0.09, 8.0), Color("#d8d3b8"), false)
    for side in [-1.0, 1.0]:
        add_box("Curb", Vector3(0.28, 0.20, 36.0), Vector3(side * 5.65, 0.14, 5.0), Color("#8d9398"), false)
        for z in [13.0, 17.0, 21.0]:
            add_box("ParkingMark", Vector3(2.6, 0.025, 0.10), Vector3(side * 3.9, 0.09, z), Color("#c9c9bd"), false)

    add_box("MainBuilding", Vector3(18.0, 7.0, 8.0), Vector3(0.0, 3.5, -16.0), Color("#68737d"), true)
    add_box("FacadeBand", Vector3(18.4, 0.55, 0.35), Vector3(0.0, 5.75, -11.82), Color("#283a46"), false)
    add_box("Roof", Vector3(19.0, 0.5, 9.0), Vector3(0.0, 7.25, -16.0), Color("#252b31"), false)
    add_box("EntranceCanopy", Vector3(4.2, 0.20, 1.8), Vector3(0.0, 3.65, -10.95), Color("#26343e"), false)
    add_box("EntranceFrame", Vector3(3.3, 3.8, 0.25), Vector3(0.0, 1.9, -11.88), Color("#303943"), false)
    add_box("GlassDoor", Vector3(2.5, 3.2, 0.16), Vector3(0.0, 1.6, -11.70), Color("#406278"), false)

    # Rooftop infrastructure hints at the technical character of the company.
    add_box("CoolingUnitA", Vector3(2.6, 1.0, 2.0), Vector3(-4.0, 7.85, -16.0), Color("#4e5961"), false)
    add_box("CoolingUnitB", Vector3(2.6, 1.0, 2.0), Vector3(4.0, 7.85, -16.0), Color("#4e5961"), false)

    for row in range(2):
        for column in range(5):
            var x := -6.0 + column * 3.0
            var y := 3.0 + row * 2.0
            var window := add_box(
                "Window_%s_%s" % [row, column],
                Vector3(1.25, 1.0, 0.18),
                Vector3(x, y, -11.74),
                Color("#83c4df"),
                false
            )
            _window_meshes.append(window)

    add_label_3d("DBA WORLD LABS", Vector3(0.0, 6.15, -11.53), Color("#d7f3ff"), 52)

    for i in range(5):
        _add_tree(Vector3(-11.0, 0.0, -7.0 - i * 4.0))
        _add_tree(Vector3(11.0, 0.0, -7.0 - i * 4.0))

    for side in [-1.0, 1.0]:
        for z in [-3.0, -10.0, -18.0]:
            _add_lamp(Vector3(side * 7.0, 0.0, z))

    _add_clouds()
    _build_holiday_decorations()

    _entrance = add_marker("MainEntrance", Vector3(0.0, 1.0, -10.6))
    add_label_3d("ENTER", Vector3(0.0, 2.5, -11.2), Color("#9fe7ff"), 32)


func _process(delta: float) -> void:
    super._process(delta)
    _update_precipitation(delta)


func _register_world_interactions() -> void:
    register_interaction(
        _entrance,
        "Enter DBA World Labs",
        func(): change_world(OFFICE_SCENE),
        2.8
    )


func _on_environment_snapshot_changed(snapshot: Dictionary) -> void:
    if _outdoor_environment == null:
        return
    _apply_atmosphere(snapshot)
    _apply_season_materials(snapshot)
    _apply_building_lights(snapshot)
    _set_weather(str(snapshot.get("weather", "clear")), int(snapshot.get("world_day", 1)))
    _set_holiday_decorations(bool(snapshot.get("new_year_decorations", false)))


func _apply_atmosphere(snapshot: Dictionary) -> void:
    if _outdoor_environment == null or _sun == null:
        return

    var phase := str(snapshot.get("day_phase", "day"))
    var weather := str(snapshot.get("weather", "clear"))
    var sky := Color("#78a8c6")
    var ambient := Color("#d7e8ef")
    var ambient_energy := 0.72
    var sun_color := Color("#fff1d0")
    var sun_energy := 1.05
    var sun_rotation := Vector3(-48.0, -28.0, 0.0)

    match phase:
        "morning":
            sky = Color("#d89672")
            ambient = Color("#e8c7ad")
            ambient_energy = 0.62
            sun_color = Color("#ffd1a2")
            sun_energy = 0.78
            sun_rotation = Vector3(-20.0, -70.0, 0.0)
        "evening":
            sky = Color("#8b6379")
            ambient = Color("#a38c9b")
            ambient_energy = 0.48
            sun_color = Color("#ffb179")
            sun_energy = 0.55
            sun_rotation = Vector3(-13.0, 64.0, 0.0)
        "night":
            sky = Color("#111b2b")
            ambient = Color("#6680a8")
            ambient_energy = 0.26
            sun_color = Color("#9fb8e8")
            sun_energy = 0.16
            sun_rotation = Vector3(-52.0, 145.0, 0.0)

    if weather == "cloudy":
        sky = sky.darkened(0.18)
        ambient_energy *= 0.82
        sun_energy *= 0.58
    elif weather == "rain":
        sky = sky.darkened(0.28)
        ambient = ambient.darkened(0.10)
        ambient_energy *= 0.74
        sun_energy *= 0.36
    elif weather == "snow":
        sky = sky.lightened(0.08)
        ambient_energy *= 0.92
        sun_energy *= 0.72

    _outdoor_environment.background_color = sky
    _outdoor_environment.ambient_light_color = ambient
    _outdoor_environment.ambient_light_energy = ambient_energy
    _sun.light_color = sun_color
    _sun.light_energy = sun_energy
    _sun.rotation_degrees = sun_rotation


func _apply_season_materials(snapshot: Dictionary) -> void:
    var season := str(snapshot.get("season", "winter"))
    var weather := str(snapshot.get("weather", "clear"))
    if _ground != null:
        _ground.material_override = material(_ground_color(season))
    if _walkway != null:
        var walkway_color := Color("#aeb3b7")
        if season == "winter":
            walkway_color = Color("#c9d0d1")
        elif weather == "rain":
            walkway_color = Color("#747b80")
        _walkway.material_override = material(walkway_color)
    if _road != null:
        var road_color := Color("#30343a")
        if weather == "rain":
            road_color = Color("#1e252b")
        elif season == "winter":
            road_color = Color("#41484d")
        _road.material_override = material(road_color, 0.04)

    var crown_color := _tree_color(season)
    for crown in _tree_crowns:
        if is_instance_valid(crown):
            crown.material_override = material(crown_color)


func _apply_building_lights(snapshot: Dictionary) -> void:
    var phase := str(snapshot.get("day_phase", "day"))
    var lights_on := phase == "evening" or phase == "night"
    var window_color := Color("#74a9ba")
    var emission := Color(0.0, 0.0, 0.0, 1.0)
    if lights_on:
        window_color = Color("#e6c884")
        emission = Color("#8a6327")

    for window in _window_meshes:
        if is_instance_valid(window):
            window.material_override = material(window_color, 0.04, emission)
    for lamp in _lamp_lights:
        if is_instance_valid(lamp):
            lamp.visible = lights_on


func _ground_color(season: String) -> Color:
    match season:
        "spring": return Color("#63835a")
        "summer": return Color("#4e7446")
        "autumn": return Color("#77664b")
        _: return Color("#d5dcdb")


func _tree_color(season: String) -> Color:
    match season:
        "spring": return Color("#70a967")
        "summer": return Color("#356a40")
        "autumn": return Color("#b36b35")
        _: return Color("#87918b")


func _add_tree(position: Vector3) -> void:
    add_cylinder("TreeTrunk", 0.2, 2.2, position + Vector3(0.0, 1.1, 0.0), Color("#624630"))
    var crown := MeshInstance3D.new()
    var mesh := SphereMesh.new()
    mesh.radius = 1.15
    mesh.height = 2.3
    crown.mesh = mesh
    crown.position = position + Vector3(0.0, 2.8, 0.0)
    crown.material_override = material(_tree_color(str(environment_snapshot().get("season", "winter"))))
    add_child(crown)
    _tree_crowns.append(crown)


func _add_lamp(position: Vector3) -> void:
    add_cylinder("LampPole", 0.09, 3.6, position + Vector3(0.0, 1.8, 0.0), Color("#252a30"))
    var light := add_omni_light(position + Vector3(0.0, 3.55, 0.0), Color("#ffe0a0"), 1.05, 6.5)
    _lamp_lights.append(light)


func _add_clouds() -> void:
    var cloud_color := Color("#b8c0c5")
    for i in range(6):
        var cloud := MeshInstance3D.new()
        cloud.name = "Cloud_%s" % i
        var mesh := SphereMesh.new()
        mesh.radius = 1.0
        mesh.height = 2.0
        cloud.mesh = mesh
        cloud.scale = Vector3(3.8 + float(i % 2), 0.55, 1.7)
        cloud.position = Vector3(-17.0 + i * 6.2, 13.0 + (i % 2) * 1.2, -22.0 + (i % 3) * 7.0)
        cloud.material_override = material(cloud_color)
        add_child(cloud)


func _build_holiday_decorations() -> void:
    var tree_root := Node3D.new()
    tree_root.name = "NewYearDecorations"
    tree_root.position = Vector3(-6.8, 0.0, -8.5)
    add_child(tree_root)
    _holiday_nodes.append(tree_root)

    var trunk := MeshInstance3D.new()
    var trunk_mesh := CylinderMesh.new()
    trunk_mesh.top_radius = 0.18
    trunk_mesh.bottom_radius = 0.22
    trunk_mesh.height = 1.3
    trunk.mesh = trunk_mesh
    trunk.position = Vector3(0.0, 0.65, 0.0)
    trunk.material_override = material(Color("#5b3f2c"))
    tree_root.add_child(trunk)

    for level in range(3):
        var crown := MeshInstance3D.new()
        var crown_mesh := CylinderMesh.new()
        crown_mesh.top_radius = 0.05
        crown_mesh.bottom_radius = 1.35 - level * 0.25
        crown_mesh.height = 1.7
        crown.mesh = crown_mesh
        crown.position = Vector3(0.0, 1.5 + level * 0.85, 0.0)
        crown.material_override = material(Color("#24583b"))
        tree_root.add_child(crown)

    var light_colors := [Color("#ff6f66"), Color("#76c9ff"), Color("#ffd66f")]
    for i in range(9):
        var bulb := MeshInstance3D.new()
        var bulb_mesh := SphereMesh.new()
        bulb_mesh.radius = 0.10
        bulb_mesh.height = 0.20
        bulb.mesh = bulb_mesh
        var angle := float(i) * TAU / 9.0
        bulb.position = Vector3(cos(angle) * 0.9, 1.7 + (i % 3) * 0.65, sin(angle) * 0.9)
        var color: Color = light_colors[i % light_colors.size()]
        bulb.material_override = material(color, 0.0, color)
        tree_root.add_child(bulb)


func _set_holiday_decorations(enabled: bool) -> void:
    for node in _holiday_nodes:
        if is_instance_valid(node):
            node.visible = enabled


func _set_weather(weather: String, world_day: int) -> void:
    if weather == _active_weather:
        return
    _active_weather = weather
    for node in _weather_nodes:
        if is_instance_valid(node):
            node.queue_free()
    _weather_nodes.clear()

    if weather != "rain" and weather != "snow":
        return

    _weather_rng.seed = world_day * 97 + (11 if weather == "snow" else 23)
    var count := 58 if weather == "snow" else 72
    for i in range(count):
        var particle := MeshInstance3D.new()
        particle.name = "Weather_%s" % i
        var mesh := BoxMesh.new()
        if weather == "rain":
            mesh.size = Vector3(0.025, 0.55, 0.025)
            particle.material_override = material(Color(0.55, 0.72, 0.82, 0.70))
        else:
            mesh.size = Vector3(0.10, 0.10, 0.10)
            particle.material_override = material(Color(0.94, 0.97, 1.0, 0.92))
        particle.mesh = mesh
        particle.position = Vector3(
            _weather_rng.randf_range(-18.0, 18.0),
            _weather_rng.randf_range(2.0, 13.0),
            _weather_rng.randf_range(-22.0, 15.0)
        )
        particle.set_meta("fall_speed", _weather_rng.randf_range(7.0, 11.0) if weather == "rain" else _weather_rng.randf_range(0.9, 1.8))
        particle.set_meta("drift", _weather_rng.randf_range(-0.35, 0.35))
        add_child(particle)
        _weather_nodes.append(particle)


func _update_precipitation(delta: float) -> void:
    if _weather_nodes.is_empty() or player == null:
        return
    var is_snow := _active_weather == "snow"
    for particle in _weather_nodes:
        if not is_instance_valid(particle):
            continue
        var speed := float(particle.get_meta("fall_speed", 1.0))
        var drift := float(particle.get_meta("drift", 0.0))
        particle.position.y -= speed * delta
        if is_snow:
            particle.position.x += drift * delta
        if particle.position.y < 0.15:
            particle.position = Vector3(
                player.position.x + _weather_rng.randf_range(-16.0, 16.0),
                _weather_rng.randf_range(9.0, 14.0),
                player.position.z + _weather_rng.randf_range(-16.0, 16.0)
            )
