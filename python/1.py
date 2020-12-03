from pathlib import Path

target = 2020

text = Path("../input1.txt").read_text()

numbers = []
for line in text.split("\n"):
    if not line:
        continue
    
    number = int(line)
    if number >= target:
        continue

    numbers.append(number)

numbers = sorted(numbers)

def f(numbers, target):
    for i in numbers:
        for j in reversed(numbers):
            s = i + j
            if s < target:
                break

            if s == target:
                return i, j

    return None


def part1():
    result = f(numbers, target)
    if result is None:
        print("No solution")
    else:
        i, j = result
        print(i * j)


def part2():
    for i, number in enumerate(numbers):
        result = f(numbers[i + 1:], target - number)
        if result:
            x, y = result
            print(x * y * number)
            return

    print("No solution")


part1()
part2()
