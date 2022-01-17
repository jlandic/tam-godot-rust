extends Node

signal movement_input(direction)

const DIRECTIONS := {
    "left": Vector2.LEFT,
    "right": Vector2.RIGHT,
    "up": Vector2.UP,
    "down": Vector2.DOWN,
    "up_left": Vector2.UP + Vector2.LEFT,
    "up_right": Vector2.UP + Vector2.RIGHT,
    "down_left": Vector2.DOWN + Vector2.LEFT,
    "down_right": Vector2.DOWN + Vector2.RIGHT,
}

const DIAGONAL_DIRECTIONS := [
    "up_left",
    "up_right",
    "down_left",
    "down_right",
]

func _unhandled_input(event):
    for direction in DIRECTIONS.keys():
        if event.is_action_pressed(direction):
            emit_signal("movement_input", DIRECTIONS[direction])
            return
