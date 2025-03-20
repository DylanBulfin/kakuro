extends Object
class_name Puzzle

var width: int = 10
var height: int = 10

var rules: Array[Rule]

var columns: Array[Array]
var rows: Array[Array]

func _init(width_: int, height_: int, rules_: Array[Rule]):
	width = width_
	height = height_
	
	rules = rules_
	
	generate_cells()
	
func generate_cells():
	for x in range(width):
		rows.append([])
		for _y in range(height):
			rows[x].append(null)
			
	for rule in rules:
		# Set up rule cell
		if not rows[rule.row][rule.column]:
			rows[rule.row][rule.column] = Cell.new(false)
		rows[rule.row][rule.column].rules.append(rule)

		
		for i in range(1, rule.num_cells + 1):
			var coords: Vector2i
			if rule.is_vertical:
				coords = Vector2i(rule.row + i, rule.column)
			else:
				coords = Vector2i(rule.row, rule.column + i)
		
			if not rows[coords.x][coords.y]:
				# create cell
				rows[coords.x][coords.y] = Cell.new(true)
			
			rows[coords.x][coords.y].rules.append(rule)

	# Fill in any uninitialized cells
	for row: Array in rows:
		for i: int in range(len(row)):
			if row[i] == null:
				row[i] = Cell.new(false)

	# Set up columns as transpose of rows
	for y in range(height):
		columns.append([])
		for x in range(width):
			columns[y].append(rows[x][y])
			
