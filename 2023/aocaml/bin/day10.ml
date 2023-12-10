open Core

module IntPair = struct
  type t = int * int
  [@@deriving compare, sexp, hash]
end

module Coord = struct
  type t = int * int

  let (+) (x1, y1) (x2, y2) = (x1 + x2, y1 + y2)
  let (=) (x1, y1) (x2, y2) = x1 = x2 && y1 = y2
  let (<>) (x1, y1) (x2, y2) = x1 <> x2 || y1 <> y2

  let show (x, y) = sprintf "(%i, %i)" x y
end

let directions = [(0, 1); (0, -1); (1, 0); (-1, 0)]

let char_to_connections = function
  | '|' -> [(0, -1); (0, 1)]  (* a vertical pipe connecting north and south *)
  | '-' -> [(1, 0); (-1, 0)]  (* a horizontal pipe connecting east and west *)
  | 'L' -> [(0, -1); (1, 0)]  (* a 90-degree bend connecting north and east *)
  | 'J' -> [(0, -1); (-1, 0)] (* a 90-degree bend connecting north and west *)
  | '7' -> [(0, 1); (-1, 0)]  (* a 90-degree bend connecting south and west *)
  | 'F' -> [(0, 1); (1, 0)]   (* a 90-degree bend connecting south and east *)
  | c -> invalid_arg @@ String.of_char c

let parse_map map lines =
  let rec parse map y = function
    | [] -> ()
    | line :: rest ->
      let rec parse_line map x y = function
        | [] -> ()
        | char :: rest -> begin
          (match char with
            | '.' -> ()
            | 'S' -> Hashtbl.add_exn map ~key:(x, y) ~data:[]
            | c -> Hashtbl.add_exn map ~key:(x, y) ~data:(char_to_connections c));
          parse_line map (x + 1) y rest
        end
      in
      parse_line map 0 y line;
      parse map (y + 1) rest
  in
  parse map 0 lines

let find_connecting_pipes map pos =
  let rec find map pos = function
    | [] -> []
    | dir :: rest -> let coord = Coord.(pos + dir) in
      match Hashtbl.find map coord with
        | None -> find map pos rest
        | Some conns ->
          if not @@ List.is_empty @@ List.filter conns ~f:(fun conn -> Coord.(coord + conn = pos)) then
            coord :: find map pos rest
          else
            find map pos rest
  in
  find map pos directions

let solve1 input =
  let input = List.map input ~f:String.to_list in
  let map = Hashtbl.create (module IntPair) in
  parse_map map input;

  let starting_pos = Hashtbl.filter map ~f:(List.is_empty) |> Hashtbl.to_alist |> List.map ~f:fst |> List.hd_exn in
  let curr_positions = find_connecting_pipes map starting_pos |> List.map ~f:(fun pos -> (starting_pos, pos)) in

  let rec walk steps curr_positions =
    let first = snd @@ List.hd_exn curr_positions in
    if List.map curr_positions ~f:(fun (_, pos) -> Coord.(pos = first)) |> List.for_all ~f:(fun a -> a) then
      steps
    else
      let rec next_positions = function
        | [] -> []
        | (prev_pos, pos) :: rest -> let connections = Hashtbl.find_exn map pos in
          let conn = List.hd_exn @@ List.filter connections ~f:(fun conn -> Coord.(pos + conn <> prev_pos)) in
          (pos, Coord.(pos + conn)) :: next_positions rest
      in
      walk (steps + 1) @@ next_positions curr_positions
  in
  walk 1 curr_positions

let solve2 _ = 0
