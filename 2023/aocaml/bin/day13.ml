open Core

type mirror =
  | Row of int
  | Column of int

let split_list_on list ~f =
  let rec split acc f = function
    | [] -> [List.rev acc]
    | x :: xs -> 
      if f x then (List.rev acc) :: split [] f xs 
      else split (x :: acc) f xs
  in
  split [] f list

let is_perfect_reflection pair_start map =
  let reflected_last = pair_start - ((List.length map - 1) - pair_start) + 1 in
  let reflected_first = (pair_start + 1) * 2 - 1 in
  let last_equal = reflected_last >= 0 && List.equal Char.equal (List.last_exn map) @@ List.nth_exn map reflected_last in
  let first_equal = reflected_first < List.length map && List.equal Char.equal (List.hd_exn map) @@ List.nth_exn map reflected_first in

  if not first_equal && not last_equal then false
  else
    let (before, after) = 
      if first_equal then (List.range 0 pair_start, List.range (pair_start + 2) @@ reflected_first + 1)
      else (List.range reflected_last pair_start, List.range (pair_start + 2) @@ List.length map) in
    let (before, after) = (List.map before ~f:(List.nth_exn map), List.map after ~f:(List.nth_exn map)) in
    List.equal (List.equal Char.equal) after @@ List.rev before

let count_equality_mistakes l1 l2 cmp =
  List.zip_exn l1 l2 |> List.filter_map ~f:(fun (el1, el2) -> if cmp el1 el2 then None else Some false) |> List.length

let is_perfect_reflection_with_smudge pair_start map =
  let reflected_last = pair_start - ((List.length map - 1) - pair_start) + 1 in
  let reflected_first = (pair_start + 1) * 2 - 1 in

  let last_mistakes = if reflected_last >= 0 then Some (count_equality_mistakes (List.last_exn map) (List.nth_exn map reflected_last) Char.equal) else None in
  let first_mistakes = if reflected_first < List.length map then Some (count_equality_mistakes (List.hd_exn map) (List.nth_exn map reflected_first) Char.equal) else None in

  let count_mistakes before after =
    let (before, after) = (List.map before ~f:(List.nth_exn map), List.map after ~f:(List.nth_exn map)) in
    List.zip_exn (List.rev before) after |> List.map ~f:(fun (el1, el2) -> count_equality_mistakes el1 el2 Char.equal) |> List.fold ~init:0 ~f:(+)
  in

  match (first_mistakes, last_mistakes) with
    | (Some n, None) when n <= 1 -> let (before, after) = (List.range 0 @@ pair_start + 1, List.range (pair_start + 1) @@ reflected_first + 1) in count_mistakes before after = 1
    | (None, Some n) when n <= 1 ->  let (before, after) = (List.range reflected_last @@ pair_start + 1, List.range (pair_start + 1) @@ List.length map) in count_mistakes before after = 1
    | _ -> false

let rec find_pairs = function
  | [] | _ :: [] -> []
  | (i, v) :: rest -> 
    let other = snd @@ List.hd_exn rest in
    if List.equal Char.equal v other then i :: find_pairs rest
    else find_pairs rest

let rec find_pairs_with_smudge = function
  | [] | _ :: [] -> []
  | (i, v) :: rest -> 
    let other = snd @@ List.hd_exn rest in
    if count_equality_mistakes v other Char.equal <= 1 then i :: find_pairs_with_smudge rest
    else find_pairs_with_smudge rest

let find_perfect_reflection map find_pairs validator =
  let indexed l = List.zip_exn (List.range 0 @@ List.length l) l in

  let find_perfect_mirror map = List.find (find_pairs @@ indexed map) ~f:(fun start -> validator start map) in
  let find_perfect_mirror_exn map = match find_perfect_mirror map with | Some v -> v | None -> raise (Failure "find_perfect_mirror") in

  match find_perfect_mirror map with
    | Some v -> Row v
  | _ -> Column (find_perfect_mirror_exn @@ List.transpose_exn map)

let solve1 input =
  let input = List.map input ~f:String.to_list in
  let maps = split_list_on input ~f:List.is_empty in
  let mirror_values = List.map maps ~f:(fun map -> match find_perfect_reflection map find_pairs is_perfect_reflection with | Column i -> i + 1 | Row i -> (i + 1) * 100) in
  List.fold mirror_values ~init:0 ~f:(+)

let solve2 input =
  let input = List.map input ~f:String.to_list in
  let maps = split_list_on input ~f:List.is_empty in
  let mirror_values = List.map maps ~f:(fun map -> match find_perfect_reflection map find_pairs_with_smudge is_perfect_reflection_with_smudge with | Column i -> i + 1 | Row i -> (i + 1) * 100) in
  List.fold mirror_values ~init:0 ~f:(+)
