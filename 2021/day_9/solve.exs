tiles =
  File.read!("input.txt")
  |> String.split("\n", trim: true)
  |> Stream.with_index()
  |> Stream.flat_map(fn {line, x} ->
    String.codepoints(line)
    |> Stream.with_index()
    |> Enum.map(fn {char, y} ->
      num = String.to_integer(char)
      {{x, y}, num}
    end)
  end)
  |> Enum.into(%{})

low_points =
  Enum.filter(tiles, fn {{x, y}, cur_height} ->
    # Filter out any points that aren't low points
    offsets = [{-1, 0}, {1, 0}, {0, -1}, {0, 1}]

    Enum.all?(offsets, fn {x_o, y_o} ->
      # Getting with a default of 9, so that if it's not in the grid it'll be max height
      Map.get(tiles, {x + x_o, y + y_o}, 9) > cur_height
    end)
  end)

# Part 1
# Take all low points, add 1 to their height, and sum
Stream.map(low_points, fn {_, num} -> num + 1 end)
|> Enum.sum()
|> IO.puts()

# Part 2
# Take all low points, and recursively build a set of adjacent tiles that are less than 9
defmodule Basin do
  def get_basin(x, y, points, tiles) do
    offsets =
      Enum.map([{-1, 0}, {1, 0}, {0, -1}, {0, 1}], fn {off_x, off_y} -> {off_x + x, off_y + y} end)

    new_points =
      Enum.reduce(offsets, points, fn cur, cur_points ->
        if Map.get(tiles, cur, 9) < 9 do
          MapSet.put(cur_points, cur)
        else
          cur_points
        end
      end)

    Enum.reduce(offsets, new_points, fn cur, cur_points ->
      if cur in new_points and cur not in points do
        {x, y} = cur
        get_basin(x, y, cur_points, tiles)
      else
        cur_points
      end
    end)
  end
end

Stream.map(low_points, fn {{x, y}, _} ->
  # For each low point, map to a basin, and return it's size
  MapSet.size(Basin.get_basin(x, y, MapSet.new(), tiles))
end)
|> Enum.sort()
|> Enum.take(-3)
|> then(fn [a, b, c] -> a * b * c end)
|> IO.puts()
