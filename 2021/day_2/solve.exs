parser = ~r/(up|down|forward) (\d+)/

instructions =
  File.read!("input.txt")
  |> String.split("\n", trim: true)
  |> Stream.map(fn x ->
    [_, instr, num] = Regex.run(parser, x)
    {String.to_atom(instr), String.to_integer(num)}
  end)
  |> Enum.map(fn
    {:forward, n} -> {n, 0}
    {:up, n} -> {0, -n}
    {:down, n} -> {0, n}
  end)

# Part 1
instructions
|> Enum.reduce(fn {x, y}, {curx, cury} -> {curx + x, cury + y} end)
|> Tuple.product()
|> IO.puts()

# Part 2
instructions
|> Enum.reduce({0, 0, 0}, fn {move, adjust}, {curx, cury, aim} ->
  aim = aim + adjust
  {curx + move, cury + move * aim, aim}
end)
|> then(fn {x, y, _} -> x * y end)
|> IO.puts()
