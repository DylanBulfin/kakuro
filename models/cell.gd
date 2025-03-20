extends Object
class_name Cell

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

func _init(fillable_: bool) -> void:
	fillable = fillable_
