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
    number = Core.String.of_char_list chars |> int_of_string;
  }

type symbol = {
  adjacent_numbers: (int * int) list;
}

let parse_map input ~include_char =
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

            if match include_char with
                | None -> char <> '.'
                | Some c -> char = c
            then
              Hashtbl.add sym_tbl (x, y) {adjacent_numbers = []};

            parse_line sym_tbl nums [] y (x + 1) rest
      in
      let (sym_tbl, nums) = parse_line sym_tbl nums [] y 0 line in
      parse_lines sym_tbl nums (y + 1) rest
  in
  parse_lines (Hashtbl.create 1234) [] 0 @@ List.map string_to_char_list input

let solve1 input = 
  let (sym_tbl, nums) = parse_map input ~include_char:None in
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

let solve2 input =
  let (sym_tbl, nums) = parse_map input ~include_char:(Some '*') in
  let rec mark sym_tbl = function
    | [] -> ();
    | {x; y; length; _} :: rest ->
      let rec mark_vert sym_tbl x y ymax num_x num_y =
        if y < ymax then begin
          match Hashtbl.find_opt sym_tbl (x, y) with
            | Some {adjacent_numbers} -> Hashtbl.add sym_tbl (x, y) @@ {adjacent_numbers = (num_x, num_y) :: adjacent_numbers};
            | _ -> ();
          mark_vert sym_tbl x (y + 1) ymax num_x num_y;
        end
      in
      let rec mark_horz sym_tbl x y xmax num_x num_y =
        if x < xmax then begin
          mark_vert sym_tbl x (y - 1) (y + 2) num_x num_y;
          mark_horz sym_tbl (x + 1) y xmax num_x num_y;
        end
      in
      mark_horz sym_tbl (x - 1) y (x + length + 1) x y;
      mark sym_tbl rest;
  in
  mark sym_tbl nums;
  Hashtbl.filter_map_inplace (fun _ v -> if (List.length v.adjacent_numbers) = 2 then Some v else None) sym_tbl;
  let gear_adj_nums = Hashtbl.fold (fun _ v acc -> v.adjacent_numbers :: acc) sym_tbl [] in
  List.map (fun adj_nums ->
    let nums = List.map (fun (x, y) -> List.find (fun num -> x = num.x && y = num.y) nums) adj_nums in
    List.fold_left (fun acc num -> num.number * acc) 1 nums
  ) gear_adj_nums
  |> List.fold_left (fun ratio acc -> acc + ratio) 0
