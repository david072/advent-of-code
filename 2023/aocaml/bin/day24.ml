open Core

type hailstone2d = {
  pos : float * float;
  vel : float * float;
}

let parse_line2 line =
  let parts = String.split ~on:'@' line |> List.map ~f:(String.split ~on:',') |> Stdlib.List.flatten |> List.map ~f:String.strip in
  let ints = Array.of_list @@ List.map parts ~f:float_of_string in
  { pos = (ints.(0), ints.(1)); vel = (ints.(3), ints.(4)) }

let make_pairs list =
  let gen i a =
    List.init (List.length list - i - 1) ~f:(fun i2 -> (a, List.nth_exn list @@ i + i2 + 1))
  in
  List.mapi list ~f:gen |> Stdlib.List.flatten

let get_intersection h1 h2 =
  let (xu1, yu1) = h1.pos in
  let (xu2, yu2) = h2.pos in
  let (xr1, yr1) = h1.vel in
  let (xr2, yr2) = h2.vel in

  let s = Float.((yu1 * xr1 + yr1 * xu2 - yu2 * xr1 - yr1 * xu1) / (-yr1 * xr2 + yr2 * xr1)) in
  if Float.(s = Float.infinity) then None
  else
    let t = Float.((xu2 + xr2 * s - xu1) / xr1) in
    if Float.(s < 0. || t < 0.) then None
    else Some Float.(xu2 + xr2 * s, yu2 + yr2 * s)

let solve1 lines =
  let stones = List.map lines ~f:parse_line2 in
  let pairs = make_pairs stones in

  let check_pair mn mx (h1, h2) =
    match get_intersection h1 h2 with
      | None -> false
      | Some (x, y) -> begin
        Float.(x >= mn && x <= mx && y >= mn && y <= mx)
      end
  in
  List.count pairs ~f:(check_pair 200000000000000. 400000000000000.)

let solve2 _ = 0
