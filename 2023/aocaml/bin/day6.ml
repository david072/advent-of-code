open Core

let parse_numbers str = String.split str ~on:' ' |> List.tl_exn |> List.filter ~f:(fun el -> not @@ String.is_empty el) |> List.map ~f:int_of_string

(* The distance travelled can be seen as a function f with f(x) = x * (t - x),
   where x is the time the button is held for and t is the duration of the race.
   With that, we can solve the equation f(x) = d, where d is the *record distance + 1* 
   (since we want to be above the record distance). Solving this gives the quadratic
   formula x1,2 = (-t +- sqrt(t^2 - 4d)) / (-2), which produces the minimum and maximum
   possible times the button can be held for to still win. *)
let calculate_error time dist = 
  let root = sqrt @@ (Float.int_pow time 2) -. 4. *. (dist +. 1.) in
  let x1 = (-.time +. root) /. -2. in
  let x2 = (-.time -. root) /. -2. in
  let min = int_of_float @@ Float.round_up @@ Float.min x1 x2 in
  let max = int_of_float @@ Float.round_down @@ Float.max x1 x2 in
  max - min + 1

let solve1 input = 
  let times = parse_numbers @@ List.hd_exn input in
  let dists = parse_numbers @@ List.nth_exn input 1 in
  List.fold2_exn times dists ~init:1 ~f:(fun acc time dist -> acc * (calculate_error (float_of_int time) @@ float_of_int dist))

let solve2 input = 
  let parse line = parse_numbers line |> List.map ~f:string_of_int |> String.concat |> int_of_string in
  let time = parse @@ List.hd_exn input in
  let dist = parse @@ List.nth_exn input 1 in
  calculate_error (float_of_int time) @@ float_of_int dist
