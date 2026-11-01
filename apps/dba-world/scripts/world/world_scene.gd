extends Node3D

const PLAYER_SCRIPT_PATH := "res://scripts/player/player_controller.gd"
const JOYSTICK_SCRIPT_PATH := "res://scripts/ui/virtual_joystick.gd"

var player: CharacterBody3D
var _prompt_panel: PanelContainer
var _prompt_label: Label
var _interact_button: Button
var _objective_label: Label
var _date_label: Label
var _message_label: Label
var _message_timer: Timer
var _interactions: Array[Dictionary] = []
var _active_interaction: Dictionary = {}
var _environment_snapshot: Dictionary = {}
var _environment_poll_elapsed := 0.0


func _ready() -> void:
    _environment_snapshot = _read_environment_snapshot()
    _build_environment()
    _build_world()
    _spawn_player(_player_spawn())
    _build_hud()
    _register_world_interactions()
    _update_date_label()
    _on_environment_snapshot_changed(_environment_snapshot)


func _process(delta: float) -> void:
    _update_interaction_prompt()
    _environment_poll_elapsed += delta
    if _environment_poll_elapsed >= 1.0:
        _environment_poll_elapsed = 0.0
        _poll_environment_snapshot()


func _unhandled_input(event: InputEvent) -> void:
    if event.is_action_pressed("interact"):
        _activate_interaction()
        get_viewport().set_input_as_handled()
        return

    if player == null:
        return

    if event is InputEventScreenDrag:
        var viewport_width := get_viewport().get_visible_rect().size.x
        if event.position.x > viewport_width * 0.42 and player.has_method("orbit_camera"):
            player.call("orbit_camera", event.screen_relative)
    elif event is InputEventMouseMotion and Input.is_mouse_button_pressed(MOUSE_BUTTON_RIGHT):
        if player.has_method("orbit_camera"):
            player.call("orbit_camera", event.screen_relative)


func _build_world() -> void:
    pass


func _player_spawn() -> Vector3:
    return Vector3(0.0, 1.0, 0.0)


func _scene_title() -> String:
    return "DBA World"


func _scene_objective() -> String:
    return "Explore"


func _register_world_interactions() -> void:
    pass


func register_interaction(
    target: Node3D,
    label: String,
    action: Callable,
    radius: float = 2.4
) -> void:
    _interactions.append({
        "target": target,
        "label": label,
        "action": action,
        "radius": radius,
    })


func add_marker(marker_name: String, position: Vector3) -> Node3D:
    var marker := Node3D.new()
    marker.name = marker_name
    marker.position = position
    add_child(marker)
    return marker


func change_world(scene_path: String) -> void:
    if player != null and player.has_method("clear_mobile_input"):
        player.call("clear_mobile_input")
    get_tree().change_scene_to_file(scene_path)


func show_message(text: String, seconds: float = 3.0) -> void:
    if _message_label == null or _message_timer == null:
        return
    _message_label.text = text
    _message_label.visible = true
    _message_timer.stop()
    _message_timer.wait_time = seconds
    _message_timer.start()


func _hide_message() -> void:
    if _message_label != null:
        _message_label.visible = false


func _spawn_player(position: Vector3) -> void:
    # Load gameplay scripts at runtime instead of preloading them while the base
    # class is parsed. The embedded Android build currently ships raw Godot
    # assets without project.binary/global script cache, so a broken optional
    # gameplay script must not prevent the whole world scene from resolving.
    var player_script = load(PLAYER_SCRIPT_PATH)
    if player_script == null:
        push_error("DBA World: failed to load player controller: %s" % PLAYER_SCRIPT_PATH)
        return

    var instance = player_script.new()
    if not (instance is CharacterBody3D):
        push_error("DBA World: player controller did not create CharacterBody3D")
        if instance != null:
            instance.free()
        return

    player = instance
    player.name = "Engineer"
    player.position = position
    add_child(player)


func _build_environment() -> void:
    var world_environment := WorldEnvironment.new()
    var env := Environment.new()
    env.background_mode = Environment.BG_COLOR
    env.background_color = _background_color()
    env.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
    env.ambient_light_color = Color("#dce8ed")
    env.ambient_light_energy = 0.72
    world_environment.environment = env
    add_child(world_environment)

    var sun := DirectionalLight3D.new()
    sun.rotation_degrees = Vector3(-48.0, -28.0, 0.0)
    sun.light_energy = 1.08
    sun.shadow_enabled = true
    add_child(sun)


func _background_color() -> Color:
    return Color("#718fa3")


func _on_environment_snapshot_changed(_snapshot: Dictionary) -> void:
    pass


func environment_snapshot() -> Dictionary:
    return _environment_snapshot.duplicate(true)


func _platform_bridge() -> Node:
    return get_node_or_null("/root/PlatformBridge")


func _read_environment_snapshot() -> Dictionary:
    var bridge = _platform_bridge()
    if bridge != null and bridge.has_method("world_environment"):
        var snapshot = bridge.call("world_environment")
        if snapshot is Dictionary:
            return snapshot.duplicate(true)
    return _fallback_environment_snapshot()


