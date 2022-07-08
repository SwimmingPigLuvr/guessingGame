use std::io;
use rand::thread_rng;
use rand::Rng;

// TO DO

// GAME
// total score not being recorded
// turn counter that makes sure each player gets the same # of turns
// user sets amount of players and names of players: create player function
// create player struct 
// game loop points to player struct values

// FRONT END
// research rust -> react
// create assets in photoshop/illustrator/after effects

// SOLANA
// use anchor to bring game into solana
// features:
// connect w friends
// set wager amount
// 5% of winnings go back to community wallet


// problem: adds turn score from previous turn to next turn







// pub struct Scores {
//     total: u32,
//     turn: u32,
//     roll: u32
// }
fn main() {
    fn dice_roll() -> i32 {
        // dice simulation
        let mut rng = thread_rng();
        let roll: i32 = rng.gen_range(1..7);
        roll
    }
// fn check_response(&response) -> u8 {
//     match response {
//         String::from("y") => 1,
//         String::from("n") => 0,
//     }
// }
        let mut p1_score = 0;
        // let mut p1_turn_count: i32= 0;

        let mut p2_score = 0;
        // let mut p2_turn_count: i32 = 0;
        
        // let mut p3_score = 0;
        // let mut p3_turn_score = 0;
        // let mut p3_turn_count: i32 = 0;

    'game: loop {
        let mut p1_turn_score = 0;
        let mut p2_turn_score = 0;

        if p1_score >= 100 {
        println!("Player 1 wins");
        }

        if p2_score < p1_score && p1_score >= 100 {
            println!("p1 wins");
            break 'game;
        }
        
        'turn1: loop {
            println!("Player1, type 'roll' to roll");
            let roll1 = dice_roll();
            let roll2: i32 = dice_roll();
            let mut virtual_dice_roll = String::new();
            io::stdin()
                .read_line(&mut virtual_dice_roll)
                .expect("not sure what that was");
            if virtual_dice_roll.contains("roll") {
            println!("p1 rolled: {} & {}", roll1, roll2);
            if roll1 == 1 && roll2 == 1 {
                p1_score *= 0;
                println!("snake eyes! your score is now {}", p1_score);
                // p1_turn_count += 1;
                break 'turn1;
            } else if roll1 ==1 || roll2 == 1 {
                println!("0 points for this turn!");
                println!("{} is your total score", p1_score);
                // p1_turn_count += 1;
                break 'turn1;
            } else if roll1 == roll2 {
                println!(" double {}'s! good job", roll1);
                println!("that counts for {} points", roll1*4);
                p1_turn_score += (roll1)*4;
            println!("p1 score for this turn: {} keep rolling? [y,n]", p1_turn_score);
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .expect("can't read");
            let binary = response.contains("y");
            if binary == true {
                continue 'turn1;
            } else {
                p1_score += p1_turn_score;
                println!("p1 total score: {}", p1_score);
                if p1_score >= 100 {
                    println!("YOU WIN!");
                    println!("...but not so fast p2 gets one last chance");
                    break 'turn1;
                }
                // p1_turn_count += 1;
                break 'turn1;
            }
            }
            p1_turn_score += roll1 + roll2;
            println!("p1 score for this turn: {} keep rolling? [y,n]", p1_turn_score);
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .expect("can't read");
            let binary = response.contains("y");
            if binary == true {
                continue 'turn1;
            } else {
                p1_score += p1_turn_score;
                println!("p1 total score: {}", p1_score);
                if p1_score >= 100 {
                    println!("YOU WIN!");
                    println!("...but not so fast p2 gets one last chance");
                    break 'turn1;
                }
                break 'turn1;
            }
        }
    }
        'turn2: loop {
            println!("Player2, type 'roll' to roll");
            let roll1 = dice_roll();
            let roll2: i32 = dice_roll();

            // check if roll is entered
            let mut virtual_dice_roll = String::new();
            io::stdin()
                .read_line(&mut virtual_dice_roll)
                .expect("not sure what that was");
            if virtual_dice_roll.contains("roll") {
            println!("p2 rolled: {} & {}", roll1, roll2);

            // compare dice values to special cases
            // snake eyes
            if roll1 == 1 && roll2 == 1 {
                p2_score *= 0;
                println!("snake eyes! your total score is now {}", p2_score);
                break 'turn2;
            // roll a 1
            } else if roll1 ==1 || roll2 == 1 {
                println!("0 points for this turn!");
                println!("{} is your total score", p2_score);
                break 'turn2;
            // roll doubles
            } else if roll1 == roll2 {
                println!(" double {}'s!", roll1);
                println!("that counts for {} points", roll1*4);
                p2_turn_score += (roll1)*4;

            // tell player how many points they have
            // ask if they want to continue
            println!("p2 score for this turn: {} keep rolling? [y,n]", p2_turn_score);
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .expect("can't read");
            let binary = response.contains("y");
            // if yes, continue loop
            if binary == true {
                continue 'turn2;
            // if no, add turn score to total and end turn
            } else {
                p2_score += p2_turn_score;
                println!("p2 total score: {}", p2_score);
                // if they are at 100 they win
                if p2_score >= 100 {
                    println!("you win");
                    break 'game;
                }
                break 'turn2;
            }
            }
            p2_turn_score += roll1 + roll2;
            println!("p2 score for this turn: {} keep rolling? [y,n]", p2_turn_score);
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .expect("can't read");
            let binary = response.contains("y");
            if binary == true {
                continue 'turn2;
            } else {
                p2_score += p2_turn_score;
                println!("p2 total score: {}", p2_score);
                if p2_score >= 100 && p2_score > p1_score {
                    println!("you win");
                    break 'game;
                }
                break 'turn2;
            }
        }
    }

        

        
    }
}





// game needs to:
//
// know whose turn it is, (player)
// roll the dice and add score to player's total, (total score)
// ask player if they want to go again,
// either go again or pass the dice to the next player,
// repeat process until any player reaches total score of 100,
// allow players same number of turns,
// if player 1 reaches 100,
// then players 2 & 3 get 1 turn to score higher than player 1
//
//
// players have names, total scores, and turn counters
//     names: taken from user input
//     turn counter: tracks how many turns player has had
//     total scores: sum of turn scores
//         turn scores: sum of dice scores
//             dice scores: sum of 2 dice rolled
//
// game
//     turn
//         dice_roll
//             if 1 is rolled 
//                 turn score = 0
//                 end turn,
//             if snake eyes
//                 total score = 0
//                 end turn,
//             if doubles
//                 roll score *= 2
//                 turn score += roll score
//                 roll again?
//             else
//                 turn score += roll score
//                 roll again?
// 
//         roll again? [y,n]
//             if y -> dice_roll
//             if n -> end turn
//                 end turn: total score += turn score


// how i want it to loop
// turn 1 => player1
// turn 2 => player2
// turn 3 => player
  