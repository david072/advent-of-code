open Core

type status =
  | Operational
  | Damaged
  | Unknown
[@@deriving show, compare]

let is_unknown state = match state with | Unknown -> true | _ -> false

type row = {
  statuses : status list;
  damaged_groups : int list;
}
[@@deriving show]

let parse_input =
  let parse_line line =
    let parts = String.split line ~on:' ' in
    let statuses = List.map (String.to_list @@ List.hd_exn parts) ~f:(fun c -> 
      match c with 
        | '#' -> Damaged 
        | '.' -> Operational 
        | '?' -> Unknown
        | _ -> raise (Failure "no")
    ) in
    let damaged_groups = String.split (List.nth_exn parts 1) ~on:',' |> List.map ~f:int_of_string in
    { statuses; damaged_groups }
  in
  List.map ~f:parse_line

let seq_next_exn seq = match Sequence.next seq with | Some v -> v | _ -> raise (Failure "next")

let count_possibilities row =
  let is_valid row = 
    List.group row.statuses ~break:(fun s1 s2 -> compare_status s1 s2 <> 0) 
    |> List.filter ~f:(fun group -> compare_status Damaged @@ List.hd_exn group = 0) 
    |> List.map ~f:List.length 
    |> List.equal (=) row.damaged_groups
  in

  let nth_bit x n = x land (1 lsl n) <> 0 in

  let rec count state row =
    let unknowns = List.count row.statuses ~f:is_unknown in
    let max_state = Int.pow 2 unknowns in
    match state with
      | s when s = max_state -> 0
      | _ -> let bits = Sequence.init unknowns ~f:(nth_bit state) in
        let rec cnt bits = function
          | [] -> []
          | status :: rest -> 
            match status with
              | Unknown -> let (b, bits) = seq_next_exn bits in
                (if b then Operational else Damaged) :: cnt bits rest
              | _ -> status :: cnt bits rest
        in

        let statuses = cnt bits row.statuses in
        (if is_valid {statuses; damaged_groups = row.damaged_groups} then 1
        else 0) + count (state + 1) row
  in
  count 0 row

let solve1 input =
  let rows = parse_input input in
  List.iter rows ~f:(fun r -> print_endline @@ show_row r);
  List.map rows ~f:count_possibilities |> List.fold ~init:0 ~f:(+)

let solve2 _ = 0
