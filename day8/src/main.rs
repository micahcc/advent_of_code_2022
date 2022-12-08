use std::collections::HashMap;
use std::env;
use std::fs;
/*
--- Day 8: Treetop Tree House ---
The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The
Elves explain that a previous expedition planted these trees as a reforestation effort. Now,
they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this,
you need to count the number of trees that are visible from outside the grid when looking directly
along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your
puzzle input). For example:

30373
25512
65332
33549
35390

Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9
is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it.
Only consider trees in the same row or column; that is, only look up, down, left, or right from any
given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge,
there are no trees to block the view. In this example, that only leaves the interior nine trees to
consider:

The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since
other trees of height 5 are in the way.)

The top-middle 5 is visible from the top and right.

The top-right 1 is not visible from any direction; for it to be visible, there would need to only
be trees of height 0 between it and an edge.

The left-middle 5 is visible, but only from the right.

The center 3 is not visible from any direction; for it to be visible, there would need to be only
trees of at most height 2 between it and an edge.

The right-middle 3 is visible from the right.
In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are
visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?

--- Part Two ---
Content with the amount of tree cover available, the Elves just need to know the best spot to build
their tree house: they would like to be able to see a lot of trees.

To measure the viewing distance from a given tree, look up, down, left, and right from that tree;
stop if you reach an edge or at the first tree that is the same height or taller than the tree
under consideration. (If a tree is right on the edge, at least one of its viewing distances will be
zero.)

The Elves don't care about distant trees taller than those found by the rules above; the proposed
tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree
house anyway.

In the example above, consider the middle 5 in the second row:

30373
25512
65332
33549
35390

Looking up, its view is not blocked; it can see 1 tree (of height 3).

Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to
it).

Looking right, its view is not blocked; it can see 2 trees.

Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of
height 5 that blocks its view).

A tree's scenic score is found by multiplying together its viewing distance in each of the four
directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).

However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

30373
25512
65332
33549
35390

Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
Looking left, its view is not blocked; it can see 2 trees.
Looking down, its view is also not blocked; it can see 1 tree.
Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

Consider each tree on your map. What is the highest scenic score possible for any tree?
*/

fn part1(contents: &str) {
    // find height,width
    let mut heights: Vec<Vec<i8>> = Default::default();

    // 0 => not analyzed or not visible
    // 1 => visible
    let mut is_visible: Vec<Vec<i8>> = Default::default();

    let mut width = 0;
    let mut height = 0;
    for line in contents.lines() {
        if line.trim().len() == 0 {
            break;
        }

        heights.push(
            line.trim()
                .chars()
                .map(|u| (u as u8 - '0' as u8) as i8)
                .collect(),
        );
        is_visible.push(vec![0; heights.last().unwrap().len()]);

        width = heights.last().unwrap().len();
        height += 1;
    }

    println!("Width: {}, Height: {}", width, height);
    // left -> right
    for row in 0..height {
        let mut max_height = -1;
        for col in 0..width {
            let h = heights[row][col];
            if h > max_height {
                is_visible[row][col] = 1;
                max_height = h;
            }
        }
    }

    // right -> left
    for row in 0..height {
        let mut max_height = -1;
        for rcol in 0..width {
            let col = width - rcol - 1;
            let h = heights[row][col];
            if h > max_height {
                is_visible[row][col] = 1;
                max_height = h;
            }
        }
    }

    // top -> bottom
    for col in 0..width {
        let mut max_height = -1;
        for row in 0..height {
            let h = heights[row][col];
            if h > max_height {
                is_visible[row][col] = 1;
                max_height = h;
            }
        }
    }

    // bottom -> top
    for col in 0..width {
        let mut max_height = -1;
        for rrow in 0..height {
            let row = height - rrow - 1;
            let h = heights[row][col];
            if h > max_height {
                is_visible[row][col] = 1;
                max_height = h;
            }
        }
    }

    //for row in 0..height {
    //    for col in 0..width {
    //        print!("{}", heights[row][col]);
    //    }
    //    print!("\n");
    //}
    //print!("\n");

    let mut num_vis: u64 = 0;
    for row in 0..height {
        for col in 0..width {
            //print!("{}", is_visible[row][col]);
            num_vis += is_visible[row][col] as u64;
        }
        //print!("\n");
    }

    println!("Num Vis: {}", num_vis);
}

fn get_view(
    mid_row: usize,
    mid_col: usize,
    image_height: usize,
    image_width: usize,
    heights: &Vec<Vec<i8>>,
) -> i32 {
    let mid_height = heights[mid_row][mid_col];

    let mut left = 0;
    for i in 1..mid_col + 1 {
        let col = mid_col - i;
        left = i;
        if heights[mid_row][col] >= mid_height {
            break;
        }
    }

    let mut right = 0;
    for col in mid_col + 1..image_width {
        right = col - mid_col;
        if heights[mid_row][col] >= mid_height {
            break;
        }
    }

    let mut up = 0;
    for i in 1..mid_row + 1 {
        let row = mid_row - i;
        up = i;
        if heights[row][mid_col] >= mid_height {
            break;
        }
    }

    let mut down = 0;
    for row in mid_row + 1..image_height {
        down = row - mid_row;
        if heights[row][mid_col] >= mid_height {
            break;
        }
    }

    let score = left * right * up * down;
    //println!(
    //    "At {} ({}, {}), Left: {}, Right: {}, Up: {}, Down: {}, Score: {}",
    //    mid_height, mid_row, mid_col, left, right, up, down, score
    //);
    return score as i32;
}

fn part2(contents: &str) {
    // find height,width
    let mut heights: Vec<Vec<i8>> = Default::default();

    // 0 => not analyzed or not visible
    // 1 => visible
    let mut left_view: Vec<Vec<i32>> = Default::default();
    let mut right_view: Vec<Vec<i32>> = Default::default();
    let mut up_view: Vec<Vec<i32>> = Default::default();
    let mut down_view: Vec<Vec<i32>> = Default::default();

    let mut width = 0;
    let mut height = 0;
    for line in contents.lines() {
        if line.trim().len() == 0 {
            break;
        }

        heights.push(
            line.trim()
                .chars()
                .map(|u| (u as u8 - '0' as u8) as i8)
                .collect(),
        );

        width = heights.last().unwrap().len();
        height += 1;
        left_view.push(Default::default());
        right_view.push(Default::default());
        up_view.push(Default::default());
        down_view.push(Default::default());
    }

    println!("Width: {}, Height: {}", width, height);

    // Brute force, for each position, get the view
    let mut max = 0;
    for row in 0..height {
        for col in 0..width {
            let score = get_view(row, col, height, width, &heights);
            max = std::cmp::max(max, score);
        }
    }
    println!("Max: {}", max);
}

fn main() {
    let fname = env::args().nth(1).expect("Should pass 1 filename arg");
    let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}
