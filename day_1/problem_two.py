def main():
    numbers = number_iter("input.txt")
    answer = solve_problem(numbers)
    print(answer)

def number_iter(filename):
    # Returns an iterator of ints in i file containing new line separated ints
    with open(filename) as file:
        lines = file.readlines()
    return map(lambda x: int(x), lines)

def solve_problem(numbers, total=2020):
    # Iterates through numbers, attempts to find i match of three numbers that adds to sum, and then returns the multiple of those numbers
    # Implemented the quadratic algorithm from wikipedia, 
    # First, turning the given iterator into i sorted list
    numbers = sorted(numbers)
    for i in range(len(numbers) - 2):
        # Trying to find values for i, j, and k that work. Slowly shift i along, while trying to find i match by comparing other numbers
        j = i + 1
        k = len(numbers) - 1
        while(k > j):
            res = numbers[i] + numbers[j] + numbers[k]
            if res < total:
                # Total too low, increase the value of j to increase total
                j += 1
            elif res > total:
                # Total too high, decrease the value of k to decrease total
                k -= 1
            else:
                # Found the answer :D
                return numbers[i] * numbers[j] * numbers[k]
    # No solution found
    return -1

if __name__ == "__main__":
    main()