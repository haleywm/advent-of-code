import re, operator
from operator import itemgetter
from typing import Optional
from functools import reduce

ROOM_REG = re.compile(r"^([a-z\-]+)-(\d+)\[([a-z]{5})\]$")

def main() -> None:
    with open("input.txt") as file:
        rooms = file.readlines()
    
    total = reduce(
        operator.add,
        map(
            get_id,
            filter(is_valid, rooms)
        )
    )

    # Part 1
    print(total)

    # Part 2
    for valid in filter(is_valid, rooms):
        cypher_name = get_name(valid)
        room_id = get_id(valid)
        if cypher_name is None or room_id is None:
            continue
        name = decrypt_name(cypher_name, room_id)
        # Still lots of valid names
        if "north" in name:
            print(f"{name}: {room_id}")


def is_valid(room_name: str) -> bool:
    cap = ROOM_REG.match(room_name)
    if cap is None:
        return False
    letters: dict[str, int] = dict()
    for letter in cap.group(1):
        if letter != "-":
            if letter in letters:
                letters[letter] += 1
            else:
                letters[letter] = 1
    # Now to produce the checksum
    # Checksum orders first on commonness, then on alphabetisation
    full_checksum = sorted(letters.items(), key=itemgetter(0))
    full_checksum.sort(key=itemgetter(1), reverse=True)
    checksum = "".join([ i[0] for i in full_checksum[:5] ])

    return checksum == cap.group(3)

def get_name(room_name: str) -> Optional[str]:
    cap = ROOM_REG.match(room_name)
    if cap is None:
        return None
    return cap.group(1)

def get_id(room_name: str) -> Optional[int]:
    cap = ROOM_REG.match(room_name)
    if cap is None:
        return None
    try:
        return int(cap.group(2))
    except ValueError:
        return None

def decrypt_name(name: str, to_shift: int) -> str:
    output = str()
    for letter in name:
        raw = ord(letter)
        if raw == 45:
            # Dash -
            output += " "
        elif raw >= 97 and raw <= 122:
            # Ascii lowercase
            raw -= 97
            raw += to_shift
            raw %= 26
            raw += 97
            output += chr(raw)
        else:
            raise ValueError("Only lowercase ascii and dashes are supported")
    
    return output

if __name__ == "__main__":
    main()
