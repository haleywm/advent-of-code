line_parse = ~r/^(\d+),(\d+) -> (\d+),(\d+)$/

defmodule Plotter do
  def is_straight({x_s, y_s, x_e, y_e}) do
    x_s == x_e || y_s == y_e
  end

  @doc """
  Gets the total number of squares that have more than 2 overlapping points
  """
  def get_overlaps(lines) do
    # Start with making a map that counts all points in lines
    all_points =
      Enum.reduce(lines, Map.new(), fn line, points ->
        line_points(line)
        |> Enum.reduce(points, fn pos, prev_points ->
          Map.update(prev_points, pos, 1, fn x -> x + 1 end)
        end)
      end)

    Map.values(all_points)
    |> Enum.count(fn x -> x > 1 end)
  end

  defp line_points({x_s, y_s, x_e, y_e}) do
    cond do
      x_s == x_e ->
        # Vertical
        for y <- y_s..y_e, do: {x_s, y}

      y_s == y_e ->
        # Horizontal
        for x <- x_s..x_e, do: {x, y_s}

      abs(x_e - x_s) == abs(y_e - y_s) ->
        # Diagonal
        x_pos = if x_s < x_e, do: 1, else: -1
        y_pos = if y_s < y_e, do: 1, else: -1
        for i <- 0..abs(x_e - x_s), do: {x_s + x_pos * i, y_s + y_pos * i}
    end
  end
end

points =
  File.stream!("input.txt")
  |> Stream.map(fn line ->
    [_ | tail] = Regex.run(line_parse, line)

    Enum.map(tail, &String.to_integer/1)
    |> List.to_tuple()
  end)

# Part 1
points
|> Stream.filter(&Plotter.is_straight/1)
|> Plotter.get_overlaps()
|> IO.puts()

# Part 2
points
|> Plotter.get_overlaps()
|> IO.puts()
