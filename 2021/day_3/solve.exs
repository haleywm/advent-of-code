import Bitwise

numbers =
  File.read!("input.txt")
  |> String.split("\n", trim: true)

defmodule Shared do
  def bti(true), do: 1
  def bti(false), do: 0

  def count_binary([next | tail], pos) do
    String.to_integer(String.at(next, pos), 2) + count_binary(tail, pos)
  end

  def count_binary([], _pos), do: 0
end

defmodule P1 do
  def calculate(list) do
    # Assuming every item in the list is the same length
    last = String.length(hd(list)) - 1

    gamma =
      0..last
      |> Stream.map(fn pos -> Shared.count_binary(list, pos) end)
      |> Enum.reduce(0, fn next, acc ->
        next = Shared.bti(next * 2 >= length(list))
        (acc <<< 1) + next
      end)

    epsilon = ~~~gamma &&& Integer.pow(2, last) - 1

    # IO.inspect([gamma, epsilon])
    gamma * epsilon
  end
end

defmodule P2 do
  def calculate(list) do
    oxygen =
      reduce(list, true)
      |> String.to_integer(2)

    scrubber =
      reduce(list, false)
      |> String.to_integer(2)

    # IO.inspect([oxygen, scrubber])
    oxygen * scrubber
  end

  defp reduce(list, most_common, reduced \\ "")

  defp reduce(list, most_common, reduced) when length(list) > 1 do
    # Find the most or least common number, and filter values that don't match
    pos = String.length(reduced)
    ones = Shared.count_binary(list, pos)

    chosen =
      Shared.bti(ones * 2 >= length(list) == most_common)
      |> Integer.to_string()

    reduced = String.replace_suffix(reduced, "", chosen)
    reduce(Enum.filter(list, &String.starts_with?(&1, reduced)), most_common, reduced)
  end

  defp reduce(list, _most_common, _reduced) when length(list) == 1 do
    hd(list)
  end
end

P1.calculate(numbers)
|> IO.puts()

P2.calculate(numbers)
|> IO.puts()