func _fallback_environment_snapshot() -> Dictionary:
    var now := Time.get_datetime_dict_from_system()
    var hour := int(now.get("hour", 12))
    var phase := "night"
    if hour >= 6 and hour <= 9:
        phase = "morning"
    elif hour >= 10 and hour <= 17:
        phase = "day"
    elif hour >= 18 and hour <= 21:
        phase = "evening"

    return {
        "year": 2045,
        "month": 1,
        "day": 1,
        "world_day": 1,
        "season": "winter",
        "day_phase": phase,
        "weather": "clear",
        "new_year_decorations": true,
    }


func _poll_environment_snapshot() -> void:
    var latest = _read_environment_snapshot()
    if latest == _environment_snapshot:
        return
    _environment_snapshot = latest
    _update_date_label()
    _on_environment_snapshot_changed(_environment_snapshot)


func _update_date_label() -> void:
    if _date_label == null:
        return
    var year := int(_environment_snapshot.get("year", 2045))
    var month := int(_environment_snapshot.get("month", 1))
    var day := int(_environment_snapshot.get("day", 1))
    _date_label.text = "%02d.%02d.%04d" % [day, month, year]


func _request_exit() -> void:
    var bridge = _platform_bridge()
    if bridge != null and bridge.has_method("request_exit"):
        bridge.call("request_exit")
        return
    get_tree().quit()


func _build_hud() -> void:
    var layer := CanvasLayer.new()
    layer.name = "WorldHud"
    add_child(layer)

    var title_panel := PanelContainer.new()
    title_panel.position = Vector2(18.0, 16.0)
    title_panel.size = Vector2(430.0, 108.0)
    layer.add_child(title_panel)

    var title_box := VBoxContainer.new()
    title_box.add_theme_constant_override("separation", 3)
    title_panel.add_child(title_box)

    var title := Label.new()
    title.text = _scene_title()
    title.add_theme_font_size_override("font_size", 23)
    title_box.add_child(title)

    _objective_label = Label.new()
    _objective_label.text = _scene_objective()
    _objective_label.add_theme_font_size_override("font_size", 14)
    title_box.add_child(_objective_label)

    _date_label = Label.new()
    _date_label.add_theme_font_size_override("font_size", 15)
    title_box.add_child(_date_label)

    var exit_button := Button.new()
    exit_button.text = "← DBA Trainer"
    exit_button.position = Vector2(1084.0, 20.0)
    exit_button.size = Vector2(174.0, 52.0)
    exit_button.pressed.connect(_request_exit)
    layer.add_child(exit_button)

    _build_move_pad(layer)

    _interact_button = Button.new()
    _interact_button.text = "INTERACT"
    _interact_button.position = Vector2(1090.0, 612.0)
    _interact_button.size = Vector2(160.0, 72.0)
    _interact_button.visible = false
    _interact_button.pressed.connect(_activate_interaction)
    layer.add_child(_interact_button)

    _prompt_panel = PanelContainer.new()
    _prompt_panel.position = Vector2(360.0, 620.0)
    _prompt_panel.size = Vector2(560.0, 58.0)
    _prompt_panel.visible = false
    layer.add_child(_prompt_panel)

    _prompt_label = Label.new()
    _prompt_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
    _prompt_label.vertical_alignment = VERTICAL_ALIGNMENT_CENTER
    _prompt_label.add_theme_font_size_override("font_size", 18)
    _prompt_panel.add_child(_prompt_label)

    _message_label = Label.new()
    _message_label.position = Vector2(370.0, 112.0)
    _message_label.size = Vector2(540.0, 54.0)
    _message_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
    _message_label.vertical_alignment = VERTICAL_ALIGNMENT_CENTER
    _message_label.add_theme_font_size_override("font_size", 18)
    _message_label.visible = false
    layer.add_child(_message_label)

    _message_timer = Timer.new()
    _message_timer.one_shot = true
    _message_timer.timeout.connect(_hide_message)
    add_child(_message_timer)


func _build_move_pad(layer: CanvasLayer) -> void:
    # Keep the analogue stick, but load it only after the world scene itself is
    # alive. This prevents an optional UI script from invalidating the base
    # world class during GDScript dependency resolution.
    var joystick_script = load(JOYSTICK_SCRIPT_PATH)
    if joystick_script == null:
        push_error("DBA World: failed to load virtual joystick; using fallback controls")
        _build_fallback_move_pad(layer)
        return

    var joystick_instance = joystick_script.new()
    if not (joystick_instance is Control):
        push_error("DBA World: virtual joystick did not create Control; using fallback controls")
        if joystick_instance != null:
            joystick_instance.free()
        _build_fallback_move_pad(layer)
        return

    var joystick: Control = joystick_instance
    joystick.name = "MovementStick"
    joystick.position = Vector2(26.0, 492.0)
    joystick.size = Vector2(196.0, 196.0)
    if joystick.has_signal("vector_changed"):
        joystick.connect("vector_changed", Callable(self, "_on_joystick_vector_changed"))
    else:
        push_error("DBA World: virtual joystick has no vector_changed signal")
    layer.add_child(joystick)


