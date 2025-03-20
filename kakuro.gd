extends Node

@onready var puzzle: Puzzle

func _ready() -> void:
	var rules: Array[Rule] = [
		Rule.new(1, 1, 2, 45, true),
		Rule.new(1, 1, 2, 32, false),
		Rule.new(0, 2, 3, 12, true),
		Rule.new(2, 0, 3, 12, false),
		Rule.new(0, 3, 3, 10, true),
		Rule.new(3, 0, 3, 11, false),
	]
	
	puzzle = Puzzle.new(4, 4, rules)
	%BackgroundLayer.draw_puzzle(puzzle)
	%Rules.draw_puzzle(puzzle)
