extends Sprite
class_name BaseSprite

export(int) var movement_animation_speed = 3

onready var tween = $Tween

func move_tween(to_position: Vector2):
    tween.interpolate_property(
        self,
        "position",
        position,
        to_position,
        1.0 / movement_animation_speed,
        Tween.TRANS_SINE,
        Tween.EASE_IN_OUT
    )

    tween.start()

func update_sprite(id: String):
    match id:
        "wall":
            region_rect.position = Vector2(0, 207)
        "floor":
            region_rect.position = Vector2(736, 272)
        "player":
            region_rect.position = Vector2(432, 16)
