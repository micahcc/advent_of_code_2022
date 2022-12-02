/*
   --- Day 2: Rock Paper Scissors ---
   The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the
   snack storage, a giant Rock Paper Scissors tournament is already in progress.

   Rock Paper Scissors is a game between two players. Each game contains many rounds; in each
   round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand
   shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper,
   and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

   Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle
   input) that they say will be sure to help you win. "The first column is what your opponent is
   going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the
   Elf is called away to help with someone's tent.

   The second column, you reason, must be what you should play in response: X for Rock, Y for
   Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have
   been carefully chosen.

   The winner of the whole tournament is the player with the highest score. Your total score is the
   sum of your scores for each round. The score for a single round is the score for the shape you
   selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the
   round (0 if you lost, 3 if the round was a draw, and 6 if you won).

   Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the
   score you would get if you were to follow the strategy guide.

   For example, suppose you were given the following strategy guide:

   A Y
   B X
   C Z
   This strategy guide predicts and recommends the following:

   1. In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This
      ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).

   2. In the second round, your opponent will choose Paper (B), and you should choose Rock (X).
      This ends in a loss for you with a score of 1 (1 + 0).

   3. The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

   In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

   What would your total score be if everything goes exactly according to your strategy guide?


*/
use std::env;
use std::fs;

fn main() {
    let fname = env::args().nth(1).expect("Should pass 1 filename arg");
    println!("File Name: {:?}", fname);
    let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

    // aggregate into a vector of elfse
    let mut total_points = 0;
    for line in contents.lines() {
        let mut iter = line.split(' ');
        let lhs = iter.next().expect("should have lhs").trim();
        let rhs = iter.next().expect("should have rhs").trim();

        match rhs {
            "X" => {
                // we choose rock
                total_points += 1;
                match lhs {
                    "A" => {
                        // tie
                        total_points += 3;
                    }
                    "B" => {
                        // us rock them paper, loss
                        total_points += 0;
                    }
                    "C" => {
                        // us rock, them scissor, win
                        total_points += 6;
                    }
                    __ => {}
                }
            }
            "Y" => {
                // we choose paper
                total_points += 2;
                match lhs {
                    "A" => {
                        // us paper, them rock, win
                        total_points += 6;
                    }
                    "B" => {
                        // us paper, them paper, tie
                        total_points += 3;
                    }
                    "C" => {
                        // us paper, them scissors, loss
                        total_points += 0;
                    }
                    __ => {}
                }
            }
            "Z" => {
                // we choose scissors
                total_points += 3;
                match lhs {
                    "A" => {
                        // us scissors, them rock, lose
                        total_points += 0;
                    }
                    "B" => {
                        // us scissors, them paper, win
                        total_points += 6;
                    }
                    "C" => {
                        // us scissors, them scissors, tie
                        total_points += 3;
                    }
                    __ => {}
                }
            }
            _ => {}
        }
    }

    println!("Total {:?}", total_points);

    // --- Part Two ---
    // The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second
    // column says how the round needs to end: X means you need to lose, Y means you need to end the
    // round in a draw, and Z means you need to win. Good luck!"

    // The total score is still calculated in the same way, but now you need to figure out what
    // shape to choose so the round ends as indicated. The example above now goes like this:

    // In the first round, your opponent will choose Rock (A), and you need the round to end in a
    //  draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    // In the second round, your opponent will choose Paper (B), and you choose Rock so you lose
    //  (X) with a score of 1 + 0 = 1.
    // In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6
    //  = 7.
    // Now that you're correctly decrypting the ultra top secret strategy guide, you would get a
    //  total score of 12.

    // Following the Elf's instructions for the second column, what would your total score be if
    // everything goes exactly according to your strategy guide?

    // aggregate into a vector of elfse
    let mut total_points2 = 0;
    for line in contents.lines() {
        let mut iter = line.split(' ');
        let lhs = iter.next().expect("should have lhs").trim();
        let rhs = iter.next().expect("should have rhs").trim();

        match rhs {
            "X" => {
                // we need to lose
                total_points2 += 0;
                match lhs {
                    "A" => {
                        // they choose rock, we choose scissors
                        total_points2 += 3;
                    }
                    "B" => {
                        // they choose paper,we choose rock
                        total_points2 += 1;
                    }
                    "C" => {
                        // they choose scissors, we choose paper
                        total_points2 += 2;
                    }
                    __ => {}
                }
            }
            "Y" => {
                // we need to tie
                total_points2 += 3;
                match lhs {
                    "A" => {
                        // they choose rock, we choose rock
                        total_points2 += 1;
                    }
                    "B" => {
                        // they choose paper, we choose paper,
                        total_points2 += 2;
                    }
                    "C" => {
                        // they choose scissors, we choose scissors,
                        total_points2 += 3;
                    }
                    __ => {}
                }
            }
            "Z" => {
                // we should win
                total_points2 += 6;
                match lhs {
                    "A" => {
                        // them rock, us paper
                        total_points2 += 2;
                    }
                    "B" => {
                        // them paper, us scissors
                        total_points2 += 3;
                    }
                    "C" => {
                        // them scissors, us rock
                        total_points2 += 1;
                    }
                    __ => {}
                }
            }
            _ => {}
        }
    }
    println!("Total {:?}", total_points2);
}
