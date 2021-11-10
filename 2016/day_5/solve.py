import sys, hashlib

def main() -> None:
    assert(len(sys.argv) >= 2)
    print(gen_password(sys.argv[1], 5, 8))
    print(gen_v2_password(sys.argv[1], 5, 8))

def gen_password(start: str, num_zeroes: int, pass_len: int) -> str:
    start_bytes = start.encode()
    index = 0
    cur = str()
    while len(cur) < pass_len:
        h = hashlib.md5()
        h.update(start_bytes + str(index).encode())
        hex = h.hexdigest()
        if hex.startswith("0" * num_zeroes):
            # Found one!
            cur += hex[5]
        index += 1

    return cur

def gen_v2_password(start: str, num_zeroes: int, pass_len: int) -> str:
    start_bytes = start.encode()
    positions_needed = set(range(pass_len))
    cur = list("*" * pass_len)
    index = 0
    while len(positions_needed) > 0:
        h = hashlib.md5()
        h.update(start_bytes + str(index).encode())
        hex = h.hexdigest()
        if hex.startswith("0" * num_zeroes):
            # Maybe found one!
            try:
                pos = int(hex[5])
                if pos in positions_needed:
                    positions_needed.remove(pos)
                    cur[pos] = hex[6]
                    print("".join(cur))
            except ValueError:
                pass
        index += 1
    return "".join(cur)

if __name__ == "__main__":
    main()
