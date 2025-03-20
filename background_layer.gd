extends TileMapLayer

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
			
