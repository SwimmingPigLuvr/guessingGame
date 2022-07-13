// use std::borrow::Borrow;
// use std::cmp;
use std::io;
use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use owo_colors::OwoColorize;
// use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

// TO DO
// final round doubles set new high score and added the roll score an extra time
#[derive(Debug, Clone)]
struct Player {
    name: String,
    score: i32,
    turn_count: i32,
}

fn main() {

    // rules
    println!("\n \n");
    println!("\t{} {} {} {} {}", ("$").blink().green(), ("$").blink_fast().bright_green(), ("  G R E E D  ").bold().blink().bright_cyan(), ("$").blink_fast().bright_green(), ("$").blink().green());
    println!("{}", ("\n\tH O W   T O   P L A Y").blink_fast().bold());
    println!("{}", ("\nPlayers take turns rolling dice,").on_truecolor(23, 23, 23));
    println!("On your turn you may roll as many times as you like.");
    println!("Only when you decide to end your turn, \nwill your turn score be added to your total score");
    println!("{}",("*******BUT DON'T BE GREEDY*******").on_truecolor(23, 123, 123));
    println!("If you roll a 1, your turn ends and you get 0 points.");
    println!("If you roll Snake Eyes, your turn ends and your total score goes to 0!");
    println!("Doubles are multiplied by 2!");
    println!("e.g: rolling two 6's would score you 24 points");
    println!("{}", ("first player to 100 points wins").black().on_yellow());

    println!("{}", ("\nTYPE 'roll' TO ROLL").on_bright_blue());

    println!("{}", ("\nHow many players will be playing?").bright_blue());
    
    
    
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
        println!("{} {}, ENTER YOUR NAME", ("PLAYER").on_bright_magenta().bold(), (i).bright_magenta().bold());
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


    // Using Closures that Capture Their Environment
    // https://doc.rust-lang.org/book/ch13-02-iterators.html

    // this function creates a vec of Players who have a certain turn_count left
    fn last_turns (players: Vec<Player>, turn_count: i32) -> Vec<Player> {
        players.into_iter().filter(|p| p.turn_count == turn_count).collect()
    }

    

    // since i only have 1 turn loop inside the game loop
    // i have to make sure that the i value changes after a player's turn ends
    let mut i: usize = 0;
    'game: loop {
        
        let mut turn_scores: Vec<i32> = vec![0; p_num.try_into().unwrap()];
        
        'turn: loop {
            
            let rp_index: usize = gen_prompt_num();
            let ro_index: usize = gen_ro_prompt();
            println!("{}: {}", pvec[i].name.trim().black().bold().on_bright_red(), random_prompts[rp_index].cyan());
            let roll1 = dice_roll();
            let roll2 = dice_roll();
            let mut keyboard_roll = String::new();
            io::stdin().read_line(&mut keyboard_roll).expect("cant read that");
            if keyboard_roll.trim().contains("roll") {
                println!("{} & {}", roll1.black().on_white().bold(), roll2.black().on_white().bold());

                // rolling snake eyes
                if roll1 == 1 && roll2 == 1 {
                    println!("{}", ("SNAKE EYES").on_bright_magenta());
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
                    println!("{}", ("DOUBLES").bright_yellow());
                    turn_scores[i] += roll1*4;
                    println!("{}: {} {}", pvec[i].name.trim().red(), ("your turn score is:").cyan(), turn_scores[i].bold().on_bright_cyan());
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
                    println!("{}, your turn score is {}", pvec[i].name.trim().on_cyan(), turn_scores[i].red());
                    println!("ROLL AGAIN? {}", ("[y,n]").black().on_white());
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
        // end of turn loop
        
        // check if anyone has won

        // temp changed to 30 so i can test the end game in terminal

        // if last player wins then this loop is unessecary
        if pvec[i].score >= 30 {
            println!("CONGRATS {}", pvec[i].name);
            println!("***YOU WIN***");
            if i == (p_num - 1).try_into().unwrap() {
                break 'game;
            } else {
            println!("...but not so fast");
            println!("you won in {} turns, so every player who hasn't had {} turns gets to try one last time", pvec[i].turn_count, pvec[i].turn_count);
            
            let total_turns_minus_one = pvec[i].turn_count - 1;
            let cloned_pvec = pvec.clone();
            let mut final_round_players = last_turns(cloned_pvec, total_turns_minus_one);

            let mut v: usize = 0;
            let mut high_scores: Vec<Player> = vec![pvec[i].clone()];
            loop {
            'final_turn: loop {
                if final_round_players[v].score > pvec[i].score {
                println!("{} you have surpassed {}'s score", final_round_players[v].name.trim().bold().on_cyan(), pvec[i].name.trim().bold().on_bright_magenta());
                high_scores.pop();
                high_scores.push(final_round_players[v].clone());
                println!("You set the new high score!");
                }
                println!("last chance {}, score {}", final_round_players[v].name.trim().bright_green().bold(), final_round_players[v].score.on_bright_green());
                println!("type 'milady' to roll for the last time");
                let mut milady = String::new();
                io::stdin().read_line(&mut milady).expect("cant read");
                if milady.trim().contains("milady") {

                let roll1 = dice_roll();
                let roll2 = dice_roll();
                println!("You");
                println!("rolled");
                println!("...");
                println!("{} + {}", roll1.red().on_bright_white(), roll2).red().on_white();
                if roll1 == 1 && roll2 == 1 {
                    println!("wow {} you are very unlucky", final_round_players[v].name.trim().bold().bright_green());
                    v += 1;
                    break 'final_turn
                } else if roll1 == 1 || roll2 == 1 {
                    println!("better luck next time. thanks for playing {}", final_round_players[v].name.trim().bright_yellow().on_bright_purple());
                    v += 1;
                    break 'final_turn
                } else if roll1 == roll2 {
                    println!("fuck yeah lets fucking go thats good thats real good keep doing that");
                    final_round_players[v].score += roll1*4;
                    println!("your score is {}, {} points away!", final_round_players[v].score.yellow().bold(), pvec[i].score - final_round_players[v].score);
                println!("keep rolling? \ny or n");
                let mut yon = String::new();
                io::stdin().read_line(&mut yon).expect("error can't read that");
                let noy: bool = yon.contains("y");
                if noy == true {
                    continue 'final_turn;
                } else {
                    v += 1;
                    break 'final_turn;
                }
                // remember to let players win on last round
            }
                 else {
                final_round_players[v].score += roll1 + roll2;
                println!("your score is {}, {} points away!", final_round_players[v].score.yellow().bold(), pvec[i].score - final_round_players[v].score);
                if final_round_players[v].score > pvec[i].score {
                println!("{} you have surpassed {}'s score", final_round_players[v].name.trim().bold().on_cyan(), pvec[i].name.trim().bold().on_bright_magenta());
                high_scores.pop();
                high_scores.push(final_round_players[v].clone());
                println!("You set the new high score!");
                println!("keep rolling? \ny or n");
                let mut yon = String::new();
                io::stdin().read_line(&mut yon).expect("error can't read that");
                let noy: bool = yon.contains("y");
                if noy == true {
                    continue 'final_turn;
                } else {
                    v += 1;
                    break 'final_turn;
                }
                
            }
                continue 'final_turn;
            }
        }
            // end final turn loop


                if v == final_round_players.len() - 1 {

                    
println!("{:?} is the high score", high_scores);
                    break 'game;
                } else {
                    v +=1;
                }
            } 
            // end final game loop
        }

        }
        // end turn loop
        }
        
        // use filter method to create vec of structs with given attribute
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
}
    // end game loop
// find high score







  