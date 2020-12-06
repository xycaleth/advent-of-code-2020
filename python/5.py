from util import read_lines

def get_seat_id(encoded_seat):
    max_row = 127
    min_row = 0

    max_col = 7
    min_col = 0

    for c in encoded_seat:
        if c == "F":
            max_row = min_row + (max_row - min_row) // 2
        elif c == "B":
            min_row += (max_row - min_row + 1) // 2
        elif c == "L":
            max_col = min_col + (max_col - min_col) // 2
        elif c == "R":
            min_col += (max_col - min_col + 1) // 2

    return min_row * 8 + min_col


def part1():
    highest_seat_id = 0
    for line in read_lines("../input5.txt"):
        if not line:
            break

        highest_seat_id = max(highest_seat_id, get_seat_id(line))

    print(highest_seat_id)


def part2():
    seat_ids = []
    for line in read_lines("../input5.txt"):
        if not line:
            break

        seat_ids.append(get_seat_id(line))

    seat_ids.sort()

    last_seat_id = seat_ids.pop(0)
    for seat_id in seat_ids:
        expected_id = last_seat_id + 1
        if expected_id != seat_id:
            break

        last_seat_id = expected_id

    print(expected_id)


part1()
part2()
