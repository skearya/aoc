import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

const num_to_word = [
  #("zero", 0),
  #("one", 1),
  #("two", 2),
  #("three", 3),
  #("four", 4),
  #("five", 5),
  #("six", 6),
  #("seven", 7),
  #("eight", 8),
  #("nine", 9),
  #("0", 0),
  #("1", 1),
  #("2", 2),
  #("3", 3),
  #("4", 4),
  #("5", 5),
  #("6", 6),
  #("7", 7),
  #("8", 8),
  #("9", 9),
]

pub fn main() {
  let assert Ok(input) = simplifile.read("../input.txt")

  let sum =
    input
    |> string.trim_right
    |> string.split("\n")
    |> list.map(parse(_, []))
    |> list.map(fn(nums) {
      let assert [Ok(first), Ok(last)] = [list.first(nums), list.last(nums)]

      first * 10 + last
    })
    |> int.sum

  io.println("sum: " <> int.to_string(sum))
}

fn parse(line: String, collected: List(Int)) -> List(Int) {
  case string.length(line) == 0 {
    True -> list.reverse(collected)
    False -> {
      let found =
        list.find(num_to_word, fn(pair) { string.starts_with(line, pair.0) })

      case found {
        Ok(pair) -> parse(string.drop_left(line, 1), [pair.1, ..collected])
        Error(Nil) -> parse(string.drop_left(line, 1), collected)
      }
    }
  }
}
