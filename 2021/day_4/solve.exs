defmodule Bingo do
  @moduledoc """
  Provides a set of functions useful for managing bingo games
  """
  defstruct all_numbers: MapSet.new(), winning_sets: []

  @doc """
  Takes an enumerator of strings and turns them into a bingo board
  """
  def parse_board(lines) do
    rows = Enum.map(lines, &Bingo.to_numbers(&1, " "))
    last_index = length(rows) - 1
    all_points = 0..last_index

    columns =
      for i <- all_points do
        for j <- all_points, into: %MapSet{}, do: Enum.at(rows, j) |> Enum.at(i)
      end

    row_sets = Stream.map(rows, &MapSet.new/1)

    # Using a list instead as I can't find a guarantee that all numbers are unique
    # all_numbers = MapSet.new(Stream.concat(rows))
    all_numbers = Enum.concat(rows)

    %Bingo{all_numbers: all_numbers, winning_sets: Enum.concat([row_sets, columns])}
  end

  @doc """
  Takes a string containing a list of numbers, and a value to split the string by, and then splits the string and its contents
  """
  def to_numbers(string, splitter) do
    String.split(string, splitter, trim: true)
    |> Stream.map(&String.trim/1)
    |> Enum.map(&String.to_integer/1)
  end

  @doc """
  Takes a list of boards, and a list of numbers to draw, in order, and then simualates a game. Returns the score of the winner.
  """
  def get_winner(boards, numbers) do
    # Iterating through the numbers, building a set, and seeing if it was a winner for each game
    Enum.reduce_while(numbers, MapSet.new(), fn next, acc ->
      selected = MapSet.put(acc, next)

      winner =
        Enum.find(boards, fn board ->
          Enum.any?(board.winning_sets, fn set ->
            MapSet.subset?(set, selected)
          end)
        end)

      # Checking if a winner or not
      if is_nil(winner) do
        # No winner
        {:cont, selected}
      else
        # Winner!
        {:halt, get_points(winner, selected) * next}
      end
    end)
  end

  @doc """
  Takes a list of boards, and a list of numbers to draw, in order, and then simualates a game. Returns the score of the last board to win.
  """
  def get_loser(boards, numbers) do
    # Iterating through the numbers, building a set, and seeing if it was a winner for each game
    Enum.reduce(numbers, {MapSet.new(), nil, boards}, fn next, {acc, last_win, boards} ->
      selected = MapSet.put(acc, next)

      winner =
        Enum.filter(boards, fn board ->
          Enum.any?(board.winning_sets, fn set ->
            MapSet.subset?(set, selected)
          end)
        end)

      # Checking if a winner or not
      if length(winner) == 0 do
        # No winner
        {selected, last_win, boards}
      else
        {points, boards} =
          Enum.reduce(winner, {nil, boards}, fn next, {lowest_score, boards} ->
            boards = List.delete(boards, next)
            this_score = get_points(next, selected)

            lowest_score =
              if is_nil(lowest_score) || lowest_score > this_score do
                this_score
              else
                lowest_score
              end

            {lowest_score, boards}
          end)

        # Winner!
        {selected, points * next, boards}
      end
    end)
    |> elem(1)
  end

  @doc """
  Returns the number of points a board would have won, by adding all numbers that aren't part of numbers
  """
  def get_points(board, numbers) do
    Enum.sum(board.all_numbers -- Enum.to_list(numbers))
  end
end

numbers = File.stream!("input.txt")

to_select =
  Enum.at(numbers, 0)
  |> then(&Bingo.to_numbers(&1, ","))

# IO.inspect(to_select)

boards =
  numbers
  |> Stream.drop(2)
  |> Stream.chunk_every(5, 6, :discard)
  |> Enum.map(&Bingo.parse_board/1)

# IO.inspect(boards)

Bingo.get_winner(boards, to_select)
|> IO.puts()

Bingo.get_loser(boards, to_select)
|> IO.puts()
