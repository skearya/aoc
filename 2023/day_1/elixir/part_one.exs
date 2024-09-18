{:ok, content} = File.read("../input.txt")

nums = Map.new(0..9, fn num -> {to_string(num), num} end)

content
|> String.split("\n")
|> Enum.map(&String.graphemes/1)
|> Enum.map(&Enum.filter(&1, fn grapheme -> Enum.member?(Map.keys(nums), grapheme) end))
|> Enum.map(&[nums[List.first(&1)] || 0, nums[List.last(&1)] || 0])
|> Enum.map(fn [first, last] -> 10 * first + last end)
|> Enum.sum()
|> IO.inspect(label: "Answer")
