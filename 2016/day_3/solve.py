import re
from typing import Iterable
from more_itertools import grouper
from itertools import chain

triangle = tuple[int, int, int]


def main() -> None:
    with open("input.txt") as file:
        lines = file.readlines()
    numbers = map(parse_line, iter(lines))
    # Part 1
    print(verify_triangle(numbers))

    assert len(lines) % 3 == 0
    numbers_grouped = chain.from_iterable(
        map(
            lambda x: [
                (x[0][0], x[1][0], x[2][0]),
                (x[0][1], x[1][1], x[2][1]),
                (x[0][2], x[1][2], x[2][2]),
            ],
            grouper(map(parse_line, iter(lines)), 3),
        )
    )
    # Part 2
    print(verify_triangle(numbers_grouped))


def parse_line(line: str) -> triangle:
    parser = re.compile(r"^\s*(\d+)\s*(\d+)\s*(\d+)")
    res = parser.match(line)
    if res is None:
        raise ValueError("Unable to parse string")
    return (int(res.group(1)), int(res.group(2)), int(res.group(3)))


def verify_triangle(triangles: Iterable[triangle]) -> int:
    total = 0
    for tringle in triangles:
        sides = sorted(tringle)
        # print(sides)
        if sides[0] + sides[1] > sides[2]:
            total += 1

    return total


if __name__ == "__main__":
    main()
