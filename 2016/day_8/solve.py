import re
import numpy as np
from typing import Match, Optional

RECT_PARSE = re.compile(r"^rect (\d+)x(\d+)$")
ROT_PARSE = re.compile(r"^rotate (row y|column x)=(\d+) by (\d+)$")

WIDTH = 50
HEIGHT = 6

def main() -> None:
    grid = np.zeros((WIDTH, HEIGHT), bool)
    with open("input.txt") as f:
        lines = f.readlines()
        for line in lines:
            line = line.strip()
            # Checking if is rect
            cap: Optional[Match[str]] = RECT_PARSE.fullmatch(line)
            if cap:
                # It is rect!
                x = int(cap.group(1))
                y = int(cap.group(2))
                grid[0:x, 0:y] = True
            else:
                cap = ROT_PARSE.fullmatch(line)
                if cap:
                    # It is a rotation!
                    direction = cap.group(1)
                    if direction == "row y":
                        row = int(cap.group(2))
                        amount = int(cap.group(3))
                        grid[:, row] = np.roll(grid[:, row], amount)
                    elif direction == "column x":
                        column = int(cap.group(2))
                        amount = int(cap.group(3))
                        grid[column] = np.roll(grid[column], amount)
                    else:
                        raise ValueError("What")
                else:
                    # Couldn't match
                    raise ValueError(f"Couldn't parse line '{line}'")
    print(grid.sum())
    # Rendering as text
    for line in np.fliplr(np.rot90(grid, 3)):
        for char in line:
            if char:
                print("█", end="")
            else:
                print(" ", end="")
        print("")

if __name__ == "__main__":
    main()
