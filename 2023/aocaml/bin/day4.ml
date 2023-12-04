open Core

type card = {
  id : int;
  winning : int list;
  have : int list;
}

let parse_part part =
  String.split part ~on:' ' |> List.filter_map ~f:(fun s -> 
    let s = String.strip s in
    if String.length s = 0 then None else Some (int_of_string s))

let parse_line line =
  let rex = Pcre.regexp {|Card *(\d+): *((\d+ *)+)\| *((\d+ *)+)|} in
  let parts = Pcre.exec ~rex line |> Pcre.get_substrings in
  let id = int_of_string parts.(1) in
  let winning = parse_part parts.(2) in
  let have = parse_part parts.(4) in
  {id; winning; have}

let get_matches card = 
  List.count card.have ~f:(fun el -> List.exists card.winning ~f:(fun w -> el = w))

let solve1 input = 
  List.map input ~f:(fun line ->
    let card = parse_line line in
    let count = get_matches card in
    if count > 0 then Int.pow 2 @@ count - 1 else 0
  ) 
  |> List.fold ~init:0 ~f:(fun points acc -> points + acc)

let solve2 input = 
  let cards = List.map ~f:parse_line input in
  let rec expand card = 
    match get_matches card with
      | 0 -> 1
      | matches -> 
        let won_cards = List.init matches ~f:(fun i -> List.nth_exn cards @@ card.id + i) in
        1 + (List.fold ~init:0 ~f:(fun acc el -> acc + expand el) won_cards)
  in
  List.fold cards ~init:0 ~f:(fun acc el -> acc + expand el)
