def filter_numbers(line) -> [int]:
    return [x for x in line.split(".") if x.isnumeric()]


def check_number(char, char_idx, line) -> int:
    if not char.isnumeric():
        return 0
    line_last_index = len(line) - 1
    start_idx = char_idx
    end_idx = char_idx
    while start_idx != -1 and line[start_idx].isnumeric():
        start_idx = start_idx - 1
    while end_idx != line_last_index + 1 and line[end_idx].isnumeric():
        end_idx = end_idx + 1
    return int(line[start_idx + 1:end_idx])


def add_number(numbers, char, char_idx, line) -> int:
    number = check_number(char, char_idx, line)
    numbers.append(number)
    return 0 if number == 0 else 1


def number_found(numbers) -> int:
    return 1 if numbers[len(numbers) - 1] != 0 else 0


def main():
    lines = open('input.txt', 'r').readlines()
    symbols = {'#', '+', '@', '$', '*', '%', '=', '/', '&', '-'}
    gear_ratios = []
    numbers = []
    for idx, line in enumerate(lines):
        check_top = idx != 0
        check_bottom = idx != (len(lines) - 1)
        for char_idx, char in enumerate(line):
            check_left = char_idx != 0
            check_right = char_idx != (len(line) - 1)
            if char in symbols:
                found_bottom = False
                numbers_found = 0
                # Check top
                if check_top:
                    number_found = add_number(numbers, lines[idx - 1][char_idx], char_idx, lines[idx - 1])
                    numbers_found = numbers_found + number_found
                # Check diagonal left
                if check_top and check_left and number_found == 0:
                    numbers_found = numbers_found + add_number(numbers, lines[idx - 1][char_idx - 1], char_idx - 1, lines[idx - 1])
                # Check diagonal top right
                if check_top and check_right and number_found == 0:
                    numbers_found = numbers_found + add_number(numbers, lines[idx - 1][char_idx + 1], char_idx + 1, lines[idx - 1])
                # Check right
                if check_right:
                    numbers_found = numbers_found + add_number(numbers, lines[idx][char_idx + 1], char_idx + 1, lines[idx])
                # Check left
                if check_left:
                    numbers_found = numbers_found + add_number(numbers, lines[idx][char_idx - 1], char_idx - 1, lines[idx])
                # Check bottom
                if check_bottom:
                    number_found = add_number(numbers, lines[idx + 1][char_idx], char_idx, lines[idx + 1])
                    numbers_found = numbers_found + number_found
                    found_bottom = number_found != 0
                # Check diagonal bottom right
                if check_bottom and check_right and not found_bottom:
                    numbers_found = numbers_found + add_number(numbers, lines[idx + 1][char_idx + 1], char_idx + 1, lines[idx + 1])
                if check_bottom and check_left and not found_bottom:
                    numbers_found = numbers_found + add_number(numbers, lines[idx + 1][char_idx - 1], char_idx - 1, lines[idx + 1])
                if numbers_found == 2 and char == '*':
                    only_numbers = [x for x in numbers if x > 0]
                    gear_ratios.append(only_numbers[len(only_numbers) - 1] * only_numbers[len(only_numbers) - 2])
    print(f"Sum of numbers {sum(numbers)}")
    print(f"Sum of gear ratio {sum(gear_ratios)}")


if __name__ == '__main__':
    main()
