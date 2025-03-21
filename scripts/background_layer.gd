extends TileMapLayer

func _ready() -> void:
	State.selected_changed.connect(_on_selected_changed)
	_on_selected_changed(Vector2i.ZERO, false)

func _on_selected_changed(old_selected: Vector2i, overwrite_old_selected: bool = true) -> void:
	# Restore old texture for previously selected
	if overwrite_old_selected:
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

func set_rule_complete(rule: Rule) -> void:
	for cell: Cell in rule.cells:
		set_cell(Vector2i(cell.x, cell.y))
	pass

func set_rule_incomplete(rule: Rule) -> void:
	pass

func set_rule_invalid(rule: Rule) -> void:
	pass
