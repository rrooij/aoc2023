f = open("input.txt", "r").readlines()
powers = []
for line in f:
    game_id = int(line.split(":")[0].replace("Game ", ""))
    game_line = line.split(": ")[1]
    tries = game_line.split(";")
    cube_info = {"green": [], "blue": [], "red": []}
    for try_ in tries:
        cubes_raw = try_.split(", ")
        for cube_raw in cubes_raw:
            cube = cube_raw.strip().split(" ")
            amount = int(cube[0])
            color = cube[1]
            cube_info[color].append(amount)
    for key in cube_info:
        cube_info[key] = max(cube_info[key])
    powers.append(cube_info["green"] * cube_info["blue"] * cube_info["red"])
print(sum(powers))
