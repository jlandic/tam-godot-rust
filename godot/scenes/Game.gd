extends Node2D

onready var ecs = $ECS

func _on_Node_movement_input(direction):
    ecs.move_player(direction)
