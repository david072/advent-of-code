open Core

let parse_numbers str = String.split str ~on:' ' |> List.map ~f:int_of_string

let rec get_diff_list = function
  | [] | [_] -> []
  | first :: rest -> (List.hd_exn rest) - first :: get_diff_list rest

let rec extrapolate_forwards nums =
  if List.for_all nums ~f:((=) 0) then 0
  else List.last_exn nums + (extrapolate_forwards @@ get_diff_list nums)

let rec extrapolate_backwards nums =
  if List.for_all nums ~f:((=) 0) then 0
  else -extrapolate_backwards (get_diff_list nums) + List.hd_exn nums

let solve1 input =
  List.map input ~f:(fun l -> parse_numbers l |> extrapolate_forwards) |> List.fold ~init:0 ~f:(+)

let solve2 input =
  List.map input ~f:(fun l -> parse_numbers l |> extrapolate_backwards) |> List.fold ~init:0 ~f:(+)
