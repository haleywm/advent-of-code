import re
from typing import Optional, Match, cast
from collections import deque
from copy import deepcopy

VALUE_MATCH = re.compile(r"^value (\d+) goes to bot (\d+)$")
RULE_MATCH = re.compile(
    r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$"
)
Bot = tuple[Optional[int], Optional[int], tuple[int, bool, int, bool]]
FullBot = tuple[int, int, tuple[int, bool, int, bool]]
BotCollection = dict[int, Bot]


def main() -> None:
    with open("input.txt") as f:
        lines = f.readlines()

    # Each bot is defined by a tuple containing two optional ints, and a tuple containing two ints, the low bot, and the high bot
    # Using map so that I don't have to insert in order
    bots: BotCollection = dict()
    values: list[tuple[int, int]] = list()

    for line in lines:
        line = line.strip()
        # Check if value
        cap: Optional[Match[str]] = VALUE_MATCH.fullmatch(line)
        if cap:
            # Match! Extracting values for later
            val = int(cap.group(1))
            bot = int(cap.group(2))
            values.append((val, bot))
        else:
            # Check if rule
            cap = RULE_MATCH.fullmatch(line)
            if cap is None:
                raise ValueError(f"Unable to parse line '{line}'")
            bot_no = int(cap.group(1))
            low_output = cap.group(2) == "output"
            low_no = int(cap.group(3))
            high_output = cap.group(4) == "output"
            high_no = int(cap.group(5))

            bots[bot_no] = (None, None, (low_no, low_output, high_no, high_output))

    # Now adding values
    for (val, bot_id) in values:
        robot = bots[bot_id]
        if robot[0] is None:
            bots[bot_id] = (val, None, robot[2])
        else:
            if robot[1] is None:
                bots[bot_id] = (robot[0], val, robot[2])
            else:
                raise ValueError("Tried to give a robot more than 2 values")

    bots_two = deepcopy(bots)

    print(task_one(bots))
    print(task_two(bots_two))


def task_one(bots: BotCollection) -> int:
    # Simulate bots until one tries to compare 61 and 17, then return that bot id
    ready_bots: deque[int] = deque()
    output: dict[int, int] = dict()
    for bot_id, bot_vals in bots.items():
        if bot_vals[1] is not None and bot_vals[0] is not None:
            ready_bots.append(bot_id)

    if len(ready_bots) == 0:
        raise ValueError("No bot to start with")

    while len(ready_bots) > 0:
        cur_id = ready_bots.popleft()
        cur_bot = cast(FullBot, bots[cur_id])
        low_num, high_num = sorted([cur_bot[0], cur_bot[1]])
        if low_num == 17 and high_num == 61:
            return cur_id
        add_num(cur_bot[2][0], cur_bot[2][1], low_num, bots, ready_bots, output)
        add_num(cur_bot[2][2], cur_bot[2][3], high_num, bots, ready_bots, output)
    raise ValueError("Ran out of bots")


def task_two(bots: BotCollection) -> int:
    # Simulate bots until one tries to compare 61 and 17, then return that bot id
    ready_bots: deque[int] = deque()
    output: dict[int, int] = dict()
    for bot_id, bot_vals in bots.items():
        if bot_vals[1] is not None and bot_vals[0] is not None:
            ready_bots.append(bot_id)

    if len(ready_bots) == 0:
        raise ValueError("No bot to start with")

    while len(ready_bots) > 0:
        cur_id = ready_bots.popleft()
        cur_bot = cast(FullBot, bots[cur_id])
        low_num, high_num = sorted([cur_bot[0], cur_bot[1]])
        add_num(cur_bot[2][0], cur_bot[2][1], low_num, bots, ready_bots, output)
        add_num(cur_bot[2][2], cur_bot[2][3], high_num, bots, ready_bots, output)
        if 0 in output and 1 in output and 2 in output:
            return output[0] * output[1] * output[2]
    raise ValueError("Ran out of bots")


def add_num(
    to_add_id: int,
    is_output: bool,
    to_add: int,
    bots: BotCollection,
    ready_bots: deque[int],
    output: dict[int, int],
) -> None:
    if not is_output:
        add_bot = bots[to_add_id]
        if add_bot[0] is None:
            bots[to_add_id] = (to_add, None, add_bot[2])
        else:
            if add_bot[1] is None:
                bots[to_add_id] = (add_bot[0], to_add, add_bot[2])
                ready_bots.append(to_add_id)
            else:
                raise ValueError("Tried to give a robot more than 2 values")
    else:
        output[to_add_id] = to_add


if __name__ == "__main__":
    main()
