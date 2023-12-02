open Core

let unwrap = function
  | None -> raise (Failure "no value")
  | Some v -> v

type line = {
  id : int;
  color_info : string list list;
}

let parse_line line =
  let parts = String.split line ~on:':' in
  let id = unwrap @@ List.nth parts 0 |> fun s -> String.sub ~pos:5 ~len:((String.length s) - 5) s |> Int.of_string in 
  let col_info = unwrap @@ List.nth parts 1 |> Str.split (Str.regexp ",\\|;") |> List.map ~f:(fun s -> String.split ~on:' ' @@ String.strip s) in
  { id = id; color_info = col_info }

let rec solve1 = function
  | [] -> 0
  | line :: rest ->
    let line = parse_line line in

    let rec check = function
      | (count :: color :: _) :: rest -> begin 
        let count = Int.of_string count in
        begin match color with
          | "red" -> count <= 12
          | "green" -> count <= 13
          | "blue" -> count <= 14
          | _ -> raise (Failure (Format.sprintf "Invalid color: %s" color))
        end && check rest
      end
      | [] -> true
      | _ -> false
    in
      if check line.color_info then line.id + solve1 rest else solve1 rest

let rec solve2 = function
    | [] -> 0
    | line :: rest ->
      let line = parse_line line in
      let rec find_mins red green blue = function
        | (count :: color :: _) :: rest -> begin
          let count = Int.of_string count in
          match color with
            | "red" -> find_mins (Int.max red count) green blue rest
            | "green" -> find_mins red (Int.max green count) blue rest
            | "blue" -> find_mins red green (Int.max blue count) rest
            | _ -> raise (Failure (Format.sprintf "Invalid color: %s" color))
        end
        | [] -> (red, green, blue)
        | _ -> raise (Failure ":(")
      in
      let (r, g, b) = find_mins 0 0 0 line.color_info in
      r * g * b + solve2 rest
