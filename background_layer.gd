extends TileMapLayer

var selected: Vector2i

func draw_puzzle(puzzle: Puzzle) -> void:
	for x in range(puzzle.width):
		for y in range(puzzle.height):
			var cell: Cell = puzzle.rows[y][x]
			
			if cell.fillable:
				set_cell(Vector2i(x, y), 0, Vector2i(1, 0))
			elif len(cell.rules) == 2:
				set_cell(Vector2i(x, y), 0, Vector2i(3, 0))
			elif len(cell.rules) == 1:
				set_cell(Vector2i(x, y), 0, Vector2i(4, 0))
			
func change_selection(row: int, column: int) -> void:
	if selected:
		set_cell(selected, 0, Vector2i(1, 0))
	selected = Vector2i(column, row)
	set_cell(selected, 0, Vector2i(2, 0))
