use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Valve {
    rate: u32,
    tunnels: Vec<String>,
    open: bool,
    visited: bool,
}

impl Valve {
    fn additional_rate(&self) -> u32 {
        return if self.open { 0 } else { self.rate };
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("No input file found");
    let valves = input
        .lines()
        .map(|line| {
            let name = line["Valve".len() + 1.."Valve".len() + 3].to_string();
            let rate = line[line.find('=').unwrap() + 1..line.find(';').unwrap()]
                .parse::<u32>()
                .unwrap();
            let last_word_end_idx = line
                .rfind("valves")
                .map(|i| i + "valves".len())
                .or_else(|| line.rfind("valve").map(|i| i + "valve".len()))
                .unwrap();

            let tunnels = line[last_word_end_idx + 1..]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            (
                name,
                Valve {
                    rate,
                    tunnels,
                    open: false,
                    visited: false,
                },
            )
        })
        .collect::<HashMap<String, Valve>>();

    let now = std::time::SystemTime::now();
    let s1 = solution1(valves.clone());
    println!("Solution 1: {s1} (took {:?})", now.elapsed().unwrap());
}

fn solution1(mut valves: HashMap<String, Valve>) -> u32 {
    /*
    fn consider_valve<'a>(
        valves: &HashMap<String, Valve>,
        name: &'a str,
        minutes_remaining: u32,
        mut open_valves: Vec<&'a str>,
    ) -> u32 {
        if minutes_remaining == 0 {
            return 0;
        }

        let valve = &valves[name];

        let max_if_not_opened = valve
            .tunnels
            .iter()
            .map(|name| consider_valve(valves, name, minutes_remaining - 1, open_valves.clone()))
            .max()
            .unwrap();
        if open_valves.contains(&name) || valve.rate == 0 {
            return max_if_not_opened;
        }

        let pressure_released_if_opened = minutes_remaining * valve.rate;
        if minutes_remaining == 1 {
            return pressure_released_if_opened;
        }

        open_valves.push(name);
        let max_if_opened = valve
            .tunnels
            .iter()
            .map(|name| {
                pressure_released_if_opened
                    + consider_valve(valves, name, minutes_remaining - 2, open_valves.clone())
            })
            .max()
            .unwrap();

        max_if_not_opened.max(max_if_opened)
    }

    let result = consider_valve(&valves, "AA", 30, vec![]);
    println!("Result: {result}");
    */

    let mut pressure_released = 0u32;
    let mut minutes_remaining = 30u32;
    let mut current_valve_name = "AA".to_string();
    let mut last_valve_name = "AA".to_string();
    loop {
        if minutes_remaining == 0 {
            break;
        }

        let current_valve = &valves[&current_valve_name];
        let mut next_options = current_valve
            .tunnels
            .iter()
            .filter(|name| **name != last_valve_name)
            .map(|name| (name, &valves[name]))
            .map(|(name, valve)| {
                let max_combined = valve
                    .tunnels
                    .iter()
                    .map(|next_name| {
                        if *next_name == current_valve_name
                            && valve.additional_rate() < current_valve.additional_rate()
                        {
                            return 0;
                        }
                        valve.additional_rate() + valves[next_name].additional_rate()
                    })
                    /*
                    .map(|value| {
                        println!("{value}");
                        value
                    })
                    */
                    .max();
                (max_combined, name, valve)
            })
            .collect::<Vec<_>>();
        next_options.sort_by(
            |(future_achievable_rate_a, ..), (future_achievable_rate_b, ..)| {
                future_achievable_rate_a.cmp(future_achievable_rate_b)
            },
        );
        let next_valve_name = next_options
            .last()
            .map(|(_, name, _)| *name)
            .unwrap_or_else(|| &current_valve.tunnels[0])
            .to_string();

        if current_valve.rate != 0 && !current_valve.open {
            if current_valve.additional_rate() * 2 >= valves[&next_valve_name].rate {
                if !(valves.iter().filter(|(_, valve)| !valve.open).count() <= 3
                    && *valves
                        .iter()
                        .filter(|(_, valve)| valve.open)
                        .max_by(|(_, a), (_, b)| a.rate.cmp(&b.rate))
                        .unwrap()
                        .0
                        != current_valve_name)
                {
                    minutes_remaining -= 1;
                    pressure_released += minutes_remaining * current_valve.rate;
                    valves.get_mut(&current_valve_name).unwrap().open = true;
                    println!("Opened {current_valve_name}");
                }
            }
        }

        valves.get_mut(&current_valve_name).unwrap().visited = true;
        println!("Went to {next_valve_name}");
        last_valve_name = current_valve_name;
        current_valve_name = next_valve_name;
        minutes_remaining -= 1;
    }

    pressure_released
}
