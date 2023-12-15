open Core

type operation =
  | Remove
  | Add of int

type init_step = {
  label : string;
  box : int;
  op : operation;
}

let hash str = String.to_list str |> List.fold ~init:0 ~f:(fun acc ch -> ((acc + int_of_char ch) * 17) mod 256)

let parse_init_step str =
  try
    let rex = Pcre.regexp {|([a-zA-Z]+)=(\d+)|} in
    let substrs = Pcre.exec ~rex str |> Pcre.get_substrings in
    let label = substrs.(1) in
    { label; box = hash label; op = Add (int_of_string substrs.(2)) }
  with _ ->
    let rex = Pcre.regexp {|([a-zA-Z]+)-|} in
    let substrs = Pcre.exec ~rex str |> Pcre.get_substrings in
    let label = substrs.(1) in
    { label; box = hash label; op = Remove }

let solve1 input =
  let init_seq = String.concat input |> String.split ~on:',' in
  List.map init_seq ~f:hash |> List.fold ~init:0 ~f:(+)

let solve2 input =
  let init_seq = String.concat input |> String.split ~on:',' |> List.map ~f:parse_init_step in
  let boxes = Hashtbl.create (module Int) in

  let rec execute = function
    | [] -> ()
    | step :: rest -> 
      begin
        match step.op with
        | Remove -> begin match Hashtbl.find boxes step.box with
          | None -> ()
          | Some box -> Hashtbl.set boxes ~key:step.box ~data:(List.filter box ~f:(fun l -> not @@ String.equal (fst l) step.label))
        end
        | Add focal_length -> begin match Hashtbl.find boxes step.box with
          | None -> Hashtbl.set boxes ~key:step.box ~data:[(step.label, focal_length)]
          | Some box -> 
            let box = if List.exists box ~f:(fun l -> String.equal (fst l) step.label) then 
              List.map box ~f:(fun l -> if String.equal (fst l) step.label then (step.label, focal_length) else l)
            else 
              box @ [(step.label, focal_length)] 
            in
            Hashtbl.set boxes ~key:step.box ~data:box;
        end
      end;
      execute rest
  in
  execute init_seq;

  let calc_focusing_power box_num i (_, focal_length) = (1 + box_num) * (1 + i) * focal_length in
  Hashtbl.to_alist boxes 
  |> List.map ~f:(fun (box_num, box) -> List.mapi box ~f:(calc_focusing_power box_num) |> List.fold ~init:0 ~f:(+))
  |> List.fold ~init:0 ~f:(+)
