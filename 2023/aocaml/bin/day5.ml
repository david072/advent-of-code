open Core
(* open Format *)

type conversion = {
  dst_start : int;
  src_start : int;
  length : int;
}
[@@deriving show]

type map = {
  src : string;
  dst : string;
  conversions : conversion list;
}
[@@deriving show]

type range = {
  start : int;
  length : int;
}
[@@deriving show]

let convert_number map n = match List.find map.conversions ~f:(fun {src_start; length; _} -> n >= src_start && n < src_start + length) with
  | Some {src_start; dst_start; _} -> (n - src_start) + dst_start
  | None -> n
  
let parse_seeds line =
  let parts = String.split line ~on:' ' in
  List.drop parts 1 |> List.map ~f:int_of_string

let parse_seed_ranges line =
  let numbers = parse_seeds line in
  let rec iter = function
    | start :: length :: rest -> {start; length} :: iter rest
    | [] -> []
    | _ -> raise (Failure "i don't know what to put here")
  in
  iter numbers

let parse_maps lines =
  let rec parse_conversions = function
    | [] -> ([], [])
    | str :: rest when String.is_empty str -> ([], rest)
    | line :: rest -> 
      let parts = String.split ~on:' ' line |> List.filter ~f:(fun s -> not @@ String.is_empty s) |> List.map ~f:int_of_string in
      let dst_start = List.nth_exn parts 0 in
      let src_start = List.nth_exn parts 1 in
      let length = List.nth_exn parts 2 in
      let (convs, rest) = parse_conversions rest in
      ({dst_start; src_start; length} :: convs, rest)
  in

  let rec parse_maps lines =
    let lines = List.drop_while lines ~f:String.is_empty in
    match lines with
    | hd :: rest -> begin
      let rex = Pcre.regexp {|(\w+)-to-(\w+) map:|} in
      let parts = Pcre.exec ~rex hd |> Pcre.get_substrings in
      let src = parts.(1) in
      let dst = parts.(2) in
      let (conversions, rest) = parse_conversions rest in
      {src; dst; conversions} :: parse_maps rest
    end
    | [] -> []
  in
  parse_maps lines

let convert_to_loc maps seed_num =
  List.fold maps ~init:seed_num ~f:(fun acc map -> convert_number map acc)

let solve1 input =
  let seeds = parse_seeds @@ List.hd_exn input in
  let maps = parse_maps @@ List.drop input 2 in
  let locations = List.map seeds ~f:(fun n -> convert_to_loc maps n) in
  match List.min_elt locations ~compare:compare with 
  | Some res -> res
  | None -> raise (Failure "waddafuck?")

let solve2 input = 
  let seeds = parse_seed_ranges @@ List.hd_exn input in
  let maps = parse_maps @@ List.drop input 2 in
  let locations = Stdlib.List.flatten @@ List.map seeds ~f:(fun {start; length} -> List.range start (start + length - 1) |> List.map ~f:(fun n -> convert_to_loc maps n)) in
  match List.min_elt locations ~compare:compare with 
  | Some res -> res
  | None -> raise (Failure "waddafuck?")

