use std::io;
use rand::Rng;
// use std::cmp::Ordering;

// GREED 
// 
// rules:
// player rolls 2 dice,

// the total of both dice are added to the players score FOR THAT ROUND
// player can keep rolling BUT IF A 1 IS ROLLED

//  THAT ROUND DOES NOT GET ADDED TO THEIR TOTAL SCORE

//first player to reach total of 100 is the winner

// doubles will be doubled (i.e. 6+6 => 12+12)

// snake eyes will set your total score to 0




// how many players?
// 1
// player 1, press R to roll...press R to roll again
// RR
// you rolled 4 + 5, round score: 9, continue? [y, n]
// n
// your total score is 9
// player 2, press R to roll...press R to roll again
// RR
// you rolled 6 + 6, round score: 24, continue? [y, n]
// y
// player 2, press R to roll...press R to roll again
// you rolled 1 + 3, round score 0, round over


// SNAKE EYES!
// get fucked

// variables:
// players
// total score
// round score
// target score
// roll 1
// roll 2
// rolling a one
// rolling doubles
// rolling snake eyes
fn main() {
    let mut total_score = 0;
    let mut round_score = 0;
    let mut roll_one = rand::thread_rng.gen_range(1..=6);
    let mut roll_two = rand::thread_rng.gen_range(1..=6);

    println!("roll");
    io::stdin()
        .read_line(&mut roll_one)
        .expect("Failed to read input");
    
    println!("roll again");
    io::stdin()
        .read_line(&mut roll_two);
        .expect("failed to read input");

    let one = 1;
    let snake_eyes = 2;


    
}

