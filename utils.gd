extends Node

func get_tile_coordinates(tile: Vector2i) -> Vector2:
	return Vector2(tile) * 32

func get_tile_coordinates_alt(x: int, y: int) -> Vector2:
	return Vector2(x, y) * 32
