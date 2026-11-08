extends Node3D

const CONCRETE_TEXTURE: Texture2D = preload("res://assets/environment/textures/facade_concrete.svg")
const METAL_TEXTURE: Texture2D = preload("res://assets/environment/textures/dark_metal.svg")

var _windows: Array[MeshInstance3D] = []
var _built := false


func _ready() -> void:
    build()


func build() -> void:
    if _built:
        return
    _built = true

    _add_collision(Vector3(25.0, 5.4, 9.0), Vector3(0.0, 2.7, 0.0))

    var shell := _box("ConcreteShell", Vector3(25.0, 5.4, 9.0), Vector3(0.0, 2.7, 0.0))
    shell.material_override = _surface(CONCRETE_TEXTURE, Color("#a4aaa9"), 0.86, Vector3(3.8, 2.2, 2.5))

    var roof := _box("Roof", Vector3(25.8, 0.34, 9.8), Vector3(0.0, 5.55, 0.0))
    roof.material_override = _surface(METAL_TEXTURE, Color("#343b3e"), 0.62, Vector3(4.0, 1.0, 2.0), 0.16)

    var base := _box("FacadeBase", Vector3(25.1, 0.64, 0.30), Vector3(0.0, 0.34, 4.47))
    base.material_override = _surface(METAL_TEXTURE, Color("#3e474a"), 0.70, Vector3(5.0, 1.0, 1.0), 0.12)

    var upper_band := _box("UpperBand", Vector3(25.1, 0.42, 0.34), Vector3(0.0, 4.36, 4.47))
    upper_band.material_override = _surface(METAL_TEXTURE, Color("#263136"), 0.58, Vector3(5.0, 1.0, 1.0), 0.20)

    for x in [-9.5, -4.9, -0.3, 9.3]:
        var joint := _box("FacadeJoint", Vector3(0.07, 4.25, 0.055), Vector3(x, 2.48, 4.55))
        joint.material_override = _plain(Color("#555e60"), 0.88)

    _build_entrance()
    _build_windows()
    _build_service_bay()
    _build_roof_plant()
    _build_facade_details()
    _build_front_landscape()


func window_meshes() -> Array[MeshInstance3D]:
    return _windows


func _build_entrance() -> void:
    var recess := _box("EntranceRecess", Vector3(5.2, 3.95, 0.52), Vector3(5.2, 1.98, 4.50))
    recess.material_override = _surface(METAL_TEXTURE, Color("#202b30"), 0.54, Vector3(2.0, 2.0, 1.0), 0.22)

    for x in [4.30, 6.10]:
        var door := _box("GlassDoor", Vector3(1.62, 2.88, 0.09), Vector3(x, 1.51, 4.82))
        door.material_override = _glass(Color(0.15, 0.29, 0.34, 0.63))
        _windows.append(door)

    var frame_left := _box("DoorFrame", Vector3(0.12, 3.06, 0.18), Vector3(3.43, 1.56, 4.87))
    frame_left.material_override = _surface(METAL_TEXTURE, Color("#192328"), 0.45, Vector3(1.0, 2.0, 1.0), 0.30)
    var frame_center := _box("DoorFrame", Vector3(0.10, 3.06, 0.18), Vector3(5.20, 1.56, 4.87))
    frame_center.material_override = frame_left.material_override
    var frame_right := _box("DoorFrame", Vector3(0.12, 3.06, 0.18), Vector3(6.97, 1.56, 4.87))
    frame_right.material_override = frame_left.material_override

    var canopy := _box("EntranceCanopy", Vector3(6.15, 0.20, 1.58), Vector3(5.2, 3.83, 5.05))
    canopy.material_override = _surface(METAL_TEXTURE, Color("#222c31"), 0.52, Vector3(2.0, 1.0, 1.0), 0.24)

    var canopy_light := _box("CanopyLight", Vector3(4.7, 0.045, 0.08), Vector3(5.2, 3.66, 5.82))
    canopy_light.material_override = _emissive(Color("#74d9e5"), Color("#1b7886"), 1.8)

    _box("AccessReader", Vector3(0.19, 0.42, 0.11), Vector3(7.20, 1.35, 4.92)).material_override = _emissive(Color("#61d7c4"), Color("#17675e"), 1.2)

    # Visible server silhouettes behind the glazing make the entrance feel occupied.
    for x in [4.35, 5.22, 6.08]:
        var rack := _box("ServerRack", Vector3(0.58, 1.92, 0.50), Vector3(x, 1.34, 4.12))
        rack.material_override = _surface(METAL_TEXTURE, Color("#151d21"), 0.48, Vector3(1.0, 2.5, 1.0), 0.28)
        for y in [0.76, 1.12, 1.48, 1.84]:
            var led := _box("RackLed", Vector3(0.35, 0.028, 0.025), Vector3(x, y, 4.39))
            led.material_override = _emissive(Color("#4fc4df"), Color("#176477"), 1.15)


