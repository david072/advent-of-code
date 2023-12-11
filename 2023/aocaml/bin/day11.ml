open Core

type galaxy = {
  id : int;
  x : int;
  y : int;
}

type pixel =
  | Empty
  | Galaxy of galaxy

let is_empty pixel = match pixel with | Empty -> true | _ -> false
let is_galaxy pixel = match pixel with | Galaxy _ -> true | _ -> false
let show_pixel pixel = match pixel with | Galaxy {id; x; y} -> sprintf "{id: %i (%i, %i)}" id x y | Empty -> "."

let parse_image lines =
  let rec parse acc galaxy_num y = function
    | [] -> (galaxy_num, List.rev acc)
    | line :: rest ->
      let rec parse_line acc galaxy_num x y = function
        | [] -> (galaxy_num, List.rev acc)
        | c :: rest -> match c with
          | '.' -> parse_line (Empty :: acc) galaxy_num (x + 1) y rest
          | '#' -> parse_line ((Galaxy {x; y; id = galaxy_num}) :: acc) (galaxy_num + 1) (x + 1) y rest
          | _ -> raise (Failure "what da haaaaaail?")
      in
      let (galaxy_num, res) = parse_line [] galaxy_num 0 y line in
      parse (res :: acc) galaxy_num (y + 1) rest
  in
  parse [] 0 0 lines

let rec image_to_map tbl y = function
  | [] -> ()
  | row :: rest ->
    let rec row_to_map tbl x y = function
      | [] -> ()
      | Empty :: rest -> row_to_map tbl (x + 1) y rest
      | Galaxy {id; x; y} :: rest -> begin
        Hashtbl.add_exn tbl ~key:id ~data:(x, y);
        row_to_map tbl (x + 1) y rest
      end
    in
    row_to_map tbl 0 y row;
    image_to_map tbl (y + 1) rest

let rec expand_rows_and_convert tbl y = function
  | [] -> ()
  | row :: rest -> 
    if List.exists row ~f:is_galaxy then
      expand_rows_and_convert tbl (y + 2) rest
    else
      let rec conv tbl x y = function
        | [] -> ()
        | Empty :: rest -> conv tbl (x + 1) y rest
        | Galaxy {id; x; y} :: rest -> begin
          Hashtbl.add_exn tbl ~key:id ~data:(x, y);
          conv tbl (x + 1) y rest
        end
      in
      conv tbl 0 y row;
      expand_rows_and_convert tbl (y + 1) rest

let expand_rows ?(set_x=false) multiplier image = 
  let rec expand to_add = function
    | [] -> []
    | row :: rest ->
      if not @@ List.exists row ~f:is_galaxy then
        row :: expand (to_add + multiplier - 1) rest
      else
        List.map row ~f:(
          fun el -> match el with
            | Galaxy {id; x; y} when not set_x -> Galaxy {id; y = y + to_add; x} 
            | Galaxy {id; x; y} -> Galaxy {id; y; x = x + to_add} 
            | Empty -> Empty
        ) :: expand to_add rest
  in
  expand 0 image

let expand_image multiplier image =
  let image = expand_rows multiplier image in
  List.transpose_exn image |> expand_rows ~set_x:true multiplier |> List.transpose_exn

let solve1 input =
  let (max_galaxy_num, image) = parse_image @@ List.map input ~f:String.to_list in
  let image = expand_image 2 image in
  (* List.iter image ~f:(fun row -> List.iter row ~f:(fun px -> printf "%s" @@ show_pixel px); printf "\n"); *)

  let map = Hashtbl.create (module Int) in
  image_to_map map 0 image;

  let distance (x1, y1) (x2, y2) = abs (x1 - x2) + abs (y1 - y2) in
  let solve id =
    let dists = List.range (id + 1) max_galaxy_num |> List.map ~f:(fun other_id -> distance (Hashtbl.find_exn map id) @@ Hashtbl.find_exn map other_id) in
    List.fold dists ~init:0 ~f:(+)
  in
  List.range 0 max_galaxy_num |> List.map ~f:solve |> List.fold ~init:0 ~f:(+)

let solve2 input =
  let (max_galaxy_num, image) = parse_image @@ List.map input ~f:String.to_list in
  let image = expand_image 1_000_000 image in
  (* List.iter image ~f:(fun row -> List.iter row ~f:(fun px -> printf "%s" @@ show_pixel px); printf "\n"); *)

  let map = Hashtbl.create (module Int) in
  image_to_map map 0 image;

  let distance (x1, y1) (x2, y2) = abs (x1 - x2) + abs (y1 - y2) in
  let solve id =
    let dists = List.range (id + 1) max_galaxy_num |> List.map ~f:(fun other_id -> distance (Hashtbl.find_exn map id) @@ Hashtbl.find_exn map other_id) in
    List.fold dists ~init:0 ~f:(+)
  in
  List.range 0 max_galaxy_num |> List.map ~f:solve |> List.fold ~init:0 ~f:(+)
