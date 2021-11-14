import re
from typing import Optional, Match

def main() -> None:
    with open("input.txt") as f:
        text = f.read().strip()

    print(task_one(text))
    print(task_two(text))

def task_one(input: str) -> int:
    # Takes an input, and returns the length of the parsed string
    marker = re.compile(r"\((\d+)x(\d+)\)")
    total: int = 0
    while True:
        # Finding next marker
        cap: Optional[Match[str]] = marker.search(input)
        if cap is None:
            # Reached last marker
            total += len(input)
            break
        marker_len = int(cap.group(1))
        times = int(cap.group(2))
        total += marker_len * times + cap.start()
        input = input[cap.end() + marker_len:]

    return total

def task_two(input: str) -> int:
    # Takes an input, and returns the length of the parsed string
    marker = re.compile(r"\((\d+)x(\d+)\)")
    total: int = 0
    while True:
        # Finding next marker
        cap: Optional[Match[str]] = marker.search(input)
        if cap is None:
            # Reached last marker
            total += len(input)
            break
        marker_len = int(cap.group(1))
        times = int(cap.group(2))
        total += task_two(input[cap.end() : cap.end() + marker_len]) * times + cap.start()
        input = input[cap.end() + marker_len:]

    return total

if __name__ == "__main__":
    main()
