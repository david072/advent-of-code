open Core

type signal_type = | Low | High
[@@deriving compare]

let show_signal_type = function | High -> "high" | Low -> "low"

type signal = {
  ty : signal_type;
  src : string;
  dst : string;
}

type cmodule_type =
  | Id
  | FlipFlop of bool
  | Conjunction of (string, signal_type) Hashtbl.t * int

type cmodule = {
  ty : cmodule_type;
  name : string;
  connections : string list;
}

let parse_module_name name =
  let (first_char, rest) = List.split_n (String.to_list name) 1 in
  let rem_name = String.of_list rest in
  match List.hd_exn first_char with
    | '%' -> (rem_name, FlipFlop false)
    | '&' -> (rem_name, Conjunction (Hashtbl.create (module String), 0))
    | _ -> (name, Id)

let parse_modules input =
  let parse_line line =
    let rex = Pcre.regexp {|([a-zA-Z%&]+) -> ((\w+(, )?)+)|} in
    let substrs = Pcre.exec ~rex line |> Pcre.get_substrings in
    let (mod_name, ty) = parse_module_name substrs.(1) in
    let connections = substrs.(2) |> String.split ~on:',' |> List.map ~f:String.strip in
    (mod_name, { ty; name = mod_name; connections })
  in
  Hashtbl.of_alist_exn (module String) @@ List.map input ~f:parse_line

let process_signal cmod (sig' : signal) = 
  let src = cmod.name in
  let output_signals ty = List.map ~f:(fun conn -> {ty; src; dst = conn}) in
  match cmod.ty with
    | Id -> (cmod, output_signals sig'.ty cmod.connections)
    | FlipFlop state ->
      if compare_signal_type sig'.ty Low = 0 then
        let state = not state in
        let cmod = { cmod with ty = FlipFlop state } in
        (cmod, output_signals (if state then High else Low) cmod.connections)
      else (cmod, [])
    | Conjunction (states, states_len) -> begin
      let states = Hashtbl.copy states in
      Hashtbl.set states ~key:sig'.src ~data:sig'.ty;
      let out_signal_type = 
        if Hashtbl.length states = states_len && Hashtbl.for_all states ~f:(function | High -> true | _ -> false) then Low
        else High
      in
      ({cmod with ty = Conjunction (states, states_len)}, output_signals out_signal_type cmod.connections)
    end

(** Count how many connections each conjunction module has, since we need this to determine whether it should send a high or a low. *)
let count_conjunction_connections modules =
  let modules_list = Hashtbl.to_alist modules in
  Hashtbl.map_inplace modules ~f:(fun cmod -> match cmod.ty with
    | Conjunction (states, _) -> 
      let count = List.count modules_list ~f:(fun (_, other_cmod) -> List.exists other_cmod.connections ~f:(String.equal cmod.name)) in
      {cmod with ty = Conjunction (states, count)}
    | _ -> cmod
  );
;;

let process_signals modules signals =
  let process sig' =
    match Hashtbl.find modules sig'.dst with
      | Some dst -> begin
        let (cmod, signals) = process_signal dst sig' in
        Hashtbl.set modules ~key:cmod.name ~data:cmod;
        signals
      end
      | None -> []
  in
  List.map signals ~f:process |> Stdlib.List.flatten

let button_signal = {ty = Low; src = "button"; dst = "broadcaster"}

let solve1 input =
  let modules = parse_modules input in
  count_conjunction_connections modules;

  let rec simulate (signals : signal list) =
    let high_pulse_count = List.count signals ~f:(fun sig' -> match sig'.ty with | High -> true | _ -> false) in
    let low_pulse_count = List.length signals - high_pulse_count in
    let new_signals = process_signals modules signals in
    if not @@ List.is_empty new_signals then 
      let (hp, lp) = simulate new_signals in
      (hp + high_pulse_count, lp + low_pulse_count)
    else (high_pulse_count, low_pulse_count)
  in

  let (high_pulses, low_pulses) = List.range 0 1000 
    |> List.map ~f:(fun _ -> simulate [button_signal]) 
    |> List.fold ~init:(0, 0) ~f:(fun (hp_acc, lp_acc) (hp, lp) -> (hp_acc + hp, lp_acc + lp)) in
  high_pulses * low_pulses

(** Doesn't finish :( *)
let solve2 input =
  let modules = parse_modules input in
  count_conjunction_connections modules;

  let rec simulate did_activate_rx (signals : signal list) =
    let new_signals = process_signals modules signals in
    let did_activate_rx = did_activate_rx || List.exists new_signals ~f:(fun sig' -> compare_signal_type sig'.ty Low = 0 && String.equal sig'.dst "rx") in
    if not @@ List.is_empty new_signals then simulate did_activate_rx new_signals
    else did_activate_rx
  in

  let rec count () =
    if simulate false [button_signal] then 1
    else 1 + count ()
  in
  count ()
