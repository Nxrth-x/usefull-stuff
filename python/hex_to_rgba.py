def hex_to_rgba(hex_value: str) -> str:
	hex_value: str = hex_value.replace("#", "")
	assert len(hex_value) == 6
	values: list = [hex_value[i:i+2] for i in range(0, 6, 2)]
	for index, value in enumerate(values):
		values[index] = int(value, 16)
	return f"rgba({values[0]}, {values[1]}, {values[2]}, 1)"
