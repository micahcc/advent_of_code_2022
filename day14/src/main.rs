use ndarray::prelude::*;
use ndarray::Array2;
use std::cmp;
use std::collections::VecDeque;
use std::env;
use std::fmt;
use std::fs;

/*
--- Day 14: Regolith Reservoir ---
The distress signal leads you to a giant waterfall! Actually, hang on - the signal seems like it's coming from the waterfall itself, and that doesn't make any sense. However, you do notice a little path that leads behind the waterfall.

Correction: the distress signal leads you behind a giant waterfall! There seems to be a large cave system here, and the signal definitely leads further inside.

As you begin to make your way deeper underground, you feel the ground rumble for a moment. Sand begins pouring into the cave! If you don't quickly figure out where the sand is going, you could quickly become trapped!

Fortunately, your familiarity with analyzing the path of falling material will come in handy here. You scan a two-dimensional vertical slice of the cave above you (your puzzle input) and discover that it is mostly air with structures made of rock.

Your scan traces the path of each solid rock structure and reports the x,y coordinates that form the shape of the path, where x represents distance to the right and y represents distance down. Each path appears as a single line of text in your scan. After the first point of each path, each point indicates the end of a straight horizontal or vertical line to be drawn from the previous point. For example:

498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
This scan means that there are two paths of rock; the first path consists of two straight lines, and the second path consists of three straight lines. (Specifically, the first path consists of a line of rock from 498,4 through 498,6 and another line of rock from 498,6 through 496,6.)

The sand is pouring into the cave from point 500,0.

Drawing rock as #, air as ., and the source of the sand as +, this becomes:


  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.
6 ..###...#.
7 ........#.
8 ........#.
9 #########.
Sand is produced one unit at a time, and the next unit of sand is not produced until the previous unit of sand comes to rest. A unit of sand is large enough to fill one tile of air in your scan.

A unit of sand always falls down one step if possible. If the tile immediately below is blocked (by rock or sand), the unit of sand attempts to instead move diagonally one step down and to the left. If that tile is blocked, the unit of sand attempts to instead move diagonally one step down and to the right. Sand keeps moving as long as it is able to do so, at each step trying to move down, then down-left, then down-right. If all three possible destinations are blocked, the unit of sand comes to rest and no longer moves, at which point the next unit of sand is created back at the source.

So, drawing sand that has come to rest as o, the first unit of sand simply falls straight down and then stops:

......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
......o.#.
#########.
The second unit of sand then falls straight down, lands on the first one, and then comes to rest to its left:

......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
.....oo.#.
#########.
After a total of five units of sand have come to rest, they form this pattern:

......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
......o.#.
....oooo#.
#########.
After a total of 22 units of sand:

......+...
..........
......o...
.....ooo..
....#ooo##
....#ooo#.
..###ooo#.
....oooo#.
...ooooo#.
#########.
Finally, only two more units of sand can possibly come to rest:

......+...
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########.
Once all 24 units of sand shown above have come to rest, all further sand flows out the bottom, falling into the endless void. Just for fun, the path any new sand takes before falling forever is shown here with ~:

.......+...
.......~...
......~o...
.....~ooo..
....~#ooo##
...~o#ooo#.
..~###ooo#.
..~..oooo#.
.~o.ooooo#.
~#########.
~..........
~..........
~..........
Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?


--- Part Two ---
You realize you misread the scan. There isn't an endless void at the bottom of the scan - there's
floor, and you're standing on it!

You don't have time to scan the floor, so assume the floor is an infinite horizontal line with a y
coordinate equal to two plus the highest y coordinate of any point in your scan.

In the example above, the highest y coordinate of any point is 9, and so the floor is at y=11. (This is as if your scan contained one extra rock path like -infinity,11 -> infinity,11.) With the added floor, the example above now looks like this:

        ...........+........
        ....................
        ....................
        ....................
        .........#...##.....
        .........#...#......
        .......###...#......
        .............#......
        .............#......
        .....#########......
        ....................
<-- etc #################### etc -->

To find somewhere safe to stand, you'll need to simulate falling sand until a unit of sand comes to
rest at 500,0, blocking the source entirely and stopping the flow of sand into the cave. In the
example above, the situation finally looks like this after 93 units of sand come to rest:

............o............
...........ooo...........
..........ooooo..........
.........ooooooo.........
........oo#ooo##o........
.......ooo#ooo#ooo.......
......oo###ooo#oooo......
.....oooo.oooo#ooooo.....
....oooooooooo#oooooo....
...ooo#########ooooooo...
..ooooo.......ooooooooo..
#########################

Using your scan, simulate the falling sand until the source of the sand becomes blocked. How many
units of sand come to rest?


*/

#[derive(Debug)]
struct World {
    state: Array2<u8>,
    origin: Point,
    input: Point, // x,y of where sand comes in
}

#[derive(Debug)]
enum SandResult {
    Oblivion,
    Falling,
    Settled,
    Plugged,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.state.rows() {
            for v in row {
                write!(f, "{}", *v as char);
            }
            write!(f, "\n");
        }
        return Ok(());
    }
}

