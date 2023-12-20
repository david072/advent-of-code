open Core

module type Day = sig
  val solve1 : string list -> int
  val solve2 : string list -> int
end

let print_solutions day (module D: Day) = 
  let str_day = Int.to_string day in
  let lines = In_channel.read_lines ("day" ^ str_day ^ "_input.txt") in
  printf "Day %i:\n" day;
  printf "- Part One: %i\n" @@ D.solve1 lines;
  printf "- Part Two: %i\n" @@ D.solve2 lines

let () = 
  (* print_solutions 1 (module Day1); *)
  (* print_solutions 2 (module Day2); *)
  (* print_solutions 3 (module Day3); *)
  (* print_solutions 4 (module Day4); *)
  (* print_solutions 5 (module Day5); *)
  (* print_solutions 6 (module Day6); *)
  (* print_solutions 7 (module Day7); *)
  (* print_solutions 8 (module Day8); *)
  (* print_solutions 9 (module Day9); *)
  (* print_solutions 10 (module Day10); *)
  (* print_solutions 11 (module Day11); *)
  (* print_solutions 12 (module Day12); *)
  (* print_solutions 13 (module Day13); *)
  (* print_solutions 15 (module Day15); *)
  print_solutions 19 (module Day19);
  (* print_solutions 20 (module Day20); *)
