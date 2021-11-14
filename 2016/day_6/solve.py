from typing import Iterable
from more_itertools import minmax
from itertools import tee
from operator import itemgetter


def main() -> None:
    with open("input.txt") as f:
        print(find_codes(f.readlines()))


def find_codes(lines: Iterable[str]) -> tuple[str, str]:
    common_tracker: list[dict[str, int]] = list()
    # Building commonness map
    for line in lines:
        for (pos, letter) in enumerate(line.strip()):
            if len(common_tracker) == pos:
                common_tracker.append(dict())
            cur = common_tracker[pos]
            if letter in cur:
                cur[letter] += 1
            else:
                cur[letter] = 1

    # Then finding the most common letter for each value
    codes_a, codes_b = tee(map(get_top, common_tracker))
    return (
        "".join(map(lambda x: x[0], codes_a)),
        "".join(map(lambda x: x[1], codes_b)),
    )


def get_top(map: dict[str, int]) -> tuple[str, str]:
    versions = minmax(map.items(), key=itemgetter(1))
    return (versions[1][0], versions[0][0])


if __name__ == "__main__":
    main()
