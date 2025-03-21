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
		if not rows[rule.y][rule.x]:
			rows[rule.y][rule.x] = Cell.new(false, rule.x, rule.y)
		rows[rule.y][rule.x].rules.append(rule)

		
		for i in range(1, rule.num_cells + 1):
			var coords: Vector2i
			if rule.is_vertical:
				coords = Vector2i(rule.x, rule.y + i)
			else:
				coords = Vector2i(rule.x + i, rule.y)
		
			if not rows[coords.y][coords.x]:
				# create cell
				rows[coords.y][coords.x] = Cell.new(true, coords.x, coords.y)
	
			# Add rule to cell
			rows[coords.y][coords.x].rules.append(rule)
			
			# Add cell to rule
			rule.cells.append(rows[coords.y][coords.x])

	# Fill in any uninitialized cells
	for y: int in range(height):
		for x: int in range(width):
			var row = rows[y]
			if row[x] == null:
				row[x] = Cell.new(false, x, y)

func get_cell(x: int, y: int) -> Cell:
	return rows[y][x]

func get_cellv(v: Vector2i) -> Cell:
	return rows[v.y][v.x]
