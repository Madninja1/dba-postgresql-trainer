extends Control

signal vector_changed(value: Vector2)

const BASE_RADIUS := 86.0
const KNOB_RADIUS := 34.0
const DEAD_ZONE := 0.10

var _touch_index := -1
var _mouse_active := false
var _value := Vector2.ZERO
var _knob_offset := Vector2.ZERO


func _ready() -> void:
    mouse_filter = Control.MOUSE_FILTER_STOP
    custom_minimum_size = Vector2(BASE_RADIUS * 2.0, BASE_RADIUS * 2.0)
    queue_redraw()


func _draw() -> void:
    var center := size * 0.5
    draw_circle(center, BASE_RADIUS, Color(0.05, 0.08, 0.11, 0.42))
    draw_circle(center, BASE_RADIUS - 4.0, Color(0.18, 0.25, 0.31, 0.24), false, 4.0)
    draw_circle(center + _knob_offset, KNOB_RADIUS, Color(0.50, 0.82, 0.95, 0.72))
    draw_circle(center + _knob_offset, KNOB_RADIUS - 4.0, Color(0.09, 0.22, 0.29, 0.78))


func _gui_input(event: InputEvent) -> void:
    if event is InputEventScreenTouch:
        if event.pressed and _touch_index == -1:
            _touch_index = event.index
            _update_from_position(event.position)
            accept_event()
        elif not event.pressed and event.index == _touch_index:
            _touch_index = -1
            _reset()
            accept_event()
        return

    if event is InputEventScreenDrag and event.index == _touch_index:
        _update_from_position(event.position)
        accept_event()
        return

    if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT:
        _mouse_active = event.pressed
        if _mouse_active:
            _update_from_position(event.position)
        else:
            _reset()
        accept_event()
        return

    if event is InputEventMouseMotion and _mouse_active:
        _update_from_position(event.position)
        accept_event()


func _update_from_position(position: Vector2) -> void:
    var center := size * 0.5
    var delta := position - center
    if delta.length() > BASE_RADIUS:
        delta = delta.normalized() * BASE_RADIUS

    _knob_offset = delta
    var normalized := delta / BASE_RADIUS
    if normalized.length() < DEAD_ZONE:
        normalized = Vector2.ZERO
    _value = normalized
    vector_changed.emit(_value)
    queue_redraw()


func _reset() -> void:
    _value = Vector2.ZERO
    _knob_offset = Vector2.ZERO
    vector_changed.emit(Vector2.ZERO)
    queue_redraw()
