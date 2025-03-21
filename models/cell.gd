extends Object
class_name Cell

var x: int
var y: int

var fillable: bool = false

# 0 indicates empty
var digit: int = 0

var rules: Array[Rule]

var notes: Array[bool] = [
	false, # 1 note active
	false, # 2 note active
	false, #...
	false,
	false,
	false,
	false,
	false,
	false
]

func _init(fillable_: bool, x: int, y: int) -> void:
	fillable = fillable_
