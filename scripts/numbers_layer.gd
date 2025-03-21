extends TileMapLayer

func _ready() -> void:
	State.cell_digit_changed.connect(_on_cell_digit_changed)

func _on_cell_digit_changed(coords: Vector2i, cell: Cell) -> void:
	set_cell(coords, 0, Vector2i(cell.digit - 1, 0))

func draw_puzzle() -> void:
	for x in range(State.puzzle.width):
		for y in range(State.puzzle.height):
			var cell: Cell = State.puzzle.rows[y][x]
			
			if cell.fillable and cell.digit != 0:
				set_cell(Vector2i(x, y), 0, Vector2i(cell.digit - 1, 0))
			

func update_cell(coords: Vector2i) -> void:
	var cell: Cell = State.puzzle.rows[coords.y][coords.x]
	
	if cell.fillable and cell.digit != 0:
		set_cell(coords, 0, Vector2i(cell.digit - 1, 0))
