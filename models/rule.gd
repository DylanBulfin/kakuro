extends Object
class_name Rule

# Represents one rule on the kakuro board

# row and column 0 are the part outside of the game area, for clues
var row: int = 0
var column: int = 0

var is_vertical: bool = false
var num_cells: int = 0

# The value all relevant cells are expected to add to
var value: int = 0

func _init(row_: int, column_: int, num_cells_: int, value_: int, is_vertical_: bool = false):
	row = row_
	column = column_
	num_cells = num_cells_
	value = value_
	is_vertical = is_vertical_
