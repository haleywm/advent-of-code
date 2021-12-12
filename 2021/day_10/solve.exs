input =
  File.read!("input.txt")
  |> String.split("\n", trim: true)

defmodule SyntaxParser do
  @doc """
  Return the score for an invalid line
  0 if the lines issue is incompleteness
  3 if the line's first illegal character is )
  57 if ]
  1197 if }
  25137 if >
  """
  def get_invalid_score(line) do
    find_invalid([], String.codepoints(line))
  end

  defp find_invalid(cur, [next | tail]) when next in ["(", "[", "{", "<"] do
    # Add tail to the stack
    find_invalid([next | cur], tail)
  end

  defp find_invalid([needed | stack], [next | tail])
       when (needed == "(" and next == ")") or (needed == "[" and next == "]") or
              (needed == "{" and next == "}") or (needed == "<" and next == ">") do
    # Found match, can pop both sides off stack
    find_invalid(stack, tail)
  end

  defp find_invalid(_stack, [next | _tail]) do
    # More items are available, but aren't valid
    case next do
      ")" -> 3
      "]" -> 57
      "}" -> 1197
      ">" -> 25137
    end
  end

  defp find_invalid(_stack, []) do
    # Ran out of items to parse, incomplete string
    0
  end

  def get_incomplete_score(line) do
    find_incomplete([], String.codepoints(line))
  end

  defp find_incomplete(cur, [next | tail]) when next in ["(", "[", "{", "<"] do
    # Add tail to the stack
    find_incomplete([next | cur], tail)
  end

  defp find_incomplete([needed | stack], [next | tail])
       when (needed == "(" and next == ")") or (needed == "[" and next == "]") or
              (needed == "{" and next == "}") or (needed == "<" and next == ">") do
    # Found match, can pop both sides off stack
    find_incomplete(stack, tail)
  end

  defp find_incomplete(_stack, [_next | _tail]) do
    # More items are available, but aren't valid
    0
  end

  defp find_incomplete(stack, []) do
    # Ran out of items to parse, incomplete string
    get_stack_score(0, stack)
  end

  defp get_stack_score(score, [next | tail]) do
    bonus =
      case next do
        "(" -> 1
        "[" -> 2
        "{" -> 3
        "<" -> 4
      end

    get_stack_score(score * 5 + bonus, tail)
  end

  defp get_stack_score(score, []) do
    score
  end
end

# Part 1
Stream.map(input, &SyntaxParser.get_invalid_score/1)
|> Enum.sum()
|> IO.puts()

# Part 2
Stream.map(input, &SyntaxParser.get_incomplete_score/1)
|> Stream.filter(fn x -> x != 0 end)
|> Enum.sort()
|> then(fn list ->
  mid = div(length(list), 2)
  Enum.at(list, mid)
end)
|> IO.puts()
