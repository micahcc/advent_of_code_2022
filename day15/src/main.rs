use ndarray::prelude::*;
use ndarray::Array2;
use std::cmp;
use std::collections::VecDeque;
use std::env;
use std::fmt;
use std::fs;

/*
--- Day 15: Beacon Exclusion Zone ---
You feel the ground rumble again as the distress signal leads you to a large network of subterranean tunnels. You don't have time to search them all, but you don't need to: your pack contains a set of deployable sensors that you imagine were originally built to locate lost Elves.

The sensors aren't very powerful, but that's okay; your handheld device indicates that you're close enough to the source of the distress signal to use them. You pull the emergency sensor system out of your pack, hit the big button on top, and the sensors zoom off down the tunnels.

Once a sensor finds a spot it thinks will give it a good reading, it attaches itself to a hard surface and begins monitoring for the nearest signal source beacon. Sensors and beacons always exist at integer coordinates. Each sensor knows its own position and can determine the position of a beacon precisely; however, sensors can only lock on to the one beacon closest to the sensor as measured by the Manhattan distance. (There is never a tie where two beacons are the same distance to a sensor.)

It doesn't take long for the sensors to report back their positions and closest beacons (your puzzle input). For example:

Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
So, consider the sensor at 2,18; the closest beacon to it is at -2,15. For the sensor at 9,16, the closest beacon to it is at 10,16.

Drawing sensors as S and beacons as B, the above arrangement of sensors and beacons looks like this:

               1    1    2    2
     0    5    0    5    0    5
 0 ....S.......................
 1 ......................S.....
 2 ...............S............
 3 ................SB..........
 4 ............................
 5 ............................
 6 ............................
 7 ..........S.......S.........
 8 ............................
 9 ............................
10 ....B.......................
11 ..S.........................
12 ............................
13 ............................
14 ..............S.......S.....
15 B...........................
16 ...........SB...............
17 ................S..........B
18 ....S.......................
19 ............................
20 ............S......S........
21 ............................
22 .......................B....
This isn't necessarily a comprehensive map of all beacons in the area, though. Because each sensor only identifies its closest beacon, if a sensor detects a beacon, you know there are no other beacons that close or closer to that sensor. There could still be beacons that just happen to not be the closest beacon to any sensor. Consider the sensor at 8,7:

               1    1    2    2
     0    5    0    5    0    5
-2 ..........#.................
-1 .........###................
 0 ....S...#####...............
 1 .......#######........S.....
 2 ......#########S............
 3 .....###########SB..........
 4 ....#############...........
 5 ...###############..........
 6 ..#################.........
 7 .#########S#######S#........
 8 ..#################.........
 9 ...###############..........
10 ....B############...........
11 ..S..###########............
12 ......#########.............
13 .......#######..............
14 ........#####.S.......S.....
15 B........###................
16 ..........#SB...............
17 ................S..........B
18 ....S.......................
19 ............................
20 ............S......S........
21 ............................
22 .......................B....
This sensor's closest beacon is at 2,10, and so you know there are no beacons that close or closer (in any positions marked #).

None of the detected beacons seem to be producing the distress signal, so you'll need to work out where the distress beacon is by working out where it isn't. For now, keep things simple by counting the positions where a beacon cannot possibly be along just a single row.

So, suppose you have an arrangement of beacons and sensors like in the example above and, just in the row where y=10, you'd like to count the number of positions a beacon cannot possibly exist. The coverage from all sensors near that row looks like this:

                 1    1    2    2
       0    5    0    5    0    5
 9 ...#########################...
10 ..####B######################..
11 .###S#############.###########.
In this example, in the row where y=10, there are 26 positions where a beacon cannot be present.

Consult the report from the sensors you just deployed. In the row where y=2000000, how many positions cannot contain a beacon?


*/
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct World {
    map: Array2<u8>,
    origin: Point,
}

