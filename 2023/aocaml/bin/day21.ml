open Core
open Util

module Comparable_vec = struct
  include Vec
  include Base.Comparator.Make (Vec)
end

type tile = {
  distance : int;
  visited : bool;
  is_start : bool;
}

let parse_tile = function
  | '.' -> Some { distance = Int.max_value; visited = false; is_start = false }
  | 'S' -> Some { distance = Int.max_value; visited = false; is_start = true }
  | _ -> None

let parse_map tbl input =
  let rec parse tbl y = function
    | [] -> ()
    | line :: rest ->
      let rec parse_line tbl x y = function
        | [] -> ()
        | ch :: rest -> begin
          (match parse_tile ch with
            | Some t -> Hashtbl.set tbl ~key:(x, y) ~data:t
            | None -> ());
          parse_line tbl (x + 1) y rest
          end
      in
      parse_line tbl 0 y line;
      parse tbl (y + 1) rest
  in
  parse tbl 0 @@ List.map input ~f:String.to_list

let solve1 input =
  let map = Hashtbl.create (module Util.Vec) in
  parse_map map input;

  let (start_pos, _) = Hashtbl.to_alist map |> List.find_exn ~f:(fun (_, {is_start; _}) -> is_start) in

  let rec step positions steps =
    let get_neighbors pos = 
      List.map Vec.directions ~f:(Vec.(+) pos) |> List.filter ~f:(fun p -> Option.is_some @@ Hashtbl.find map p) in

    let positions = Stdlib.List.flatten @@ List.map ~f:get_neighbors @@ Set.to_list positions in
    let positions = Set.of_list (module Comparable_vec) positions in
    if steps = 1 then Set.length positions
    else step positions @@ steps - 1
  in
  step (Set.of_list (module Comparable_vec) [start_pos]) 64

let solve2 _ = 0
