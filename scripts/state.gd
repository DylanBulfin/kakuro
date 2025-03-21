extends Node

var puzzle: Puzzle
signal puzzle_changed
signal cell_digit_changed(cell: Cell)

# It would be useful to have no selection, for example a mode where you select the number first
var is_selected: bool = true
var selected_cell: Cell
signal selected_changed(old_selected: Cell)

var babby_rules: Array[Rule] = [
	Rule.new_vert(1, 1, 2, 17),
	Rule.new_horiz(1, 1, 2, 16),
	Rule.new_vert(2, 0, 3, 12),
	Rule.new_horiz(0, 2, 3, 12),
	Rule.new_vert(3, 0, 3, 10),
	Rule.new_horiz(0, 3, 3, 11),
]

var less_babby_rules: Array[Rule] = [
	Rule.new_vert(1, 0, 2, 11),
	Rule.new_vert(2, 0, 8, 43),
	Rule.new_vert(6, 0, 2, 17),
	Rule.new_vert(7, 0, 8, 41),
	Rule.new_vert(4, 1, 2, 3),
	Rule.new_vert(5, 1, 2, 13),
	Rule.new_vert(3, 2, 2, 4),
	Rule.new_vert(8, 2, 2, 4),
	Rule.new_vert(1, 4, 2, 15),
	Rule.new_vert(6, 4, 2, 17),
	Rule.new_vert(4, 5, 2, 16),
	Rule.new_vert(5, 5, 2, 9),
	Rule.new_vert(3, 6, 2, 15),
	Rule.new_vert(8, 6, 2, 7),
	
	Rule.new_horiz(0, 1, 2, 16),
	Rule.new_horiz(0, 2, 2, 8),
	Rule.new_horiz(0, 5, 2, 8),
	Rule.new_horiz(0, 6, 2, 11),
	Rule.new_horiz(1, 3, 4, 19),
	Rule.new_horiz(1, 4, 2, 11),
	Rule.new_horiz(1, 7, 4, 19),
	Rule.new_horiz(1, 8, 2, 14),
	Rule.new_horiz(3, 2, 4, 24),
	Rule.new_horiz(3, 6, 4, 30),
	Rule.new_horiz(5, 1, 2, 9),
	Rule.new_horiz(5, 5, 2, 16),
	Rule.new_horiz(6, 3, 2, 3),
	Rule.new_horiz(6, 4, 2, 12),
	Rule.new_horiz(6, 7, 2, 6),
	Rule.new_horiz(6, 8, 2, 9),	
]

func _ready() -> void:
	#load_new_puzzle(4, 4, babby_rules)
	load_new_puzzle(9, 9, less_babby_rules)

func load_new_puzzle(width: int, height: int, rules: Array[Rule]) -> void:
	puzzle = Puzzle.new(width, height, rules)
	
	# Find first selectable entry
	for x in range(puzzle.width):
		var cell: Cell = puzzle.get_cell(x, 1)
		if cell.fillable:
			selected_cell = cell
			break

	selected_changed.emit()
	puzzle_changed.emit()

func change_selection(coords: Vector2i) -> void:
	# No need to reselect
	if coords == selected_cell.position: return
	
	# Outside of the game board, ignore
	if coords.x >= puzzle.width or coords.y >= puzzle.height: return
	
	var cell: Cell = puzzle.get_cellv(coords)
	if cell.fillable:
		var old_selected: Cell = selected_cell
		selected_cell = cell
		is_selected = true
		
		selected_changed.emit(old_selected)

func update_selected_digit(digit: int) -> void:
	if is_selected and selected_cell.digit != digit:
		selected_cell.digit = digit
		
		var all_finished: bool = true
		
		for rule in selected_cell.rules:
			var state: Rule.RuleState = rule.validate_rule()
			if state == Rule.RuleState.Incomplete:
				all_finished = false
		
		cell_digit_changed.emit(selected_cell)
		
		if all_finished:
			var complete: bool = true
			
			# All of this cell's rules are satisfied, check the rest
			for rule: Rule in puzzle.rules:
				if rule.validate_rule() != Rule.RuleState.Complete:
					complete = false
					break
			
			if complete:
				print("Complete")
