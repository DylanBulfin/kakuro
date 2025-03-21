extends Object
class_name Rule

enum RuleState {
	Incomplete,
	Complete,
	Invalid
}

# Represents one rule on the kakuro board

# row and column 0 are the part outside of the game area, for clues
var row: int = 0
var column: int = 0

var is_vertical: bool = false
var num_cells: int
var cells: Array[Cell]

# The value all relevant cells are expected to add to
var value: int = 0

func _init(row_: int, column_: int, num_cells_: int, value_: int, is_vertical_: bool = false):
	row = row_
	column = column_
	num_cells = num_cells_
	value = value_
	is_vertical = is_vertical_

# Check if rule is invalidated
func validate_rule() -> RuleState:
	var sum: int = 0
	# Contains digits we've already seen, to prevent duplicates
	var digit_set: Dictionary = {}
	# Whether any of this rule's cells are empty
	var found_empty: bool = false
	for cell: Cell in cells:
		if cell.digit == 0: 
			# Found empty cell, note that and stop processing it
			found_empty = true
			continue
			
		sum += cell.digit
		if digit_set.has(cell.digit): 
			# Found a duplicate, invalid
			return RuleState.Invalid
			
		# Dummy value, so we can treat it as a set
		digit_set[cell.digit] = null
	
	if sum > value or (sum == value and found_empty):
		# If sum is too high, or is perfect with an empty cell, error
		return RuleState.Invalid
	elif sum < value or found_empty:
		# If there is an empty cell or sum is too low, it's valid but incomplete
		return RuleState.Incomplete
	else:
		# Otherwise it's complete
		return RuleState.Complete
