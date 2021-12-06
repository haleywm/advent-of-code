defmodule FishSim do
  # Simulating fish
  # Use a list that keeps track of how many fish are in each day of growth

  defp progress_list([hatching | tail], days) when days > 0 do
    new = List.insert_at(tail, 8, hatching)
    add = hatching + Enum.at(new, 6)
    progress_list(List.replace_at(new, 6, add), days - 1)
  end

  defp progress_list(list, _days) do
    Enum.sum(list)
  end

  @doc """
  Takes a list of days until fish first spawn, and a map of fish that contains at least until those days
  And returns a total
  """
  def get_fish_total(fish_list, days) do
    freq = Enum.frequencies(fish_list)
    fish = for n <- 0..8, do: Map.get(freq, n, 0)
    progress_list(fish, days)
  end
end

fish =
  File.read!("input.txt")
  |> String.split(",", trim: true)
  |> Stream.map(&String.trim/1)
  |> Enum.map(&String.to_integer/1)

target =
  Enum.at(System.argv(), 0, "80")
  |> String.to_integer()

FishSim.get_fish_total(fish, target)
|> IO.puts()
