tool
extends Area2D

onready var animation_player = $AnimationPlayer

export(bool) var open := false setget set_open

func set_open(new_open: bool):
    open = new_open

    if Engine.editor_hint:
        if open:
            $AnimationPlayer.play("Open")
        else:
            $AnimationPlayer.play("Close")
    else:
        if open:
            animation_player.play("Open")
        else:
            animation_player.play("Close")

func interact():
    set_open(true)
