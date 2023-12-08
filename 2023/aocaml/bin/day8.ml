open Core
module SHashtbl = Stdlib.Hashtbl

let rec parse_nodes tbl = function
  | [] -> ()
  | line :: lines -> let rex = Pcre.regexp {|(\w+) = \((\w+), (\w+)\)|} in
    let parts = Pcre.exec ~rex line |> Pcre.get_substrings in
    Hashtbl.set tbl ~key:parts.(1) ~data:(parts.(2), parts.(3));
    parse_nodes tbl lines

let rec parse_nodes2 tbl = function
  | [] -> ()
  | line :: lines -> let rex = Pcre.regexp {|(\w+) = \((\w+), (\w+)\)|} in
    let parts = Pcre.exec ~rex line |> Pcre.get_substrings in
    let rev = String.to_list_rev in
    SHashtbl.add tbl (rev parts.(1)) (rev parts.(2), rev parts.(3));
    parse_nodes2 tbl lines

let solve1 input =
  let instructions = List.hd_exn input |> String.to_list in
  let nodes = Hashtbl.create (module String) in
  parse_nodes nodes @@ List.drop input 2;
  let seq = Sequence.cycle_list_exn instructions in
  let rec walk steps seq = function
  | "ZZZ" -> steps
  | node -> 
    match Sequence.next seq with
    | None -> raise (Failure "seq should be infinite! :^(")
    | Some (inst, seq) -> 
      let (left, right) = Hashtbl.find_exn nodes node in
      let next_node = if Char.equal inst 'L' then left else right in
      walk (steps + 1) seq next_node
  in
  walk 0 seq "AAA"

(* TODO: This needs to be optimized a lot.. *)
let solve2 input =
  let instructions = List.hd_exn input |> String.to_list in
  let nodes = SHashtbl.create 1234 in
  parse_nodes2 nodes @@ List.drop input 2;
  let seq = Sequence.cycle_list_exn instructions in
  let starting_nodes = SHashtbl.to_seq_keys nodes |> Seq.filter (fun node -> Char.equal 'A' @@ List.hd_exn node) |> Stdlib.List.of_seq in
  let rec walk steps seq curr_nodes =
    if List.for_all curr_nodes ~f:(fun node -> Char.equal 'Z' @@ List.hd_exn node) then
      steps
    else
      match Sequence.next seq with
      | None -> raise (Failure "seq should be infinite! :^(")
      | Some (inst, seq) -> 
        let curr_nodes = List.map curr_nodes ~f:(
          fun node -> let (left, right) = SHashtbl.find nodes node in
            if Char.equal 'L' inst then left else right
        ) in
        walk (steps + 1) seq curr_nodes
  in
  walk 0 seq starting_nodes
