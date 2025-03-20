extends Node

@onready var puzzle: Puzzle
var selected: Cell

func _ready() -> void:
	var rules: Array[Rule] = [
		Rule.new(1, 1, 2, 17, true),
		Rule.new(1, 1, 2, 16, false),
		Rule.new(0, 2, 3, 12, true),
		Rule.new(2, 0, 3, 12, false),
		Rule.new(0, 3, 3, 10, true),
		Rule.new(3, 0, 3, 11, false),
	]
	
	puzzle = Puzzle.new(4, 4, rules)
	%BackgroundLayer.draw_puzzle(puzzle)
	%Rules.draw_puzzle(puzzle)
	
	# Find first selectable entry
	var first_entry: Cell
	for cell in puzzle.rows[1]:
		if cell.fillable:
			first_entry = cell
			break

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		var tile_pos: Vector2i = event.position / 32
		change_selection(tile_pos.y, tile_pos.x)
		

func change_selection(row: int, column: int) -> void:
	var cell: Cell = puzzle.rows[row][column]
	if cell.fillable:
		selected = cell
		%BackgroundLayer.change_selection(row, column)
