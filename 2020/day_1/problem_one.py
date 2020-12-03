def main():
    numbers = number_iter("input.txt")
    answer = solve_problem(numbers)
    print(answer)

def number_iter(filename):
    # Returns an iterator of ints in a file containing new line separated ints
    with open(filename) as file:
        lines = file.readlines()
    return map(lambda x: int(x), lines)

def solve_problem(numbers, total=2020):
    # Iterates through numbers, attempts to find a match that adds to sum, and then returns the multiple of those numbers
    # Using a dictionary method because that seemed neat
    seen = dict()
    for num in numbers:
        seen[num] = 0
        if (total - num) in seen:
            # Found a match
            return (total - num) * num
    # Didn't find anything
    return -1

if __name__ == "__main__":
    main()