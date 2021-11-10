from typing import Iterable, Optional

def main() -> None:
    with open("input.txt") as input:
        moves = input.readlines()
    print(task_one(moves))
    print(task_two(moves))

def task_one(sequences: Iterable[Iterable[str]]) -> int:
    pos = [2, 2]
    total = 0
    LOW = 1
    HIGH = 3
    for moves in sequences:
        for move in moves.strip():
            match move:
                case "U":
                    pos[1] = max(LOW, pos[1] - 1)
                case "R":
                    pos[0] = min(HIGH, pos[0] + 1)
                case "D":
                    pos[1] = min(HIGH, pos[1] + 1)
                case "L":
                    pos[0] = max(LOW, pos[0] - 1)
                case _:
                    raise ValueError("Unrecognized instruction")
        # Numba time
        num = pos[0] + 3 * (pos[1] - 1)
        total = total * 10 + num
        
    return total

def task_two(sequences: Iterable[Iterable[str]]) -> str:
    key_map = [
        [None, None, "1", None, None],
        [None, "2", "3", "4", None],
        ["5", "6", "7", "8", "9"],
        [None, "A", "B", "C", None],
        [None, None, "D", None, None]
    ]
    pos = [2, 0]
    total = str()
    LOW = 0
    HIGH = 4
    for moves in sequences:
        for move in moves.strip():
            new_pos = pos.copy()
            match move:
                case "U":
                    new_pos[0] -= 1
                case "R":
                    new_pos[1] += 1
                case "D":
                    new_pos[0] += 1
                case "L":
                    new_pos[1] -= 1
                case _:
                    raise ValueError("Unrecognized instruction")
            if (
                new_pos[0] >= LOW
                and new_pos[0] <= HIGH
                and new_pos[1] >= LOW
                and new_pos[1] <= HIGH
                and key_map[new_pos[0]][new_pos[1]] is not None
            ):
                pos = new_pos
        # Numba time
        num = key_map[pos[0]][pos[1]]
        total += num
        
    return total

if __name__ == "__main__":
    main()
