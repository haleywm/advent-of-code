defmodule FishSim do
  # Simulating fish
  # Couldn't find a cheaty mathematical model
  # But for optimisation will instead calculate a standard growth model for how a single fish would reprodce for each day
  # And use that to get the sums of fish

  # In this simplified model, day 1 starts with a fish with a score of 8, and goes from there
  @doc """
  Generates a map containing how many fish would result from a single fish, going from 0 to total_days
  """
  def gen_fish_map(total_days) do
    {_, map} =
      Enum.reduce(0..total_days, {[8], Map.new()}, fn day, {list, cur_map} ->
        {progress_list(list), Map.put(cur_map, day, length(list))}
      end)

    map
  end

  defp progress_list([0 | tail]) do
    [6 | [8 | progress_list(tail)]]
  end

  defp progress_list([num | tail]) do
    [num - 1 | progress_list(tail)]
  end

  defp progress_list([]) do
    []
  end

  @doc """
  Takes a list of days until fish first spawn, and a map of fish that contains at least until those days
  And returns a total
  """
  def get_fish_total(fish_list, fish_map, day) do
    Stream.map(fish_list, fn fish ->
      Map.get(fish_map, day + 8 - fish)
    end)
    |> Enum.sum()
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

highest_needed = 8 - Enum.min(fish) + target

map = FishSim.gen_fish_map(highest_needed)

FishSim.get_fish_total(fish, map, target)
|> IO.puts()
