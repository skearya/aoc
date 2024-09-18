{:ok, content} = File.read("../input.txt")

content
|> String.split("\n", trim: true)
|> Enum.map(&(String.split(&1, ": ") |> Enum.at(-1)))
|> Enum.map(fn numbers ->
  numbers
  |> String.split(" | ")
  |> Enum.map(fn numbers ->
    numbers |> String.split(" ", trim: true) |> Enum.map(&(Integer.parse(&1) |> elem(0)))
  end)
end)
|> Enum.map(fn [numbers, winning] ->
  Enum.filter(numbers, &Enum.member?(winning, &1)) |> length()
end)
|> Enum.map(fn num_winning ->
  if num_winning == 0, do: 0, else: Integer.pow(2, num_winning - 1)
end)
|> Enum.sum()
|> IO.inspect(label: "Answer")
