extends TileMapLayer

const INVALID_ID: int = 6
const INCOMPLETE_ID: int = 1
const COMPLETE_ID: int = 5

func _ready() -> void:
	State.selected_changed.connect(_on_selected_changed)
	State.rule_state_changed.connect(_on_rule_state_changed)

func _on_selected_changed(old_selected: Cell) -> void:
	# Restore old texture for previously selected
	set_cell(old_selected.position, 0, Vector2i(1, 0))
	update_correctness()


func _on_rule_state_changed() -> void:
	update_correctness()
	
func update_correctness() -> void:
	# Recalculate validity for each cell, I should do this more efficiently
	for row: Array in State.puzzle.rows:
		for cell: Cell in row:
			if not cell.fillable: continue
			
			var states: Array = cell.rules.map(
				func(r): return r.validate_rule())
			
			if Rule.RuleState.Invalid in states:
				set_cell(cell.position, 0, Vector2i(INVALID_ID, 0))
			elif Rule.RuleState.Incomplete in states:
				set_cell(cell.position, 0, Vector2i(INCOMPLETE_ID, 0))
			else:
				# All complete
				set_cell(cell.position, 0, Vector2i(COMPLETE_ID, 0))
	
	highlight_selected()

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

func set_rule_complete(rule: Rule) -> void:
	for cell: Cell in rule.cells:
		set_cell(cell.position, 0, Vector2i(5, 0))

func set_rule_incomplete(rule: Rule) -> void:
	for cell: Cell in rule.cells:
		set_cell(cell.position, 0, Vector2i(1, 0))
		# In case this was the selected cell
		highlight_selected()

func set_rule_invalid(rule: Rule) -> void:
	for cell: Cell in rule.cells:
		set_cell(cell.position, 0, Vector2i(6, 0))
