from typing import Collection, Iterable, cast
from itertools import pairwise
from more_itertools import windowed


def main() -> None:
    with open("input.txt") as f:
        lines = list(map(int, f.readlines()))

    print(task_one(lines))
    print(task_two(lines))


def task_one(lines: Iterable[int]) -> int:
    total: int = 0
    for (prev, cur) in pairwise(lines):
        if cur > prev:
            total += 1

    return total


def task_two(lines: Collection[int]) -> int:
    total: int = 0
    assert len(lines) >= 3
    grouped = map(
        lambda win: cast(int, win[0]) + cast(int, win[1]) + cast(int, win[2]),
        windowed(lines, 3),
    )
    for (prev, cur) in pairwise(grouped):
        if cur > prev:
            total += 1

    return total


if __name__ == "__main__":
    main()
