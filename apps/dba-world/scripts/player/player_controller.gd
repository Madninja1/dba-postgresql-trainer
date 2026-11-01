extends CharacterBody3D

const MOVE_SPEED := 5.6
const ACCELERATION := 18.0
const DECELERATION := 24.0
const GRAVITY := 18.0
const TURN_SPEED := 9.0
const CAMERA_DISTANCE := 6.5
const CAMERA_HEIGHT := 2.25
const CAMERA_MIN_PITCH := -0.82
const CAMERA_MAX_PITCH := 0.18

var _camera_rig: Node3D
var _spring_arm: SpringArm3D
var _camera: Camera3D
var _camera_yaw := 0.0
var _camera_pitch := -0.24

var _mobile_vector := Vector2.ZERO


func _ready() -> void:
    _build_character()
    _build_camera()


func _physics_process(delta: float) -> void:
    var input_vector := Input.get_vector("move_left", "move_right", "move_forward", "move_back")
    if _mobile_vector.length() > input_vector.length():
        input_vector = _mobile_vector

    var direction := Vector3.ZERO
    var input_strength := min(input_vector.length(), 1.0)
    if input_strength > 0.01:
        var forward := -_camera_rig.global_transform.basis.z
        var right := _camera_rig.global_transform.basis.x
        forward.y = 0.0
        right.y = 0.0
        direction = (right.normalized() * input_vector.x + forward.normalized() * -input_vector.y).normalized()

    var target_x := direction.x * MOVE_SPEED * input_strength
    var target_z := direction.z * MOVE_SPEED * input_strength
    var rate := ACCELERATION if direction.length_squared() > 0.01 else DECELERATION
    velocity.x = move_toward(velocity.x, target_x, rate * delta)
    velocity.z = move_toward(velocity.z, target_z, rate * delta)

    if is_on_floor():
        velocity.y = -0.2
    else:
        velocity.y -= GRAVITY * delta

    move_and_slide()

    if direction.length_squared() > 0.01:
        var target_rotation := atan2(direction.x, direction.z)
        rotation.y = lerp_angle(rotation.y, target_rotation, delta * TURN_SPEED)

    _camera_rig.global_position = global_position + Vector3(0.0, CAMERA_HEIGHT, 0.0)


func set_mobile_vector(value: Vector2) -> void:
    _mobile_vector = value.limit_length(1.0)


func clear_mobile_input() -> void:
    _mobile_vector = Vector2.ZERO


func orbit_camera(delta_pixels: Vector2) -> void:
    _camera_yaw -= delta_pixels.x * 0.0055
    _camera_pitch = clamp(
        _camera_pitch - delta_pixels.y * 0.0042,
        CAMERA_MIN_PITCH,
        CAMERA_MAX_PITCH
    )
    _apply_camera_rotation()


func _build_character() -> void:
    var collision := CollisionShape3D.new()
    var capsule := CapsuleShape3D.new()
    capsule.radius = 0.42
    capsule.height = 1.8
    collision.shape = capsule
    add_child(collision)

    var body_mesh := MeshInstance3D.new()
    var body := CapsuleMesh.new()
    body.radius = 0.42
    body.height = 1.8
    body_mesh.mesh = body
    body_mesh.material_override = _material(Color("#247ca5"))
    add_child(body_mesh)

    var visor := MeshInstance3D.new()
    var visor_mesh := BoxMesh.new()
    visor_mesh.size = Vector3(0.38, 0.16, 0.12)
    visor.mesh = visor_mesh
    visor.position = Vector3(0.0, 0.43, 0.39)
    visor.material_override = _material(Color("#9be8ff"), 0.15)
    add_child(visor)


func _build_camera() -> void:
    _camera_rig = Node3D.new()
    _camera_rig.name = "CameraRig"
    _camera_rig.top_level = true
    add_child(_camera_rig)

    _spring_arm = SpringArm3D.new()
    _spring_arm.name = "SpringArm"
    _spring_arm.spring_length = CAMERA_DISTANCE
    _spring_arm.margin = 0.18
    _spring_arm.add_excluded_object(get_rid())
    _camera_rig.add_child(_spring_arm)

    _camera = Camera3D.new()
    _camera.name = "ThirdPersonCamera"
    _camera.current = true
    _spring_arm.add_child(_camera)

    _camera_rig.global_position = global_position + Vector3(0.0, CAMERA_HEIGHT, 0.0)
    _apply_camera_rotation()


func _apply_camera_rotation() -> void:
    if _camera_rig == null:
        return
    _camera_rig.rotation = Vector3(_camera_pitch, _camera_yaw, 0.0)


func _material(color: Color, metallic: float = 0.0) -> StandardMaterial3D:
    var material := StandardMaterial3D.new()
    material.albedo_color = color
    material.metallic = metallic
    material.roughness = 0.72
    return material
