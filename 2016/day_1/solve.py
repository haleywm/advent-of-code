from typing import Iterable, Optional

def main() -> None:
    with open("input.txt") as input:
        moves = input.read().split(", ")
    print(task_one(moves))
    print(task_two(moves))

def task_one(moves: Iterable[str]) -> int:
    facing = (1, 0) # (0, 1), (-1, 0), (0, -1)
    pos = [0, 0]
    for move in moves:
        turn = move[0]
        dist = int(move[1:])
        match turn:
            case "R":
                facing = (-facing[1], facing[0])
            case "L":
                facing = (facing[1], -facing[0])
            case _:
                raise ValueError("Invalid direction")
        
        pos[0] += dist * facing[0]
        pos[1] += dist * facing[1]

    return abs(pos[0]) + abs(pos[1])

def task_two(moves: Iterable[str]) -> Optional[int]:
    facing = (1, 0)
    pos = (0, 0)
    visited: set[tuple[int, int]] = set()
    visited.add(pos)
    for move in moves:
        turn = move[0]
        dist = int(move[1:])
        match turn:
            case "R":
                facing = (-facing[1], facing[0])
            case "L":
                facing = (facing[1], -facing[0])
            case _:
                raise ValueError("Invalid direction")
        
        for _ in range(1, dist + 1):
            pos = (pos[0] + facing[0], pos[1] + facing[1])
            if pos in visited:
                return abs(pos[0]) + abs(pos[1])
            visited.add(pos)
    # None found
    return None


if __name__ == "__main__":
    main()
