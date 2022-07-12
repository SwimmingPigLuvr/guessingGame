use std::io;
use rand::prelude::ThreadRng;
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
    println!("2 players take turns rolling dice,");
    println!("racing to get a score of {}.", ("100").black().on_yellow());
    println!("On your turn you may roll as many times as you like.");
    println!("*******but don't be Greedy!*******");
    println!("If you roll a 1, your turn ends and you get 0 points.");
    println!("If you roll Snake Eyes, your turn ends and your total score goes to 0!");
    println!("Doubles are multiplied by 2!");
    println!("e.g: rolling two 6's would score you 24 points");
    println!("If you roll to 100 or higher on your fifth turn,");
    println!("all players who have not had 5 turns get one last chance to beat your score.");
    println!("all players who have not had 5 turns get one last chance to beat your score.");
    println!("if you reach 100 or higher, you may keep rolling to set a higher total score");

    println!("{}", ("TYPE 'roll' TO ROLL").on_bright_blue());

    println!("{}", ("How many players will be playing?").bright_blue());
    
    
    
    fn set_player(name: String) -> Player {
        Player { name, score: 0, turn_count: 0, }
    }

    let mut p_string = String::new();
    io::stdin().read_line(&mut p_string).expect("cant read");
    let p_string = p_string.trim();
    let p_num: i32 = p_string.parse().unwrap();
    let mut pvec: Vec<Player> = Vec::new();

    let mut i = 1;
    loop {
        println!("PLAYER {i}, ENTER YOUR NAME");
        let mut new_name = String::new();  
        io::stdin().read_line(&mut new_name).expect("cant read");
        let player: Player = set_player(new_name);
        pvec.push(player);

        if i >= p_num.try_into().unwrap() {
            break;
        }
        i += 1;
        
    }
    
    
    
    fn dice_roll() -> i32 {
        // dice simulation
        let mut rng = thread_rng();
        let roll: i32 = rng.gen_range(1..7);
        roll
    }   

    let mut random_prompts: Vec<String> = Vec::new();
    random_prompts.push(String::from("HURRY UP AND ROLL"));
    random_prompts.push(String::from("ROLL THEM SHITS, BITCH"));
    random_prompts.push(String::from("ROLL THE DICE!!!"));
    random_prompts.push(String::from("YOUR TURN"));
    random_prompts.push(String::from("*picks up dice...shakes vigorously...rolls them across the table"));
    random_prompts.push(String::from("GOOD LUCK, HAVE FUN :)"));
    random_prompts.push(String::from("DON'T BE GREEDY"));
    random_prompts.push(String::from("YOU CAN DO IT!!!"));
    random_prompts.push(String::from("your family is counting on you"));
    random_prompts.push(String::from("please roll"));
    random_prompts.push(String::from("your turn to roll now"));
    random_prompts.push(String::from("right now it is your turn to roll now"));
    random_prompts.push(String::from("ROLL THE VIRTUAL DICE"));
    random_prompts.push(String::from("fucking roll already"));
    random_prompts.push(String::from("please roll, milady"));

    let mut random_ones: Vec<String> = Vec::new();
    random_ones.push(String::from("you rolled a 1! no points for this turn"));
    random_ones.push(String::from("get good nerd"));
    random_ones.push(String::from("no points scored because you rolled a 1"));
    random_ones.push(String::from("get rekt"));
    random_ones.push(String::from("HAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHA"));


    

    fn gen_prompt_num() -> usize {
        let mut prompt: ThreadRng = thread_rng();
        let prompt_num: usize = prompt.gen_range(0..13);
        prompt_num
    }

    fn gen_ro_prompt() -> usize {
        let mut ro: ThreadRng = thread_rng();
        let ro_num: usize = ro.gen_range(0..4);
        ro_num
    }

    // since i only have 1 turn loop inside the game loop
    // i have to make sure that the i value changes after a player's turn ends
    let mut i: usize = 0;
    loop {
        
        let mut turn_scores: Vec<i32> = vec![0; p_num.try_into().unwrap()];
        
        'turn: loop {
            
            let rp_index: usize = gen_prompt_num();
            let ro_index: usize = gen_ro_prompt();
            println!("{}: {}", pvec[i].name.trim(), random_prompts[rp_index]);
            let roll1 = dice_roll();
            let roll2 = dice_roll();
            let mut keyboard_roll = String::new();
            io::stdin().read_line(&mut keyboard_roll).expect("cant read that");
            if keyboard_roll.contains("roll") {
                println!("{} & {}", roll1, roll2);

                // rolling snake eyes
                if roll1 == 1 && roll2 == 1 {
                    println!("SNAKE EYES");
                    pvec[i].score *= 0;
                    println!("{}, your total score is {}", pvec[i].name.trim(), pvec[i].score);
                    pvec[i].turn_count += 1;
                    break 'turn;
                
                // rolling a 1
                } else if roll1 == 1 || roll2 == 1 {
                    println!("{}", random_ones[ro_index]);
                    println!("{}, your total score is {}", pvec[i].name.trim(), pvec[i].score);
                    pvec[i].turn_count += 1;
                    break 'turn;

                // rolling doubles
                } else if roll1 == roll2 {
                    println!("DOUBLES!!!");
                    turn_scores[i] += roll1*4;
                    println!("{}, your turn score is {}", pvec[i].name.trim(), turn_scores[i]);
                    println!("ROLL AGAIN? [y,n]");

                    // go again?
                    let mut response = String::new();
                    io::stdin()
                        .read_line(&mut response)
                        .expect("can't read");
                    let binary = response.contains("y");

                    // yes
                    if binary == true {
                        continue 'turn
                    } 
                    // no
                    else {
                        pvec[i].score += turn_scores[i];
                        println!("{}, your total score is {}", pvec[i].name.trim(), pvec[i].score);
                        pvec[i].turn_count += 1;
                        break 'turn;
                    }
                } 
                    // normal roll
                    // go again?
                    turn_scores[i] += roll1 + roll2;
                    println!("{}, your turn score is {}", pvec[i].name.trim(), turn_scores[i]);
                    println!("ROLL AGAIN? [y,n]");
                    let mut response = String::new();
                    io::stdin()
                        .read_line(&mut response)
                        .expect("can't read");
                    let binary = response.contains("y");

                    // yes
                    if binary == true {
                        continue 'turn
                    } 
                    // no
                    else {
                        pvec[i].score += turn_scores[i];
                        println!("{}, your total score is {}", pvec[i].name.trim(), pvec[i].score);
                        pvec[i].turn_count += 1;
                        break 'turn;
                    }
            }
            
            
        }
        
        // check if anyone has won
        if pvec[i].score >= 100 {
            println!("CONGRATS {}", pvec[i].name);
            println!("***YOU WIN***");
            println!("...but not so fast");
            println!("you won in {} turns, so every player who hasn't had {} turns gets to try one last time", pvec[i].turn_count, pvec[i].turn_count);
        }

        // give only the remaining players the option to roll 1 last time
        // iter through slice of the vector
        // then see if anyone has gotten a higher score
        // break loop print winner
        // reset turn loop back to player 1
        let p_num_minus_one = p_num - 1;
        if i == p_num_minus_one.try_into().unwrap() {
            i *= 0
        } else {
            i += 1;
        }
        

    }


// ye olde loopty loop
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

  