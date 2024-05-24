import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(input) = simplifile.read("../input.txt")

  let sum =
    input
    |> string.trim_right
    |> string.split("\n")
    |> list.map(fn(line) {
      let nums =
        line
        |> string.to_graphemes
        |> list.filter_map(int.parse)

      let assert [Ok(first), Ok(last)] = [list.first(nums), list.last(nums)]

      first * 10 + last
    })
    |> int.sum

  io.println("sum: " <> int.to_string(sum))
}
