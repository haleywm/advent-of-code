lines =
  File.read!("input.txt")
  |> String.split("\n", trim: true)
  |> Enum.map(&String.to_integer/1)

count_descending = fn x ->
  Stream.chunk_every(x, 2, 1, :discard)
  |> Enum.count(fn [prev, cur] -> cur > prev end)
end

# Task 1
lines
|> count_descending.()
|> IO.puts()

# Task 2
lines
|> Stream.chunk_every(3, 1, :discard)
|> Stream.map(&Enum.sum/1)
|> count_descending.()
|> IO.puts()
