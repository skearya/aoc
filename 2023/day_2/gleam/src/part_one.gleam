import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(input) = simplifile.read("../input.txt")

  input
  |> string.trim_right
  |> string.split("\n")
  |> list.filter_map(fn(line) {
    let assert Ok(#(info, sets)) =
      line
      |> string.split_once(": ")

    let valid =
      sets
      |> string.split("; ")
      |> list.all(fn(set) {
        set
        |> string.split(", ")
        |> list.all(fn(subset) {
          let assert Ok(#(amount, color)) = string.split_once(subset, " ")
          let assert Ok(amount) = int.parse(amount)

          case color {
            "red" if amount <= 12 -> True
            "green" if amount <= 13 -> True
            "blue" if amount <= 14 -> True
            _ -> False
          }
        })
      })

    case valid {
      True -> {
        let game_num =
          info
          |> string.split_once(" ")
          |> result.try(fn(info) {
            info
            |> pair.second
            |> int.parse
          })

        game_num
      }
      False -> Error(Nil)
    }
  })
  |> int.sum
  |> int.to_string
  |> string.append("sum: ", _)
  |> io.println
}
