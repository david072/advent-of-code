open Core

type galaxy = {
  id : int;
  x : int;
  y : int;
}

type pixel =
  | Empty
  | Galaxy of galaxy

let is_galaxy pixel = match pixel with | Galaxy _ -> true | _ -> false

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

let image_to_map tbl image =
  List.iter image ~f:(List.iter ~f:(
    fun px -> match px with 
      | Galaxy galaxy -> Hashtbl.set tbl ~key:galaxy.id ~data:galaxy 
      | _ -> ()
  ))

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

let galaxy_distance {x = x1; y = y1; _} {x = x2; y = y2; _} = abs (x1 - x2) + abs (y1 - y2)

let sum_distances image max_galaxy_num =
  let map = Hashtbl.create (module Int) in
  image_to_map map image;

  let solve id =
    let dists = List.range (id + 1) max_galaxy_num 
      |> List.map ~f:(fun other_id -> galaxy_distance (Hashtbl.find_exn map id) @@ Hashtbl.find_exn map other_id) in
    List.fold dists ~init:0 ~f:(+)
  in
  List.range 0 max_galaxy_num |> List.map ~f:solve |> List.fold ~init:0 ~f:(+)

let solve1 input =
  let (max_galaxy_num, image) = parse_image @@ List.map input ~f:String.to_list in
  let image = expand_image 2 image in
  sum_distances image max_galaxy_num

let solve2 input =
  let (max_galaxy_num, image) = parse_image @@ List.map input ~f:String.to_list in
  let image = expand_image 1_000_000 image in
  sum_distances image max_galaxy_num