func _build_windows() -> void:
    for x in [-5.5, -2.4, 0.7]:
        var frame := _box("WindowFrame", Vector3(2.42, 1.28, 0.17), Vector3(x, 2.15, 4.55))
        frame.material_override = _surface(METAL_TEXTURE, Color("#263238"), 0.52, Vector3(1.0, 1.0, 1.0), 0.22)
        var window := _box("Window", Vector3(2.12, 0.98, 0.09), Vector3(x, 2.15, 4.66))
        window.material_override = _glass(Color(0.20, 0.39, 0.45, 0.78))
        _windows.append(window)


func _build_service_bay() -> void:
    var service_frame := _box("ServiceFrame", Vector3(3.55, 3.38, 0.22), Vector3(-8.9, 1.72, 4.55))
    service_frame.material_override = _surface(METAL_TEXTURE, Color("#2b3438"), 0.58, Vector3(2.0, 2.0, 1.0), 0.24)
    var service_door := _box("ServiceDoor", Vector3(3.15, 2.98, 0.12), Vector3(-8.9, 1.52, 4.70))
    service_door.material_override = _surface(METAL_TEXTURE, Color("#50595b"), 0.68, Vector3(3.0, 2.0, 1.0), 0.18)
    for y in [0.55, 1.05, 1.55, 2.05, 2.55]:
        _box("ServiceDoorSeam", Vector3(2.82, 0.025, 0.025), Vector3(-8.9, y, 4.78)).material_override = _plain(Color("#252c2f"), 0.80)


func _build_roof_plant() -> void:
    _box("RoofParapetFront", Vector3(25.4, 0.58, 0.24), Vector3(0.0, 5.78, 4.62)).material_override = _surface(METAL_TEXTURE, Color("#4a5355"), 0.66, Vector3(5.0, 1.0, 1.0), 0.14)
    _box("RoofParapetRear", Vector3(25.4, 0.58, 0.24), Vector3(0.0, 5.78, -4.62)).material_override = _surface(METAL_TEXTURE, Color("#4a5355"), 0.66, Vector3(5.0, 1.0, 1.0), 0.14)

    for x in [-7.4, -2.5, 2.4, 7.3]:
        var unit := _box("CoolingUnit", Vector3(3.15, 1.30, 2.25), Vector3(x, 6.55, -0.2))
        unit.material_override = _surface(METAL_TEXTURE, Color("#667174"), 0.56, Vector3(2.0, 1.4, 1.5), 0.18)
        var grille := _box("CoolingGrille", Vector3(2.48, 0.74, 0.055), Vector3(x, 6.52, 0.94))
        grille.material_override = _surface(METAL_TEXTURE, Color("#20272a"), 0.48, Vector3(3.0, 2.0, 1.0), 0.30)
        for offset in [-0.82, -0.41, 0.0, 0.41, 0.82]:
            _box("CoolingSlat", Vector3(0.055, 0.58, 0.035), Vector3(x + offset, 6.52, 0.99)).material_override = _plain(Color("#111719"), 0.58)

    for x in [-9.2, -4.2, 0.8, 5.8]:
        var pipe := _cylinder("RoofPipe", 0.11, 3.6, Vector3(x, 6.05, -2.8), Color("#5b6466"))
        pipe.rotation_degrees = Vector3(0.0, 0.0, 90.0)

    var antenna := _cylinder("Antenna", 0.065, 4.2, Vector3(-10.8, 7.55, -2.6), Color("#4e595c"))
    antenna.material_override = _plain(Color("#4e595c"), 0.45, 0.25)


func _build_facade_details() -> void:
    for x in [-10.2, 10.2]:
        var vent_frame := _box("VentFrame", Vector3(1.95, 2.25, 0.16), Vector3(x, 2.05, 4.56))
        vent_frame.material_override = _surface(METAL_TEXTURE, Color("#2d3538"), 0.55, Vector3(1.0, 2.0, 1.0), 0.26)
        for y in [1.35, 1.65, 1.95, 2.25, 2.55, 2.85]:
            _box("VentSlat", Vector3(1.52, 0.065, 0.035), Vector3(x, y, 4.68)).material_override = _plain(Color("#111719"), 0.60)

    _status_light(Vector3(-11.0, 3.72, 4.68))
    _status_light(Vector3(11.0, 3.72, 4.68))

    _label("SERVER FACILITY 01", Vector3(-1.5, 3.70, 4.71), 42, 0.0038, Color("#edf4f4"))
    _label("DBA OPERATIONS · TRAINING NODE", Vector3(-1.5, 3.20, 4.71), 21, 0.0030, Color("#b5c9cb"))
    _label("SERVICE", Vector3(-8.9, 3.10, 4.79), 18, 0.0030, Color("#d9b35f"))

    # Security camera and utility cabinets are small, but they sell real scale.
    _box("CameraMount", Vector3(0.22, 0.12, 0.34), Vector3(8.5, 3.85, 4.78)).material_override = _plain(Color("#30383b"), 0.58, 0.18)
    var camera := _cylinder("SecurityCamera", 0.09, 0.42, Vector3(8.5, 3.73, 5.02), Color("#d4d6d1"))
    camera.rotation_degrees = Vector3(90.0, 0.0, 0.0)

    for x in [-11.2, -10.35]:
        var cabinet := _box("UtilityCabinet", Vector3(0.62, 1.25, 0.48), Vector3(x, 0.67, 5.03))
        cabinet.material_override = _surface(METAL_TEXTURE, Color("#626b6d"), 0.64, Vector3(1.0, 2.0, 1.0), 0.16)


