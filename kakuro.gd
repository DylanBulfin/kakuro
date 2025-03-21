extends Node

var initialized: bool = false

func _ready() -> void:
	State.puzzle_changed.connect(_on_puzzle_changed)

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		var tile_pos: Vector2i = event.position / Const.PX_PER_TILE
		State.change_selection(tile_pos)
	for i: int in range(1, 10):
		if event.is_action_pressed(str(i)):
			State.update_selected_digit(i)

func _on_puzzle_changed() -> void:
	draw_all()

func _process(_delta: float) -> void:
	if not initialized:
		draw_all()
		initialized = true

func draw_all() -> void:
	%BackgroundLayer.draw_puzzle()
	%Rules.draw_puzzle()
	%NumbersLayer.draw_puzzle()
