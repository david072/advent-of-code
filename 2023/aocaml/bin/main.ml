open Core

let print_solutions day solve1 solve2 = 
  let str_day = Int.to_string day in
  let lines = In_channel.read_lines ("day" ^ str_day ^ "_input.txt") in
  print_endline ("Day " ^ str_day ^ ":");
  print_endline ("- Part One: " ^ (Int.to_string (solve1 lines)));
  print_endline ("- Part Two: " ^ (Int.to_string (solve2 lines)))

let () = 
  (* print_solutions 1 Day1.solve1 Day1.solve2; *)
  (* print_solutions 2 Day2.solve1 Day2.solve2; *)
  (* print_solutions 3 Day3.solve1 Day3.solve2; *)
  (* print_solutions 4 Day4.solve1 Day4.solve2; *)
  (* print_solutions 5 Day5.solve1 Day5.solve2; *)
  (* print_solutions 6 Day6.solve1 Day6.solve2; *)
  (* print_solutions 7 Day7.solve1 Day7.solve2; *)
  print_solutions 8 Day8.solve1 Day8.solve2;
