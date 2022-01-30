extends Node2D

onready var ecs = $ECS

func _ready():
    Console.add_command('tdp', self, 'toggle_draw_path')\
        .set_description('Draw NPC paths')\
        .register()
    Console.add_command('noclip', self, 'toggle_no_clip')\
        .set_description('Toggle player collision')\
        .register()

func _on_Node_movement_input(direction):
    ecs.move_player(direction)

func console_print(line):
    Console.write_line(line)

func toggle_draw_path():
    ecs.toggle_draw_path()

func toggle_no_clip():
    ecs.toggle_no_clip()
