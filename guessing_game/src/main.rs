use std::io::{self, Read};
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
fn main() {

    // is it player ones turn? if so then we start the player 1 loop
    let mut player_one_turn: u32 = 1;

    // set variable for target score
    let target: u32 = 69;

    // set variable for total score
    let mut player_one_total: u32 = 0;
    let mut player_two_total: u32 = 0;

    // set variable to store score for current round
    let mut round_score: u32 = 0;

    // loop game logic for player one
    while player_one_turn == 1 {

        // start game loop
        loop {

        // if a 1 is rolled, no points are awarded
        // does this need a variable?
        let one: u32 = 1;   

        // prompt user to roll
        println!("P1: enter number to roll");
        
        // take input as roll_1
        let mut roll_1 = String::new();
        
        // take user input
        io::stdin()
            .read_line(&mut roll_1)
            .expect("failed to read");

        // change input to u32
        let roll_1: u32 = match roll_1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // declare value of roll 1 by multiplying by 0 then doing actual roll
        let die_1_val: u32 = roll_1 * 0 + rand::thread_rng().gen_range(1..7);

        // print result
        println!("P1: you rolled a {}.", die_1_val);

        // check to see if roll = 1 (round over)
        if die_1_val == one {
            println!("P1: no points for this round");
            println!("P1: total: {}", player_one_total);
            player_one_turn = 0;
        }

        // roll 2
        println!("P1: roll again.");


        // input = roll die 2
        let mut roll_2 = String::new();

        // take user input
        io::stdin()
            .read_line(&mut roll_2)
            .expect("failed to read");

        // change input to u32
        let roll_2: u32 = match roll_2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // die val 2 = multiply input by 0 then add 1-6
        let die_2_val: u32 = roll_2 * 0 +rand::thread_rng().gen_range(1..7);

        // print result
        println!("P1: you rolled a {}.", die_2_val);

        // assign roll score to sum of dice values
        let roll_score:u32 = die_1_val + die_2_val;

        // check to see if roll = 1 (round over)
        if die_2_val == one {
            println!("P1: no points for this round.");
            println!("P1 total: {}.", player_one_total);
            player_one_turn = 0;
        } else {
            round_score+=roll_score;
        }

        // print round score
        println!("P1: current round score: {}, total score: {}.", round_score, player_one_total);

        // ask player if they want to keep going
        println!("P1: continue? 0 = no, 1 = yes");

        // set variable to record answer
        let mut answer: String = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("failed to read");

        let answer: u32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        
        // if p1 wants to end round
        if answer < 1 {
            player_one_total+=round_score;
            println!("your total score is {}", player_one_total);

            player_one_turn = 0;
        }
        


    }

    // player one turn closing bracket
    } 
    //  player two start
    while player_one_turn == 0 {

        // start game loop
        loop {

        // if a 1 is rolled, no points are awarded
        // does this need a variable?
        let one: u32 = 1;   

        // prompt user to roll
        println!("player 2: enter any number to roll");
        
        // take input as roll_1
        let mut roll_1 = String::new();
        
        // take user input
        io::stdin()
            .read_line(&mut roll_1)
            .expect("failed to read");

        // change input to u32
        let roll_1: u32 = match roll_1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // declare value of roll 1 by multiplying by 0 then doing actual roll
        let die_1_val: u32 = roll_1 * 0 + rand::thread_rng().gen_range(1..7);

        // print result
        println!("you rolled: {}", die_1_val);

        // check to see if roll = 1 (round over)
        if die_1_val == one {
            println!("no points for this round");
            println!("ur total is {}", player_two_total);
            break;
        }

        // roll 2
        println!("enter any number to roll again");


        // input = roll die 2
        let mut roll_2 = String::new();

        // take user input
        io::stdin()
            .read_line(&mut roll_2)
            .expect("failed to read");

        // change input to u32
        let roll_2: u32 = match roll_2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // die val 2 = multiply input by 0 then add 1-6
        let die_2_val: u32 = roll_2 * 0 +rand::thread_rng().gen_range(1..7);

        // print result
        println!("you rolled: {}", die_2_val);

        // assign roll score to sum of dice values
        let roll_score:u32 = die_1_val + die_2_val;

        // check to see if roll = 1 (round over)
        if die_2_val == one {
            println!("P2: 0 points for this round");
            println!("P2: total: {}", player_two_total);
            player_one_turn = 0;
        } else {
            round_score+=roll_score;
        }

        // print round score
        println!("P2: round score: {}", round_score);

        // ask player if they want to keep going
        println!("P2: continue? 0 = no, 1 = yes");

        // set variable to record answer
        let mut answer: String = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("failed to read");

        let answer: u32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        

        if answer < 1 {
            println!("P2: total: {}", player_two_total);
            player_one_turn = 0;
            println!("{}", player_one_turn);
        }
        


    }

    }

// fn main closing bracket
}

