parser = ~r/(up|down|forward) (\d+)/

instructions =
  File.read!("input.txt")
  |> String.split("\n", trim: true)
  |> Enum.map(fn x ->
    [_, instr, num] = Regex.run(parser, x)
    {String.to_atom(instr), String.to_integer(num)}
  end)

defmodule P1 do
  def move(list, pos \\ {0, 0})
  def move([{:forward, dist} | tail], {x, y}), do: move(tail, {x + dist, y})
  def move([{:up, dist} | tail], {x, y}), do: move(tail, {x, y - dist})
  def move([{:down, dist} | tail], {x, y}), do: move(tail, {x, y + dist})
  def move([], {x, y}), do: x * y
end

defmodule P2 do
  def move(list, pos \\ {0, 0}, aim \\ 0)

  def move([{:forward, dist} | tail], {x, y}, aim),
    do: move(tail, {x + dist, y + aim * dist}, aim)

  def move([{:up, dist} | tail], {x, y}, aim),
    do: move(tail, {x, y}, aim - dist)

  def move([{:down, dist} | tail], {x, y}, aim),
    do: move(tail, {x, y}, aim + dist)

  def move([], {x, y}, _), do: x * y
end

P1.move(instructions)
|> IO.puts()

P2.move(instructions)
|> IO.puts()
