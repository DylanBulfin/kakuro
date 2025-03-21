extends TileMapLayer

const INVALID_ID: int = 6
const INCOMPLETE_ID: int = 1
const COMPLETE_ID: int = 5

func _ready() -> void:
	State.selected_changed.connect(_on_selected_changed)
	State.cell_digit_changed.connect(_on_cell_digit_changed)

func _on_selected_changed(old_selected: Cell) -> void:
	# Restore old texture for previously selected
	set_cell(old_selected.position, 0, Vector2i(1, 0))
	update_cell_state(old_selected)
	highlight_selected()

func _on_cell_digit_changed(changed_cell: Cell) -> void:
	var cells: Array[Cell]
	
	for rule: Rule in changed_cell.rules:
		for cell: Cell in rule.cells:
			if cell not in cells: cells.append(cell)
	
	for cell: Cell in cells:
		update_cell_state(cell)
		

	if State.selected_cell in cells:
		highlight_selected()

func update_cell_state(cell: Cell) -> void:
	if not cell.fillable: return
	
	# I should cache states
	var states: Array = cell.rules.map(func(r): return r.validate_rule())
	
	if Rule.RuleState.Complete in states:
		set_cell(cell.position, 0, Vector2i(COMPLETE_ID, 0))
	elif Rule.RuleState.Invalid in states:
		set_cell(cell.position, 0, Vector2i(INVALID_ID, 0))
	else:
		# All incomplete
		set_cell(cell.position, 0, Vector2i(INCOMPLETE_ID, 0))

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
	
	highlight_selected()

func highlight_selected() -> void:
	set_cell(State.selected_cell.position, 0, Vector2i(2, 0))
