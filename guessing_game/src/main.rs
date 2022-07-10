use std::io;
use rand::thread_rng;
use rand::Rng;
use owo_colors::OwoColorize;
// use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};


// TO DO
// create function to call the entire turn
// call function for each player
// turn_count will be useful so that each player gets equal turns
// e.g. 

struct Player {
    name: String,
    score: i32,
    turn_count: i32,
}

fn main() {

    // rules
    println!("{}", ("**HOW TO PLAY**").blink_fast());
    println!("2 players take turns rolling dice, racing to get a score of {},", ("100").black().on_yellow());
    println!("each player may roll as many times as they want during their turn");
    println!("however, if a 1 is rolled then that player gets no points for the entire turn,");
    println!("if a player rolls Snake Eyes, then their total score goes to 0");
    println!("if doubles are rolled, then those dice are doubled.");
    println!("e.g: rolling 2 6's would score you 24 points");
    println!("if player 1 rolls to 100 first, player 2 has 1 last chance to beat them");
    println!("that way each player has an equal number of turns");
    println!("if player 1 reaches 100 or higher, they may keep rolling to set a higher total");
    println!("so if player 1 decides to stop at a score of 121,");
    println!("player 2 would have to get to 121 or higher to win");
    println!("{}", ("TYPE 'roll' TO ROLL").on_bright_blue());

    println!("How many players will be playing?");
    
    fn set_player(name: String) -> Player {
        Player { name, score: 0, turn_count: 0, }
    }

    let mut p_string = String::new();
    io::stdin().read_line(&mut p_string).expect("cant read");
    let p_string = p_string.trim();
    let p_num: i32 = p_string.parse().unwrap();
    let mut pvec: Vec<i32> = Vec::new();
    for i in 1..=p_num {
        pvec.push(i)
    }
    // this worked ✅
    for i in pvec {
        println!("Player {i}, enter your name:");
        let new_name = String::new();
        let mut i: Player = set_player(new_name);
        io::stdin().read_line(&mut i.name).expect("error");
    }
    
    
    fn dice_roll() -> i32 {
        // dice simulation
        let mut rng = thread_rng();
        let roll: i32 = rng.gen_range(1..7);
        roll
    }   

    'game: loop {
        
        // turn scores stored here
        // create vec for turn scores

        // iterate through pvec 
        // 'turn loop for each iteration of pvec
        

    }


    
