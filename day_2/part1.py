f = open("input.txt", "r").readlines()
valid_games = []
for line in f:
    game_id = int(line.split(":")[0].replace("Game ", ""))
    game_line = line.split(": ")[1]
    tries = game_line.split(";")
    cube_info = {"green": 13, "blue": 14, "red": 12}
    valid = True
    for try_ in tries:
        cubes_raw = try_.split(", ")
        for cube_raw in cubes_raw:
            cube = cube_raw.strip().split(" ")
            amount = int(cube[0])
            color = cube[1]
            if amount > cube_info[color]:
                valid = False
    if valid:
        valid_games.append(game_id)
print(sum(valid_games))
