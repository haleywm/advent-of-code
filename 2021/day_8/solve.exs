input =
  File.stream!("input.txt")
  |> Stream.map(&String.trim/1)
  |> Stream.map(fn x ->
    String.split(x, " | ")
    |> Stream.map(&String.split(&1, " ", trim: true))
    |> Enum.map(fn x ->
      Enum.map(x, fn str ->
        MapSet.new(String.codepoints(str))
      end)
    end)
  end)

# Part 1
Stream.map(input, fn line ->
  Enum.at(line, 1)
  |> Enum.count(&(MapSet.size(&1) in [2, 3, 4, 7]))
end)
|> Enum.sum()
|> IO.puts()

# Part 2

# Unique components numbers: 1, 4, 7 and 8, which have 2, 4, 3, and 7 components.
# After identifying these, identify the 6-components, 0, 6 and 9. 9 will be only one to contain all parts from 4. 0 will contain all parts from 1.
# Then, the 5 components, 2, 3, 5. 3 is only one to have all from 1. The points in both 6 and 9 will match 5, leaving 2.

calc_number_map = fn numbers ->
  # Takes a list of 10 unique MapSets containing numbers
  # Returns a Map mapping numbers 0 to 10 to mapsets

  # Mapping the easy ones
  number_map = %{
    1 => Enum.find(numbers, &(MapSet.size(&1) == 2)),
    4 => Enum.find(numbers, &(MapSet.size(&1) == 4)),
    7 => Enum.find(numbers, &(MapSet.size(&1) == 3)),
    8 => Enum.find(numbers, &(MapSet.size(&1) == 7))
  }

  # Nine
  number_map =
    Enum.find(numbers, fn x ->
      MapSet.size(x) == 6 && MapSet.subset?(number_map[4], x)
    end)
    |> then(&Map.put(number_map, 9, &1))

  # Zero
  number_map =
    Enum.find(numbers, fn x ->
      MapSet.size(x) == 6 && x != number_map[9] && MapSet.subset?(number_map[1], x)
    end)
    |> then(&Map.put(number_map, 0, &1))

  # Six
  number_map =
    Enum.find(numbers, fn x ->
      MapSet.size(x) == 6 && x != number_map[9] && x != number_map[0]
    end)
    |> then(&Map.put(number_map, 6, &1))

  # Three
  number_map =
    Enum.find(numbers, fn x ->
      MapSet.size(x) == 5 && MapSet.subset?(number_map[1], x)
    end)
    |> then(&Map.put(number_map, 3, &1))

  # Five
  number_map = Map.put(number_map, 5, MapSet.intersection(number_map[6], number_map[9]))

  # Two
  number_map =
    Enum.find(numbers, fn x ->
      MapSet.size(x) == 5 && x != number_map[3] && x != number_map[5]
    end)
    |> then(&Map.put(number_map, 2, &1))

  # Inverting the number map
  Map.new(number_map, fn {key, val} -> {val, key} end)
end

Stream.map(input, fn [numbers, output] ->
  number_map = calc_number_map.(numbers)

  Enum.reduce(output, 0, fn num, acc ->
    acc * 10 + Map.get(number_map, num)
  end)
end)
|> Enum.sum()
|> IO.puts()
