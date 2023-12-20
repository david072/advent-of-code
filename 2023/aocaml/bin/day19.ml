open Core

type part = {
  x : int;
  m : int;
  a : int;
  s : int;
}

type condition = {
  property : char;
  is_gt : bool;
  rhs : int;
  destination : string;
}

type workflow = {
  name : string;
  conditions : condition list;
  fallback : string;
}

let parse_input input =
  let (workflows, parts) = List.split_while input ~f:(fun s -> not @@ String.is_empty s) in

  let parse_workflow line =
    let rex = Pcre.regexp {|(\w+){(.*),(\w+)}|} in
    let substrs = Pcre.exec ~rex line |> Pcre.get_substrings in
    
    let parse_condition cond =
      let rex = Pcre.regexp {|(\w)([<>])(\d+):(\w+)|} in
      let substrs = Pcre.exec ~rex cond |> Pcre.get_substrings in
      {
        property = List.hd_exn @@ String.to_list substrs.(1);
        is_gt = String.equal substrs.(2) ">";
        rhs = int_of_string substrs.(3);
        destination = substrs.(4)
      }
    in

    {
      name = substrs.(1);
      conditions = String.split ~on:',' substrs.(2) |> List.map ~f:parse_condition;
      fallback = substrs.(3)
    }
    in

  let parse_part line =
    let rex = Pcre.regexp {|{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}|} in
    let substrs = Pcre.exec ~rex line |> Pcre.get_substrings in
    let nums = Array.init 4 ~f:(fun i -> int_of_string substrs.(i + 1)) in
    { x = nums.(0); m = nums.(1); a = nums.(2); s = nums.(3) }
  in

  let workflows = List.map workflows ~f:(fun str -> let wf = parse_workflow str in (wf.name, wf)) 
    |> Hashtbl.of_alist_exn (module String)
  in
  let parts = List.tl_exn parts |> List.map ~f:parse_part in
  (workflows, parts)

let run_condition {x;m;a;s} condition =
  let lhs = match condition.property with 
    | 'x' -> x 
    | 'm' -> m 
    | 'a' -> a 
    | 's' -> s 
    | _ -> raise (Failure "invalid property in workflow")
  in
  if condition.is_gt then lhs > condition.rhs
  else lhs < condition.rhs

let run_workflow part workflow =
  match List.find workflow.conditions ~f:(run_condition part) with
    | Some v -> v.destination
    | None -> workflow.fallback

let solve1 input =
  let (workflows, parts) = parse_input input in

  let rec process_part part wf_name =
    let flow = Hashtbl.find_exn workflows wf_name in
    match run_workflow part flow with
      | "A" -> true
      | "R" -> false
      | wf -> process_part part wf
  in
  List.filter parts ~f:(fun p -> process_part p "in") |> List.map ~f:(fun {x;m;a;s} -> x+m+a+s) |> List.fold ~init:0 ~f:(+)

let solve2 _ = 0