func _build_front_landscape() -> void:
    for x in [-6.8, -3.7, -0.6, 8.8]:
        var planter := _box("Planter", Vector3(2.55, 0.48, 0.88), Vector3(x, 0.29, 7.75))
        planter.material_override = _surface(CONCRETE_TEXTURE, Color("#777d7c"), 0.90, Vector3(2.0, 1.0, 1.0))
        _box("ShrubMass", Vector3(2.12, 0.42, 0.58), Vector3(x, 0.67, 7.75)).material_override = _plain(Color("#334f35"), 0.96)

    for x in [-12.0, 12.0]:
        _cylinder("FencePost", 0.055, 1.55, Vector3(x, 0.78, 6.85), Color("#2b3336"))
    _box("FenceRail", Vector3(24.0, 0.045, 0.045), Vector3(0.0, 1.32, 6.85)).material_override = _plain(Color("#2b3336"), 0.58, 0.20)
    _box("FenceRail", Vector3(24.0, 0.045, 0.045), Vector3(0.0, 0.66, 6.85)).material_override = _plain(Color("#2b3336"), 0.58, 0.20)


func _status_light(position: Vector3) -> void:
    _box("StatusLightHousing", Vector3(0.42, 0.42, 0.12), position).material_override = _plain(Color("#25302b"), 0.60, 0.10)
    _box("StatusLight", Vector3(0.22, 0.22, 0.06), position + Vector3(0.0, 0.0, 0.09)).material_override = _emissive(Color("#68e58a"), Color("#1d7e3a"), 1.4)
    var light := OmniLight3D.new()
    light.position = position + Vector3(0.0, 0.0, 0.25)
    light.light_color = Color("#70ed90")
    light.light_energy = 0.32
    light.omni_range = 1.8
    add_child(light)


func _add_collision(size: Vector3, position: Vector3) -> void:
    var body := StaticBody3D.new()
    body.name = "FacilityCollision"
    var shape_node := CollisionShape3D.new()
    var shape := BoxShape3D.new()
    shape.size = size
    shape_node.shape = shape
    shape_node.position = position
    body.add_child(shape_node)
    add_child(body)


func _box(node_name: String, size: Vector3, position: Vector3) -> MeshInstance3D:
    var node := MeshInstance3D.new()
    node.name = node_name
    var mesh := BoxMesh.new()
    mesh.size = size
    node.mesh = mesh
    node.position = position
    add_child(node)
    return node


func _cylinder(node_name: String, radius: float, height: float, position: Vector3, color: Color) -> MeshInstance3D:
    var node := MeshInstance3D.new()
    node.name = node_name
    var mesh := CylinderMesh.new()
    mesh.top_radius = radius
    mesh.bottom_radius = radius
    mesh.height = height
    node.mesh = mesh
    node.position = position
    node.material_override = _plain(color, 0.62, 0.16)
    add_child(node)
    return node


func _label(text: String, position: Vector3, font_size: int, pixel_size: float, color: Color) -> void:
    var label := Label3D.new()
    label.text = text
    label.position = position
    label.font_size = font_size
    label.pixel_size = pixel_size
    label.modulate = color
    label.outline_size = 2
    label.outline_modulate = Color(0.03, 0.04, 0.05, 0.72)
    label.billboard = BaseMaterial3D.BILLBOARD_DISABLED
    label.fixed_size = false
    label.double_sided = true
    label.shaded = false
    add_child(label)


func _surface(
    texture: Texture2D,
    tint: Color,
    roughness: float,
    uv_scale: Vector3,
    metallic: float = 0.0
) -> StandardMaterial3D:
    var result := StandardMaterial3D.new()
    result.albedo_texture = texture
    result.albedo_color = tint
    result.roughness = roughness
    result.metallic = metallic
    result.uv1_scale = uv_scale
    return result


func _plain(color: Color, roughness: float, metallic: float = 0.0) -> StandardMaterial3D:
    var result := StandardMaterial3D.new()
    result.albedo_color = color
    result.roughness = roughness
    result.metallic = metallic
    return result


func _glass(color: Color) -> StandardMaterial3D:
    var result := StandardMaterial3D.new()
    result.albedo_color = color
    result.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
    result.roughness = 0.16
    result.metallic = 0.10
    return result


func _emissive(color: Color, emission: Color, energy: float) -> StandardMaterial3D:
    var result := StandardMaterial3D.new()
    result.albedo_color = color
    result.roughness = 0.38
    result.emission_enabled = true
    result.emission = emission
    result.emission_energy_multiplier = energy
    return result
