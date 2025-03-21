extends TileMapLayer

func _ready() -> void:
	State.selected_changed.connect(_on_selected_changed)

func _on_selected_changed(old_selected: Vector2i) -> void:
	# Restore old texture for previously selected
	set_cell(old_selected, 0, Vector2i(1, 0))
	set_cell(State.selected, 0, Vector2i(2, 0))

func draw_puzzle() -> void:
	for x in range(State.puzzle.width):
		for y in range(State.puzzle.height):
			var cell: Cell = State.puzzle.rows[y][x]
			
			if cell.fillable:
				set_cell(Vector2i(x, y), 0, Vector2i(1, 0))
			elif len(cell.rules) == 2:
				set_cell(Vector2i(x, y), 0, Vector2i(3, 0))
			elif len(cell.rules) == 1:
				set_cell(Vector2i(x, y), 0, Vector2i(4, 0))
	
