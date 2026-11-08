extends Node3D

const ALEKSEY_MODEL_PATH := "res://assets/characters/aleksey/models/aleksey.glb"
const MODEL_FORWARD_YAW_DEGREES := 0.0
const MODEL_SCALE := Vector3.ONE

@onready var _model_root: Node3D = $ModelRoot
@onready var _camera_focus: Marker3D = $CameraFocus
@onready var _name_anchor: Marker3D = $NameAnchor

var _model_instance: Node3D


func _ready() -> void:
    _load_model()


func model_loaded() -> bool:
    return _model_instance != null and is_instance_valid(_model_instance)


func camera_focus() -> Marker3D:
    return _camera_focus


func name_anchor() -> Marker3D:
    return _name_anchor


func _load_model() -> void:
    if not ResourceLoader.exists(ALEKSEY_MODEL_PATH):
        _show_missing_model_marker()
        push_warning(
            "DBA World: Aleksey model is not installed yet: %s. Add the authored GLB described in assets/characters/aleksey/README.md."
            % ALEKSEY_MODEL_PATH
        )
        return

    var resource = load(ALEKSEY_MODEL_PATH)
    if not (resource is PackedScene):
        _show_missing_model_marker()
        push_error("DBA World: Aleksey GLB did not import as PackedScene: %s" % ALEKSEY_MODEL_PATH)
        return

    var packed_scene: PackedScene = resource
    var instance = packed_scene.instantiate()
    if not (instance is Node3D):
        if instance != null:
            instance.free()
        _show_missing_model_marker()
        push_error("DBA World: Aleksey GLB root must be Node3D: %s" % ALEKSEY_MODEL_PATH)
        return

    _model_instance = instance as Node3D
    _model_instance.name = "AlekseyModel"
    _model_instance.scale = MODEL_SCALE
    _model_instance.rotation_degrees.y = MODEL_FORWARD_YAW_DEGREES
    _model_root.add_child(_model_instance)


func _show_missing_model_marker() -> void:
    var marker := MeshInstance3D.new()
    marker.name = "MissingAlekseyModel"

    var mesh := CapsuleMesh.new()
    mesh.radius = 0.32
    mesh.height = 1.72
    marker.mesh = mesh

    var marker_material := StandardMaterial3D.new()
    marker_material.albedo_color = Color("#2e3236")
    marker_material.roughness = 0.82
    marker.material_override = marker_material
    _model_root.add_child(marker)
