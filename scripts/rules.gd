extends Node2D

var frames: SpriteFrames = preload("res://assets/16x16rules_frames.aseprite")

func draw_puzzle() -> void:
	for x in range(State.puzzle.width):
		for y in range(State.puzzle.height):
			var cell: Cell = State.puzzle.rows[y][x]
			
			if cell.fillable:
				continue
			elif len(cell.rules) == 1:
				# Create a sprite for this rule
				var rule: Rule = cell.rules[0]
				var texture: Texture2D = frames.get_frame_texture("default", rule.value)
				var sprite: Sprite2D = Sprite2D.new()
				sprite.texture = texture
				
				var tile_pos: Vector2 = Utils.get_tile_coordinates_alt(x, y)
				var diff: Vector2 = Vector2(17, 22) if rule.is_vertical else Vector2(22, 17)
				sprite.position = tile_pos + diff
				
				%Rules.add_child(sprite)
			elif len(cell.rules) == 2:
				# Create 2 sprite for these rules
				var vrule: Rule = cell.rules[0] if cell.rules[0].is_vertical else cell.rules[1]
				var hrule: Rule = cell.rules[0] if not cell.rules[0].is_vertical else cell.rules[1]
				var vtexture: Texture2D = frames.get_frame_texture("default", vrule.value)
				var htexture: Texture2D = frames.get_frame_texture("default", hrule.value)
				var vsprite: Sprite2D = Sprite2D.new()
				var hsprite: Sprite2D = Sprite2D.new()
				vsprite.texture = vtexture
				hsprite.texture = htexture
				
				var tile_pos: Vector2 = Utils.get_tile_coordinates_alt(x, y)
				var vdiff: Vector2 = Vector2(10, 22)
				var hdiff: Vector2 = Vector2(23, 10)
				vsprite.position = tile_pos + vdiff
				hsprite.position = tile_pos + hdiff
				
				%Rules.add_child(vsprite)
				%Rules.add_child(hsprite)
