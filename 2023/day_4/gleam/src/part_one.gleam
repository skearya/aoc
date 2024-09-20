import gleam/float
import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(input) = simplifile.read("../input.txt")

  input
  |> string.trim
  |> string.split("\n")
  |> list.filter_map(string.split_once(_, ": "))
  |> list.map(fn(line) { line.1 })
  |> list.filter_map(string.split_once(_, " | "))
  |> list.map(fn(line) {
    let parse_numbers = fn(input) {
      input |> string.split(" ") |> list.filter_map(int.parse)
    }

    #(parse_numbers(line.0), parse_numbers(line.1))
  })
  |> list.map(fn(numbers) {
    list.filter(numbers.0, fn(number) { list.contains(numbers.1, number) })
  })
  |> list.map(list.length)
  |> list.filter_map(fn(winning) {
    case winning {
      0 -> Ok(0.0)
      winning -> int.power(2, int.to_float(winning - 1))
    }
  })
  |> float.sum
  |> io.debug
}