#[derive(Debug)]
struct Sensor {
    sensor: Point,
    beacon: Point,
    dist: i32,
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.map.rows() {
            for v in row {
                write!(f, "{}", *v as char);
            }
            write!(f, "\n");
        }
        return Ok(());
    }
}

fn parse_point(line: &str) -> Point {
    let (xss, yss) = line.split_once(", ").unwrap();
    let x: i32 = xss[2..].parse().unwrap();
    let y: i32 = yss[2..].parse().unwrap();
    return Point { x: x, y: y };
}

fn parse_line(line: &str) -> Sensor {
    let (sensor_str, beacon_str) = line.split_once(":").unwrap();
    let sensor_pt = parse_point(&sensor_str[10..]);
    let beacon_pt = parse_point(&beacon_str[22..]);
    return Sensor {
        dist: ((&sensor_pt.x - &beacon_pt.x).abs() + (&sensor_pt.y - &beacon_pt.y).abs()),
        sensor: sensor_pt,
        beacon: beacon_pt,
    };
}

impl World {
    fn parse(contents: &str, make_floor: bool) -> World {
        // read lines and compute bounds
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut max_d = 0;
        let mut sensors: Vec<Sensor> = vec![];
        for line in contents.lines() {
            let sensor = parse_line(line);
            let s = &sensor.sensor;
            let b = &sensor.beacon;

            min_x = cmp::min(b.x, min_x);
            min_x = cmp::min(s.x, min_x);

            max_x = cmp::max(b.x, max_x);
            max_x = cmp::max(s.x, max_x);

            min_y = cmp::min(b.y, min_y);
            min_y = cmp::min(s.y, min_y);

            max_y = cmp::max(b.y, max_y);
            max_y = cmp::max(s.y, max_y);

            max_d = cmp::max(max_d, sensor.dist);

            sensors.push(sensor);
        }

        min_x -= max_d;
        min_y -= max_d;

        let width = (2 * max_d + 1 + max_x - min_x) as usize;
        let height = (2 * max_d + 1 + max_y - min_y) as usize;
        let mut grid: Array2<u8> = Array::zeros((height as usize, width as usize));
        for x in 0..width {
            for y in 0..height {
                grid[[y, x]] = b'.';
            }
        }

        // draw sensor and beacons
        // and render the areas that are covered
        for sensor in sensors {
            for dx in -sensor.dist..(sensor.dist + 1) {
                // ydist + dx
                let ydist = sensor.dist - dx.abs();
                for dy in -ydist..(ydist + 1) {
                    let x = dx + sensor.sensor.x - min_x;
                    let y = dy + sensor.sensor.y - min_y;
                    grid[[y as usize, x as usize]] = b'#';
                }
            }

            let sx = sensor.sensor.x - min_x;
            let sy = sensor.sensor.y - min_y;
            let bx = sensor.beacon.x - min_x;
            let by = sensor.beacon.y - min_y;
            grid[[sy as usize, sx as usize]] = b'S';
            grid[[by as usize, bx as usize]] = b'B';
        }

        return World {
            map: grid,
            origin: Point { x: min_x, y: min_y },
        };
    }

    fn count_row_coverage(&self, row: i32) -> u32 {
        let y = row - self.origin.y;
        if y < 0 || y >= self.map.nrows() as i32 {
            return 0;
        }

        let mut count = 0;
        for x in 0..self.map.ncols() {
            if self.map[[y as usize, x as usize]] == b'#' {
                count += 1;
            }
        }
        return count;
    }
}

fn part1(contents: &str) {
    let mut world = World::parse(contents, false);

    println!("World:\n\n{}", world);
    println!("Row {} coverage: {}", 10, world.count_row_coverage(10));
    println!(
        "Row {} coverage: {}",
        2000000,
        world.count_row_coverage(2000000)
    );
}

fn part2(contents: &str) {
    let mut world = World::parse(contents, true);
    println!("World: {}", world);
}

fn main() {
    let fname = env::args().nth(1).expect("Should pass 1 filename arg");
    let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

    part1(&contents);
    //part2(&contents);
}
