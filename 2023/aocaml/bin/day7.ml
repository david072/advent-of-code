open Core

let cards = ['A'; 'K'; 'Q'; 'J'; 'T'; '9'; '8'; '7'; '6'; '5'; '4'; '3'; '2']
let cards_with_joker_moved = ['A'; 'K'; 'Q'; 'T'; '9'; '8'; '7'; '6'; '5'; '4'; '3'; '2'; 'J']
let classes = [[5]; [1; 4]; [2; 3]; [1; 1; 3]; [1; 2; 2]; [1; 1; 1; 2]; [1; 1; 1; 1; 1]]

(* Converts the hand into a list of the length of how many same elements there are (i.e. "KKK56" -> [1; 1; 3]). *)
let classify_hand hand =
  List.sort ~compare:compare @@ List.map ~f:List.length @@ List.sort_and_group hand ~compare:compare_char

(* Like classify_hand, but combines the length of the jokers with the greatest other length. This makes the jokers extend
   the largest element, thereby reaching the highest possible class. *)
let classify_hand_with_jokers hand =
  let groups = List.map ~f:(fun l -> (l, List.length l)) @@ List.sort_and_group hand ~compare:compare_char in
  let sorted = List.sort groups ~compare:(fun (c1, len1) (c2, len2) -> 
    if Char.equal 'J' @@ List.hd_exn c1 then -1
    else if Char.equal 'J' @@ List.hd_exn c2 then 1
    else compare len1 len2
  ) in
  let res = match List.find sorted ~f:(fun (l, _) -> Char.equal 'J' @@ List.hd_exn l) with
    | None -> sorted
    | Some (_, joker_len) -> 
      if List.length sorted = 1 then 
        sorted 
      else
        let list = List.rev @@ List.tl_exn sorted in
        let (first_char, first_len) = List.hd_exn list in
        List.rev @@ (first_char, first_len + joker_len) :: List.drop list 1
  in
  List.map res ~f:(fun (_, len) -> len)

(* Standard compare function for hand classes *)
let compare_classes hand1 hand2 ?(classify_fn=classify_hand) () =
  let get_class hand = let (i, _) = List.findi_exn classes ~f:(fun _ cls -> List.equal Int.equal cls hand) in i in
  let hand1 = get_class @@ classify_fn hand1 in
  let hand2 = get_class @@ classify_fn hand2 in
  (* This has to be reversed, since `classes` starts with the highest ordered class. *)
  -compare hand1 hand2

(* Standard compare function for hands (i.e. 0 if equal, 1 if greater, -1 if smaller) *)
let compare_hand hand1 hand2 ?(cards=cards) ?(classify_fn=classify_hand) () =
  match compare_classes hand1 hand2 ~classify_fn () with
  | 0 ->
    let map = List.map ~f:(fun c -> let (i, _) = List.findi_exn cards ~f:(fun _ card -> Char.equal card c) in i) in
    List.compare (fun a b -> -compare a b) (map hand1) (map hand2)
  | order -> order

let parse_line line =
  let parts = String.split line ~on:' ' in
  (String.to_list @@ List.hd_exn parts, int_of_string @@ List.nth_exn parts 1)

let solve1 input = 
  let pairs = List.map input ~f:parse_line in
  let sorted = List.sort pairs ~compare:(fun (hand1, _) (hand2, _) -> compare_hand hand1 hand2 ()) in
  List.mapi sorted ~f:(fun i (_, bid) -> (i + 1) * bid) |> List.fold ~init:0 ~f:(+)

let solve2 input =
  let pairs = List.map input ~f:parse_line in
  let sorted = List.sort pairs ~compare:(fun (hand1, _) (hand2, _) -> 
    compare_hand hand1 hand2 ~cards:cards_with_joker_moved ~classify_fn:classify_hand_with_jokers ()) in
  List.mapi sorted ~f:(fun i (_, bid) -> (i + 1) * bid) |> List.fold ~init:0 ~f:(+)
