use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Instant;

pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 15)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

#[derive(Debug, Clone)]
struct Sensor {
    position: (isize, isize),
    beacon_position: (isize, isize),
    distance_to_closest_beacon: isize,
}

impl Sensor {
    pub fn parse(input: &str) -> Self {
        let parts = input.split(' ')
            .filter(|part| part.contains('='))
            .map(|part| part.split(&['=', ':', ',']).nth(1).unwrap())
            .collect::<Vec<_>>();
        let mut positions = parts.chunks(2)
            .map(|chunks| {
                let x = chunks[0].parse::<isize>().unwrap();
                let y = chunks[1].parse::<isize>().unwrap();
                (x, y)
            });
        let position = positions.next().unwrap();
        let beacon_position = positions.next().unwrap();

        Self {
            position,
            beacon_position,
            distance_to_closest_beacon: distance(position, beacon_position),
        }
    }
}

fn distance(pos1: (isize, isize), pos2: (isize, isize)) -> isize {
    (pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)) as isize
}

fn is_occupied(sensors: &[Sensor], x: isize, y: isize, count_sensors_and_beacons: bool) -> bool {
    let mut did_hit = false;
    for sensor in sensors {
        if (sensor.beacon_position == (x, y) || sensor.position == (x, y)) && !count_sensors_and_beacons {
            return false;
        }

        if did_hit { continue; }
        let distance = distance((x, y), sensor.position);
        if distance <= sensor.distance_to_closest_beacon {
            did_hit = true;
            if count_sensors_and_beacons {
                break;
            }
        }
    }

    did_hit
}

fn solve1(input: &str) -> usize {
    let sensors = input.lines()
        .map(Sensor::parse)
        .collect::<Vec<_>>();

    const Y: isize = 2_000_000;
    // const Y: isize = 10; // for test input

    // Calculate min_x and max_x.
    // By solving the equation for the Manhattan Distance |x1 - x2| + |y1 - y2| = d for x2,
    // we get two equations:
    //  - x2 = -(d - |y1 - y2| - x1)
    //  - x2 = d - |y1 - y2| + x1
    // With y2 being the y coordinate we're supposed to look at ("Y"), these equations give us the
    // two furthest points on that y coordinate that the sensor still covers. We can then construct
    // an iterator with these values and take the minimum and the maximum values.
    let iter = sensors.iter()
        .filter(|sensor| distance((sensor.position.0, Y), sensor.position) <= sensor.distance_to_closest_beacon)
        .flat_map(|sensor| {
            let y_diff = sensor.position.1.abs_diff(Y) as isize;
            let first = -(sensor.distance_to_closest_beacon - y_diff - sensor.position.0);
            let second = sensor.distance_to_closest_beacon - y_diff + sensor.position.0;
            vec![first, second]
        });

    let min_x = iter.clone().min().unwrap();
    let max_x = iter.max().unwrap();
    println!("x-range: {min_x}..{max_x}");

    let start = Instant::now();

    let mut occupied_tiles = 0usize;

    for x in min_x..=max_x {
        if is_occupied(&sensors, x, Y, false) { occupied_tiles += 1; }
    }

    println!("Finished in {:?}\n", start.elapsed());

    occupied_tiles
}

fn solve2(input: &str) -> isize {
    let sensors = input.lines()
        .map(Sensor::parse)
        .collect::<Vec<_>>();

    let sensors = Arc::new(sensors);

    const JOBS: usize = 32;
    const MAX: usize = 4_000_000;

    let slice_height = (MAX / JOBS) as isize;

    let (tx, rx) = channel::<(isize, isize)>();

    let start = Instant::now();

    for i in 0..JOBS {
        let y_offset = slice_height * (i as isize);
        let sensors = sensors.clone();
        let tx = tx.clone();
        std::thread::spawn(move || {
            for y in y_offset..y_offset + slice_height {
                println!("thread {i}: {}/{slice_height}", y - y_offset);
                let iter = sensors.iter()
                    .filter(|sensor| distance((sensor.position.0, y), sensor.position) <= sensor.distance_to_closest_beacon)
                    .flat_map(|sensor| {
                        let y_diff = sensor.position.1.abs_diff(y) as isize;
                        let first = -(sensor.distance_to_closest_beacon - y_diff - sensor.position.0);
                        let second = sensor.distance_to_closest_beacon - y_diff + sensor.position.0;
                        vec![first, second]
                    });

                let Some(min_x) = iter.clone().min().map(|min| min.max(0)) else { continue; };
                let Some(max_x) = iter.max().map(|max| max.min(4_000_000)) else { continue; };

                for x in min_x..=max_x {
                    if !is_occupied(&sensors, x, y, true) {
                        tx.send((x, y)).unwrap();
                        return;
                    }
                }
            }
        });
    }

    let (x, y) = rx.recv().unwrap();

    println!("Finished in {:?}", start.elapsed());

    x * 4_000_000 + y
}