func _on_joystick_vector_changed(value: Vector2) -> void:
    if player != null and player.has_method("set_mobile_vector"):
        player.call("set_mobile_vector", value)


func _build_fallback_move_pad(layer: CanvasLayer) -> void:
    _add_fallback_move_button(layer, "↑", Vector2(104.0, 512.0), Vector2(0.0, -1.0))
    _add_fallback_move_button(layer, "↓", Vector2(104.0, 624.0), Vector2(0.0, 1.0))
    _add_fallback_move_button(layer, "←", Vector2(24.0, 624.0), Vector2(-1.0, 0.0))
    _add_fallback_move_button(layer, "→", Vector2(184.0, 624.0), Vector2(1.0, 0.0))


func _add_fallback_move_button(
    layer: CanvasLayer,
    text: String,
    position: Vector2,
    vector: Vector2
) -> void:
    var button := Button.new()
    button.text = text
    button.position = position
    button.size = Vector2(72.0, 64.0)
    button.button_down.connect(_on_fallback_move.bind(vector))
    button.button_up.connect(_on_fallback_move.bind(Vector2.ZERO))
    button.mouse_exited.connect(_on_fallback_move.bind(Vector2.ZERO))
    layer.add_child(button)


func _on_fallback_move(vector: Vector2) -> void:
    _on_joystick_vector_changed(vector)


func _update_interaction_prompt() -> void:
    if player == null or _prompt_panel == null or _interact_button == null:
        return

    var nearest: Dictionary = {}
    var nearest_distance := INF
    for interaction in _interactions:
        var target := interaction.get("target") as Node3D
        if target == null or not is_instance_valid(target):
            continue
        var distance := player.global_position.distance_to(target.global_position)
        var radius := float(interaction.get("radius", 2.4))
        if distance <= radius and distance < nearest_distance:
            nearest = interaction
            nearest_distance = distance

    _active_interaction = nearest
    var has_active := not _active_interaction.is_empty()
    _prompt_panel.visible = has_active
    _interact_button.visible = has_active
    if has_active:
        var label := str(_active_interaction.get("label", "Interact"))
        _prompt_label.text = "E / INTERACT · %s" % label
        _interact_button.text = "INTERACT"


func _activate_interaction() -> void:
    if _active_interaction.is_empty():
        return
    var action: Callable = _active_interaction["action"]
    if action.is_valid():
        action.call()


func add_box(
    node_name: String,
    size: Vector3,
    position: Vector3,
    color: Color,
    collision: bool = true
) -> MeshInstance3D:
    var mesh_instance := MeshInstance3D.new()
    mesh_instance.name = node_name
    var mesh := BoxMesh.new()
    mesh.size = size
    mesh_instance.mesh = mesh
    mesh_instance.position = position
    mesh_instance.material_override = material(color)
    add_child(mesh_instance)

    if collision:
        var body := StaticBody3D.new()
        body.name = "%sCollision" % node_name
        body.position = position
        var shape_node := CollisionShape3D.new()
        var shape := BoxShape3D.new()
        shape.size = size
        shape_node.shape = shape
        body.add_child(shape_node)
        add_child(body)

    return mesh_instance


func add_cylinder(
    node_name: String,
    radius: float,
    height: float,
    position: Vector3,
    color: Color
) -> MeshInstance3D:
    var instance := MeshInstance3D.new()
    instance.name = node_name
    var mesh := CylinderMesh.new()
    mesh.top_radius = radius
    mesh.bottom_radius = radius
    mesh.height = height
    instance.mesh = mesh
    instance.position = position
    instance.material_override = material(color)
    add_child(instance)
    return instance


func add_label_3d(text: String, position: Vector3, color: Color = Color.WHITE, size: int = 42) -> Label3D:
    var label := Label3D.new()
    label.text = text
    label.position = position
    label.font_size = size
    label.modulate = color
    label.billboard = BaseMaterial3D.BILLBOARD_ENABLED
    label.fixed_size = true
    add_child(label)
    return label


func add_omni_light(position: Vector3, color: Color, energy: float, range: float) -> OmniLight3D:
    var light := OmniLight3D.new()
    light.position = position
    light.light_color = color
    light.light_energy = energy
    light.omni_range = range
    add_child(light)
    return light


func material(color: Color, metallic: float = 0.0, emission: Color = Color(0.0, 0.0, 0.0, 1.0)) -> StandardMaterial3D:
    var result := StandardMaterial3D.new()
    result.albedo_color = color
    result.metallic = metallic
    result.roughness = 0.76
    if emission != Color(0.0, 0.0, 0.0, 1.0):
        result.emission_enabled = true
        result.emission = emission
        result.emission_energy_multiplier = 1.5
    return result
