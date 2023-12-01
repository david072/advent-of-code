open Core

let nums = [("one", 1); ("two", 2); ("three", 3); ("four", 4); ("five", 5); ("six", 6); ("seven", 7); ("eight", 8); ("nine", 9)]

let unwrap = function
  | None -> raise (Failure "no value")
  | Some v -> v

let str_to_char_arr str = List.init (String.length str) ~f:(String.get str)

let look_for_number_string arr =
  let rec search arr = function
    | [] -> None
    | (num_str, num_int) :: rest -> if String.length num_str <= List.length arr && 
        List.init (String.length num_str) ~f:(fun i -> unwrap @@ List.nth arr i) |> String.of_char_list |> String.equal num_str
      then Some num_int
      else search arr rest
  in
    search arr nums

let rec solve1 = function
  | [] -> 0
  | line :: rest -> 
    let extract_numbers str =
      let rec extract = function
      | [] -> []
      | v :: rest -> match Char.get_digit v with
        | None -> extract rest
        | Some digit -> digit :: extract rest in
        extract @@ str_to_char_arr str
    in
      match extract_numbers line with
      | [] -> solve1 rest
      | numbers -> let n = unwrap (List.nth numbers 0) * 10 + unwrap (List.last numbers) in
        n + solve1 rest

let rec solve2 = function
  | [] -> 0
  | line :: rest -> 
    let extract_numbers str =
      let rec extract chars = match chars with
        | [] -> []
        | v :: rest -> match Char.get_digit v with
          | Some n -> n :: extract rest
          | None -> match look_for_number_string chars with 
            | None -> extract rest
            | Some n -> n :: extract rest
      in
        extract @@ str_to_char_arr str
    in
      let numbers = extract_numbers line in
      let n = unwrap (List.nth numbers 0) * 10 + unwrap (List.last numbers) in
      n + solve2 rest
