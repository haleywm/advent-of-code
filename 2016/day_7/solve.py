import regex
from more_itertools import ilen

NOBRACKETS = regex.compile(r"([a-z])([a-z])\2\1(?![a-z]*\])")
BRACKETS = regex.compile(r"\[[a-z]*([a-z])([a-z])\2\1[a-z]*\]")
UNBRACKETED_ABA = regex.compile(r"([a-z])([a-z])\1(?![a-z]*\])")

def main() -> None:
    with open("input.txt") as f:
        lines = f.readlines()
    
    total_abba = ilen(
        filter(
            is_abba,
            iter(lines)
        )
    )

    print(total_abba)

    total_aba = ilen(
        filter(
            is_aba,
            iter(lines)
        )
    )

    print(total_aba)

def is_abba(line: str) -> bool:
    valid_unbracketed = False
    has_unbracketed = NOBRACKETS.finditer(line, overlapped=True)
    for cap in has_unbracketed:
        if cap.group(1) != cap.group(2):
            valid_unbracketed = True
            break
    if not valid_unbracketed:
        # Couldn't find a valid bracketed sequence
        return False
    
    valid_bracketed = False
    has_bracketed = BRACKETS.finditer(line, overlapped=True)
    for cap in has_bracketed:
        if cap.group(1) != cap.group(2):
            valid_bracketed = True
            break
    if valid_bracketed:
        # Found bracketed sequences
        return False
    # Has unbracketed and no bracketed
    return True

def is_aba(line: str) -> bool:
    for cap in UNBRACKETED_ABA.finditer(line, overlapped=True):
        # Going through every possible match
        if cap.group(1) == cap.group(2):
            # Not valid, must be different
            continue
        # Making a regex to look for a valid bracketed bab
        bab = regex.compile(r"\[[a-z]*" + f"{cap.group(2)}{cap.group(1)}{cap.group(2)}" + r"[a-z]*\]")
        if bab.search(line):
            # Found a match!
            return True
    # Found nothing
    return False

if __name__ == "__main__":
    main()