// **GAME LOOP PAUSED FOR CONSTRUCTION***
    // 'game: loop {
    //     let mut p1_turn_score = 0;
    //     let mut p2_turn_score = 0;

    //     if p1.score >= 100 {
    //     println!("Player 1 wins");
    //     }

    //     if p2.score < p1.score && p1.score >= 100 {
    //         println!("p1 wins");
    //         break 'game;
    //     }

        
        
    //     'turn1: loop {
    //         println!("{}", "SCORE CHECK!!!".on_red());
    //         println!("{} has {} points", p1.name.trim(), p1.score);
    //         println!("{} has {} points", p2.name.trim(), p2.score);
    //         println!("{}, {}", blue.paint(p1.name.to_ascii_uppercase().trim()), abyss.paint("it's your turn to roll"));
    //         let roll1 = dice_roll();
    //         let roll2: i32 = dice_roll();
            
    //         let mut virtual_dice_roll = String::new();
    //         io::stdin()
    //             .read_line(&mut virtual_dice_roll)
    //             .expect("not sure what that was");
    //         if virtual_dice_roll.contains("roll") {
    //         println!("{} & {}", roll1.black().on_bright_white().blink_fast().bold(), roll2.black().on_white().blink().bold());
    //         if roll1 == 1 && roll2 == 1 {
    //             p1.score *= 0;
    //             println!("snake eyes! your score is now {}", p1.score.on_red());
    //             p1.turn_count += 1;
    //             break 'turn1;
    //         } else if roll1 ==1 || roll2 == 1 {
    //             println!("0 points for this turn!");
    //             println!("total score: {}", yllw.paint(p1.score));
    //             p1.turn_count += 1;
    //             break 'turn1;
    //         } else if roll1 == roll2 {
    //             println!(" double {}'s! good job", roll1);
    //             println!("that counts for {} points", roll1*4);
    //             p1_turn_score += (roll1)*4;
    //         println!("p1 score for this turn: {} keep rolling? [y,n]", p1_turn_score);
    //         let mut response = String::new();
    //         io::stdin()
    //             .read_line(&mut response)
    //             .expect("can't read");
    //         let binary = response.contains("y");
    //         if binary == true {
    //             continue 'turn1;
    //         } else {
    //             p1.score += p1_turn_score;
    //             println!("p1 total score: {}", yllw.paint(p1.score));
    //             if p1.score >= 100 {
    //                 println!("YOU WIN!");
    //                 println!("...but not so fast p2 gets one last chance");
    //                 p1.turn_count += 1;
    //                 break 'turn1;
    //             }
    //             p1.turn_count += 1;
    //             break 'turn1;
    //         }
    //         }
    //         p1_turn_score += roll1 + roll2;
    //         println!("turn score: {} keep rolling? [y,n]", yllw.paint(p1_turn_score));
    //         let mut response = String::new();
    //         io::stdin()
    //             .read_line(&mut response)
    //             .expect("can't read");
    //         let binary = response.contains("y");
    //         if binary == true {
    //             continue 'turn1;
    //         } else {
    //             p1.score += p1_turn_score;
    //             println!("p1 total score: {}", yllw.paint(p1.score));
    //             if p1.score >= 100 {
    //                 println!("YOU WIN!");
    //                 println!("...but not so fast p2 gets one last chance");
    //                 p1.turn_count += 1;
    //                 break 'turn1;
    //             }
    //             p1.turn_count += 1;
    //             break 'turn1;
    //         }
    //     }
    // }
    //     'turn2: loop {
    //         println!("{}, type 'roll' to roll", p2.name.trim());
    //         let roll1 = dice_roll();
    //         let roll2: i32 = dice_roll();

    //         // check if roll is entered
    //         let mut virtual_dice_roll = String::new();
    //         io::stdin()
    //             .read_line(&mut virtual_dice_roll)
    //             .expect("not sure what that was");
    //         if virtual_dice_roll.contains("roll") {
    //         println!("{} rolled: {} & {}", p2.name.trim(), roll1, roll2);

    //         // compare dice values to special cases
    //         // snake eyes
    //         if roll1 == 1 && roll2 == 1 {
    //             p2.score *= 0;
    //             println!("snake eyes! your total score is now {}", p2.score);
    //             p2.turn_count += 1;
    //             break 'turn2;
    //         // roll a 1
    //         } else if roll1 ==1 || roll2 == 1 {
    //             println!("0 points for this turn!");
    //             println!("{} is your total score", p2.score);
    //             p2.turn_count += 1;
    //             break 'turn2;
    //         // roll doubles
    //         } else if roll1 == roll2 {
    //             println!(" double {}'s!", roll1);
    //             println!("that counts for {} points", roll1*4);
    //             p2_turn_score += (roll1)*4;

    //         // tell player how many points they have
    //         // ask if they want to continue
    //         println!("p2 score for this turn: {} keep rolling? [y,n]", p2_turn_score);
    //         let mut response = String::new();
    //         io::stdin()
    //             .read_line(&mut response)
    //             .expect("can't read");
    //         let binary = response.contains("y");
    //         // if yes, continue loop
    //         if binary == true {
    //             continue 'turn2;
    //         // if no, add turn score to total and end turn
    //         } else {
    //             p2.score += p2_turn_score;
    //             println!("p2 total score: {}", p2.score);
    //             // if they are at 100 they win
    //             if p2.score >= 100 {
    //                 println!("you win");
    //                 break 'game;
    //             }
    //             break 'turn2;
    //         }
    //         }
    //         p2_turn_score += roll1 + roll2;
    //         println!("p2 score for this turn: {} keep rolling? [y,n]", p2_turn_score);
    //         let mut response = String::new();
    //         io::stdin()
    //             .read_line(&mut response)
    //             .expect("can't read");
    //         let binary = response.contains("y");
    //         if binary == true {
    //             continue 'turn2;
    //         } else {
    //             p2.score += p2_turn_score;
    //             println!("p2 total score: {}", p2.score);
    //             if p2.score >= 100 && p2.score > p1.score {
    //                 println!("you win");
    //                 break 'game;
    //             }
    //             p2.turn_count += 1;
    //             break 'turn2;
    //         }
    //     }
    // }

        

        
    // }
    // end game loop
}

  