use std::collections::VecDeque;
use std::env;
use std::fs;

/*
-- Day 12: Hill Climbing Algorithm ---
You try contacting the Elves using your handheld device, but the river you're following must be too
low to get a decent signal.

You ask the device for a heightmap of the surrounding area (your puzzle input). The heightmap shows
the local area from above broken into a grid; the elevation of each square of the grid is given by
a single lowercase letter, where a is the lowest elevation, b is the next-lowest, and so on up to
the highest elevation, z.

Also included on the heightmap are marks for your current position (S) and the location that should
get the best signal (E). Your current position (S) has elevation a, and the location that should
get the best signal (E) has elevation z.

You'd like to reach E, but to save energy, you should do it in as few steps as possible. During each step, you can move exactly one square up, down, left, or right. To avoid needing to get out your climbing gear, the elevation of the destination square can be at most one higher than the elevation of your current square; that is, if your current elevation is m, you could step to elevation n, but not to elevation o. (This also means that the elevation of the destination square can be much lower than the elevation of your current square.)

For example:

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
Here, you start in the top-left corner; your goal is near the middle. You could start by moving down or right, but eventually you'll need to head toward the e at the bottom. From there, you can spiral around to the goal:

v..v<<<<
>v.vv<<^
.>vv>E^^
..v>>>^^
..>>>>>^
In the above diagram, the symbols indicate whether the path exits each square moving up (^), down (v), left (<), or right (>). The location that should get the best signal is still E, and . marks unvisited squares.

This path reaches the goal in 31 steps, the fewest possible.

What is the fewest steps required to move from your current position to the location that should get the best signal?

--- Part Two ---
As you walk up the hill, you suspect that the Elves will want to turn this into a hiking trail. The
beginning isn't very scenic, though; perhaps you can find a better starting point.

To maximize exercise while hiking, the trail should start as low as possible: elevation a. The goal is still the square marked E. However, the trail should still be direct, taking the fewest steps to reach its goal. So, you'll need to find the shortest path from any square at elevation a to the square marked E.

Again consider the example from above:

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
Now, there are six choices for starting position (five marked a, plus the square marked S that counts as being at elevation a). If you start at the bottom-left square, you can reach the goal most quickly:

...v<<<<
...vv<<^
...v>E^^
.>v>>>^^
>^>>>>>^
This path reaches the goal in only 29 steps, the fewest possible.

What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?

*/

fn draw(arr: &Vec<Vec<i32>>) {
    for row in arr {
        for c in row {
            print!("{:02} ", c);
        }
        print!("\n");
    }
    print!("\n");
}

fn part1(contents: &str) {
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    let mut arr: Vec<Vec<i32>> = vec![];

    // parse
    let a = 'a' as i32;
    for (row, line) in contents.lines().enumerate() {
        let mut new_row: Vec<i32> = vec![];
        for (col, c) in line.bytes().enumerate() {
            match c {
                b'S' => {
                    new_row.push(0);
                    start = (row as i32, col as i32)
                }
                b'E' => {
                    new_row.push(25);
                    end = (row as i32, col as i32)
                }
                _ => {
                    let n = c as i32;
                    new_row.push(n - a);
                }
            }
        }
        arr.push(new_row);
    }

    draw(&arr);

    let mut queue: VecDeque<(i32, i32)> = Default::default();

    let mut visited: Vec<Vec<i32>> = vec![];
    for i in 0..arr.len() {
        visited.push(Default::default());
        for _ in 0..arr[i].len() {
            visited[i].push(-1);
        }
    }

    let neighbors: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

    // 0, distance from start, with -1 as unvisited
    visited[start.0 as usize][start.1 as usize] = 0;
    queue.push_back(start);
    while queue.len() > 0 {
        // enque unvisited valid neighbors
        let pos = queue.pop_front().unwrap();
        let d = visited[pos.0 as usize][pos.1 as usize];
        let h = arr[pos.0 as usize][pos.1 as usize];
        if pos == end {
            // done
            println!("Done, in {:?}", d);
            break;
        }

        for offset_i in 0..neighbors.len() {
            let neighbor = (pos.0 + neighbors[offset_i].0, pos.1 + neighbors[offset_i].1);
            if neighbor.0 < 0
                || neighbor.1 < 0
                || neighbor.0 >= arr.len() as i32
                || neighbor.1 >= arr[neighbor.0 as usize].len() as i32
            {
                // out of bounds
                continue;
            }

            if visited[neighbor.0 as usize][neighbor.1 as usize] >= 0 {
                // already visited
                continue;
            }

            if h + 1 >= arr[neighbor.0 as usize][neighbor.1 as usize] {
                // valid
                visited[neighbor.0 as usize][neighbor.1 as usize] = d + 1;
                queue.push_back(neighbor);
            }
        }
    }
    draw(&visited);
}

fn part2(contents: &str) {
    let mut starts: Vec<(i32, i32)> = Default::default();
    let mut end: (i32, i32) = (0, 0);
    let mut arr: Vec<Vec<i32>> = vec![];

    // parse
    let a = 'a' as i32;
    for (row, line) in contents.lines().enumerate() {
        let mut new_row: Vec<i32> = vec![];
        for (col, c) in line.bytes().enumerate() {
            match c {
                b'S' => {
                    new_row.push(0);
                    starts.push((row as i32, col as i32))
                }
                b'E' => {
                    new_row.push(25);
                    end = (row as i32, col as i32)
                }
                _ => {
                    let n = (c as i32) - a;
                    if n == 0 {
                        starts.push((row as i32, col as i32))
                    }
                    new_row.push(n);
                }
            }
        }
        arr.push(new_row);
    }

    draw(&arr);

    let neighbors: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

    // 0, distance from start, with -1 as unvisited
    let mut min_trail = i32::MAX;
    for start in starts {
        let mut queue: VecDeque<(i32, i32)> = Default::default();

        let mut visited: Vec<Vec<i32>> = vec![];
        for i in 0..arr.len() {
            visited.push(Default::default());
            for _ in 0..arr[i].len() {
                visited[i].push(-1);
            }
        }

        visited[start.0 as usize][start.1 as usize] = 0;
        queue.push_back(start);
        while queue.len() > 0 {
            // enque unvisited valid neighbors
            let pos = queue.pop_front().unwrap();
            let d = visited[pos.0 as usize][pos.1 as usize];
            let h = arr[pos.0 as usize][pos.1 as usize];
            if pos == end {
                // done
                println!("Start: {:?}, Done, in {:?}", start, d);
                min_trail = std::cmp::min(min_trail, d);
                break;
            }

            for offset_i in 0..neighbors.len() {
                let neighbor = (pos.0 + neighbors[offset_i].0, pos.1 + neighbors[offset_i].1);
                if neighbor.0 < 0
                    || neighbor.1 < 0
                    || neighbor.0 >= arr.len() as i32
                    || neighbor.1 >= arr[neighbor.0 as usize].len() as i32
                {
                    // out of bounds
                    continue;
                }

                if visited[neighbor.0 as usize][neighbor.1 as usize] >= 0 {
                    // already visited
                    continue;
                }

                if h + 1 >= arr[neighbor.0 as usize][neighbor.1 as usize] {
                    // valid
                    visited[neighbor.0 as usize][neighbor.1 as usize] = d + 1;
                    queue.push_back(neighbor);
                }
            }
        }
    }
    println!("min Trail: {}", min_trail);
}

fn main() {
    let fname = env::args().nth(1).expect("Should pass 1 filename arg");
    let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}
