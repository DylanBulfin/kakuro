extends Node

func get_tile_coordinates(tile: Vector2i) -> Vector2:
	return Vector2(tile) * Const.PX_PER_TILE

func get_tile_coordinates_alt(x: int, y: int) -> Vector2:
	return Vector2(x, y) * Const.PX_PER_TILE
