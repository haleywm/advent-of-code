# Read and parse the input into a list of sorted ints
# Sorting only necessary for an optimised part 1 solution as I couldn't find a non-bruteforce way to solve part 2
input =
  File.read!("input.txt")
  |> String.trim()
  |> String.split(",")
  |> Stream.map(&String.to_integer/1)
  |> Enum.sort()

# Getting the middle element as that'll have the shortest distance
middle = div(length(input), 2)
pos = Enum.at(input, middle)

Stream.map(input, &abs(pos - &1))
|> Enum.sum()
|> IO.puts()

get_dist = fn a, b ->
  linear = abs(a - b)
  div(linear * (linear + 1), 2)
end

{min, max} = Enum.min_max(input)

Stream.map(min..max, fn point ->
  Stream.map(input, &get_dist.(&1, point))
  |> Enum.sum()
end)
|> Enum.min()
|> IO.puts()
