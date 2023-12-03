(* open Core *)

let string_to_char_list s = List.init (String.length s) (String.get s)

type number = {
  x : int;
  y : int;
  length : int;
  number : int;
}

let new_number x y chars = 
  let length = List.length chars in
  {
    x = x;
    y = y;
    length = length;
    number = Core.String.of_char_list chars |> int_of_string
  }

let parse_map input =
  let rec parse_lines sym_tbl nums y = function
    | [] -> (sym_tbl, nums)
    | line :: rest -> 
      let rec parse_line sym_tbl nums curr_num y x = function
        | [] -> 
            let nums = if not @@ List.is_empty curr_num then
              (new_number (x - List.length curr_num) y @@ List.rev curr_num) :: nums
            else nums
            in
          (sym_tbl, nums)
        | char :: rest -> if Core.Char.is_digit char then
            parse_line sym_tbl nums (char :: curr_num) y (x + 1) rest
          else
            let nums = if not @@ List.is_empty curr_num then
              (new_number (x - List.length curr_num) y @@ List.rev curr_num) :: nums
            else nums
            in

            if char <> '.' then
              Hashtbl.add sym_tbl (x, y) ();

            parse_line sym_tbl nums [] y (x + 1) rest
      in
      let (sym_tbl, nums) = parse_line sym_tbl nums [] y 0 line in
      parse_lines sym_tbl nums (y + 1) rest
  in
  parse_lines (Hashtbl.create 1234) [] 0 @@ List.map string_to_char_list input

let solve1 input = 
  let (sym_tbl, nums) = parse_map input in
  let rec search sym_tbl = function
    | [] -> 0
    | {x; y; length; number} :: rest -> 
      let rec search_vert sym_tbl x y ymax = 
        if y = ymax then
          false
        else
          Option.is_some @@ Hashtbl.find_opt sym_tbl (x, y) || search_vert sym_tbl x (y + 1) ymax
      in
      let rec search_horz sym_tbl x y xmax =
        if x = xmax then
          false
        else
          search_vert sym_tbl x (y - 1) (y + 2) || search_horz sym_tbl (x + 1) y xmax
      in
      if search_horz sym_tbl (x - 1) y (x + length + 1) then
        number + search sym_tbl rest
      else
        search sym_tbl rest
  in
  search sym_tbl nums

let solve2 _ = 0