impl World {
    fn parse(contents: &str, make_floor: bool) -> World {
        // read lines and compute bounds
        let mut min_x = 500;
        let mut max_x = 500;
        let mut min_y = 0;
        let mut max_y = 0;
        let mut lines: Vec<Line> = vec![];
        for line in contents.lines() {
            let mut last = None;
            for pos in line.split(" -> ") {
                let maybe_spl = pos.split_once(",");
                if !maybe_spl.is_some() {
                    continue;
                }
                let (xs, ys) = maybe_spl.unwrap();
                let x: i32 = xs.parse().unwrap();
                let y: i32 = ys.parse().unwrap();

                min_x = cmp::min(x, min_x);
                max_x = cmp::max(x, max_x);
                min_y = cmp::min(y, min_y);
                max_y = cmp::max(y, max_y);

                let curr_pt = Point { x: x, y: y };
                match last {
                    Some(prev_pt) => {
                        lines.push(Line {
                            from: prev_pt,
                            to: curr_pt,
                        });
                    }
                    _ => {}
                }
                last = Some(Point { x: x, y: y });
            }
        }

        let floor_y = max_y + 2;

        // allocate grid, pad by alot to help with bounds checking or weird
        // cases where sand needs to fall off known platforms
        min_x -= 150;
        min_y -= 4;
        max_x += 150;
        max_y += 4;
        let width = (1 + max_x - min_x) as usize;
        let height = (1 + max_y - min_y) as usize;
        let mut grid: Array2<u8> = Array::zeros((height as usize, width as usize));
        for x in 0..width {
            for y in 0..height {
                grid[[y, x]] = b'.';
            }
        }

        if make_floor {
            for x in 0..width {
                let r = (floor_y - min_y) as usize;
                grid[[r, x]] = b'#';
            }
        }

        // draw lines
        for line in lines {
            if line.from.x == line.to.x {
                // vertical
                let x = (line.from.x - min_x) as usize;
                let y0 = (cmp::min(line.from.y, line.to.y) - min_y) as usize;
                let y1 = (cmp::max(line.from.y, line.to.y) + 1 - min_y) as usize;
                for y in y0..y1 {
                    grid[[y, x]] = b'#';
                }
            }
            if line.from.y == line.to.y {
                // vertical
                let y = (line.from.y - min_y) as usize;
                let x0 = (cmp::min(line.from.x, line.to.x) - min_x) as usize;
                let x1 = (cmp::max(line.from.x, line.to.x) + 1 - min_x) as usize;
                for x in x0..x1 {
                    grid[[y, x]] = b'#';
                }
            }
        }

        grid[[((0 - min_y) as usize), ((500 - min_x) as usize)]] = b'+';
        return World {
            state: grid,
            origin: Point { x: min_x, y: min_y },
            input: Point { x: 500, y: 0 },
        };
    }

    fn add_sand(&mut self) -> SandResult {
        let offsets = [(0, 1), (-1, 1), (1, 1)];
        let start_pos = Point {
            x: self.input.x - self.origin.x,
            y: self.input.y - self.origin.y,
        };
        let mut pos = Point {
            x: start_pos.x,
            y: start_pos.y,
        };
        //println!("Initial position: {:?}", pos);

        if self.state[[start_pos.y as usize, start_pos.x as usize]] == b'o' {
            return SandResult::Plugged;
        }

        // drop sand downward
        for _ in 0..1000 {
            let mut found_air = false;
            for offset in offsets {
                let target = Point {
                    x: pos.x + offset.0,
                    y: pos.y + offset.1,
                };
                if target.y as usize >= self.state.nrows() {
                    // done, found a way out of this world
                    //println!("OBLIVION");
                    return SandResult::Oblivion;
                }

                // should be in bounds, we padded it
                assert!(target.x >= 0 && (target.x as usize) < self.state.ncols());

                match self.state[[target.y as usize, target.x as usize]] {
                    b'.' => {
                        // air, we can move forward
                        found_air = true;
                        // println!("{:?} -> {:?}", pos, target);
                        pos = target;
                        break;
                    }
                    b'#' => {
                        // rock, keep searching
                    }
                    b'+' => {
                        // starting point, keep searching
                    }
                    b'o' => {
                        // more sand keep searching
                    }
                    _ => {
                        panic!("WTF");
                    }
                }
            }

            if !found_air {
                self.state[[pos.y as usize, pos.x as usize]] = b'o';
                return SandResult::Settled;
            }

            // otherwise keep falling
        }

        return SandResult::Falling;
    }
}

fn part1(contents: &str) {
    let mut world = World::parse(contents, false);

    let mut iters = 0;
    for _ in 0..1000 {
        //println!("Add Sand");
        match world.add_sand() {
            SandResult::Falling => {
                panic!("Sand fell forever?");
            }
            SandResult::Settled => {
                // pour more!
            }
            SandResult::Plugged => {
                println!("Sand plugged the whole, done");
            }
            SandResult::Oblivion => {
                println!("Sand left this world, done");
                break;
            }
        }
        //println!("{}", world);
        iters += 1;
    }

    println!("Finished at {}, World: {}", iters, world);
}

fn part2(contents: &str) {
    let mut world = World::parse(contents, true);

    let mut iters = 0;
    for _ in 0..50000 {
        //println!("Add Sand");
        match world.add_sand() {
            SandResult::Falling => {
                panic!("Sand fell forever?");
            }
            SandResult::Settled => {
                // pour more!
            }
            SandResult::Plugged => {
                println!("Sand plugged the whole, done");
                break;
            }
            SandResult::Oblivion => {
                println!("Sand left this world, done");
                break;
            }
        }
        //println!("{}", world);
        iters += 1;
    }

    println!("Finished at {}, World: {}", iters, world);
}

fn main() {
    let fname = env::args().nth(1).expect("Should pass 1 filename arg");
    let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

    //part1(&contents);
    part2(&contents);
}
