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
    
    // set variable for target score
    let target: u32 = 69;

    // set variable for total score
    let mut total: u32 = 0;

    // set variable to store score for current round
    let mut round_score: u32 = 0;
    
    // start game loop
    loop {

    
    

    

    // if a 1 is rolled, no points are awarded
    // does this need a variable?
    let one: u32 = 1;   

    // prompt user to roll
    println!("enter any number to roll");
    

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
        println!("ur total is {}", total);
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
        println!("no points for you");
        println!("ur total: {}", total);
        break;
    } else {
        round_score+=roll_score;
    }

    // print round score
    println!("your score for this round is {}", round_score);

    // if y continue loop, 
    // if n add round_score to total
    println!("keep rolling? enter 0 to continue, enter 1 to end round");

    io::stdin()
        .read_line(&mut keep_rolling)
        .expect("failed to read line");

    if keep_rolling == n {
        
        println!("total score: {}", total);
        total+=round_score;
    }
}
    
}

