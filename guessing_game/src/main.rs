use std::io::{self, Read};
use rand::Rng;
use rand::thread_rng;
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
// player 1, press enter to roll...press enter to roll again
// RR
// you rolled 4 + 5, round score: 9, continue? [y, n]
// n
// your total score is 9
// player 2, press enter to roll...press enter to roll again
// RR
// you rolled 6 + 6, round score: 24, continue? [y, n]
// y
// player 2, press enter to roll...press enter to roll again
// you rolled 1 + 3, round score 0, round over


// SNAKE EYES!
// get fucked

// variables:
// players(for later)

// total score
// round score
// roll score(2dice)
// target score
// roll 1
// roll 2
// rolling a one => {round score = 0}
// rolling doubles => {roll_score * 2}
// rolling snake eyes => {total score * 0}

struct Player {
    name: String,
    their_turn: bool,
    score: u64,
    round_score: u64,
    dice_total: u64,
}

impl Player {
    fn dice_roll(&self) -> u64 {
        // let mut rng = thread_rng();
        // let roll1: u64 = rng.gen_range(1..7);
        // let roll2: u64 = rng.gen_range(1..7);
        // println!("first die shows {}", roll1);
        // println!("second die shows {}", roll2);
    }
    
}


fn main() {
    
    let mut player1: Player = Player { 
        name: String::new(), 
        their_turn: true, 
        score: 0, 
        round_score: 0,
        dice_total: 0
    };
    
    let mut player2: Player = Player { 
        name: String::new(), 
        their_turn: false, 
        score: 0, 
        round_score: 0,
        dice_total: 0
    };
    
    let mut player3: Player = Player { 
        name: String::new(), 
        their_turn: true, 
        score: 0, 
        round_score: 0,
        dice_total: 0
    };
    
    
    // player 1 name input
    println!("Player 1, enter your name");
    io::stdin()
        .read_line(&mut player1.name)
        .expect("cant read");

    // player 2 name input
    println!("Player 2, enter your name");
    io::stdin()
        .read_line(&mut player2.name)
        .expect("illiterate");

    // player 3 name input
    println!("Player 3, enter your name");
    io::stdin()
        .read_line(&mut player3.name)
        .expect("illiterate");

    println!("Player 1: {}", player1.name);
    println!("Player 2: {}", player2.name);
    println!("Player 3: {}", player3.name);
    

}

fn build_player(name: String) -> Player {
        Player {
            name: String::new(),
            their_turn: false,
            score: 0,
            round_score: 0,
            dice_total: 0,
        }
    }
