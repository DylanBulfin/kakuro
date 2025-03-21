extends Node

var puzzle: Puzzle
signal puzzle_changed
signal cell_digit_changed(coords: Vector2i, cell: Cell)

# It would be useful to have no selection, for example a mode where you select the number first
var is_selected: bool = true
var selected: Vector2i
var selected_cell: Cell
signal selected_changed(old_selected: Vector2i)

func _ready() -> void:
	var rules: Array[Rule] = [
		Rule.new_vert(1, 1, 2, 17),
		Rule.new_horiz(1, 1, 2, 16),
		Rule.new_vert(2, 0, 3, 12),
		Rule.new_horiz(0, 2, 3, 12),
		Rule.new_vert(3, 0, 3, 10),
		Rule.new_horiz(0, 3, 3, 11),
	]
	
	load_new_puzzle(4, 4, rules)

func load_new_puzzle(width: int, height: int, rules: Array[Rule]) -> void:
	puzzle = Puzzle.new(width, height, rules)
	
	# Find first selectable entry
	for i in puzzle.width:
		if puzzle.rows[1][i].fillable:
			selected = Vector2i(i, 1)
			break
	
	selected_cell = puzzle.rows[selected.y][selected.x]
	selected_changed.emit()
	
	puzzle_changed.emit()

func change_selection(coords: Vector2i) -> void:
	# No need to reselect
	if coords == selected: return
	
	# Outside of the game board, ignore
	if coords.x >= puzzle.width or coords.y >= puzzle.height: return
	
	var cell: Cell = puzzle.rows[coords.y][coords.x]
	if cell.fillable:
		var old_selected = selected
		selected_cell = cell
		selected = coords
		is_selected = true
		
		selected_changed.emit(old_selected)

func update_selected_digit(digit: int) -> void:
	if is_selected and selected_cell.digit != digit:
		var old_digit: int = selected_cell.digit
		selected_cell.digit = digit
		
		var all_finished: bool = true
		
		for rule in selected_cell.rules:
			var state: Rule.RuleState = rule.validate_rule()
			if state == Rule.RuleState.Invalid:
				selected_cell.digit = old_digit
				return
			if state == Rule.RuleState.Incomplete:
				all_finished = false
		
		cell_digit_changed.emit(selected, selected_cell)
		
		if all_finished:
			var complete: bool = true
			
			# All of this cell's rules are satisfied, check the rest
			for rule: Rule in puzzle.rules:
				if rule.validate_rule() != Rule.RuleState.Complete:
					complete = false
					break
			
			if complete:
				print("Complete")
