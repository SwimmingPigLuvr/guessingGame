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
#[derive(Debug, Clone)]
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


    // Using Closures that Capture Their Environment
    // https://doc.rust-lang.org/book/ch13-02-iterators.html

    // this function creates a vec of Players who have a certain turn_count left
    fn last_turns (players: Vec<Player>, turn_count: i32) -> Vec<Player> {
        players.into_iter().filter(|p| p.turn_count == turn_count).collect()
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
        if pvec[i].score >= 30 {
            println!("CONGRATS {}", pvec[i].name);
            println!("***YOU WIN***");
            println!("...but not so fast");
            println!("you won in {} turns, so every player who hasn't had {} turns gets to try one last time", pvec[i].turn_count, pvec[i].turn_count);
            
            let total_turns_minus_one = pvec[i].turn_count - 1;
            let cloned_pvec = pvec.clone();
            let final_round_players = last_turns(cloned_pvec, total_turns_minus_one);

            // maybe a for loop isnt the right idea 
            // how will i roll more than once?
            for mut val in final_round_players {
                println!("last chance {}, score {}", val.name.trim().bright_green().bold(), val.score.on_bright_green());
                let roll1 = dice_roll();
                let roll2 = dice_roll();
                println!("You");
                println!("rolled");
                println!("...");
                println!("{} + {}", roll1.black().on_bright_white(), roll2).bright_white().on_black();
                if roll1 == 1 && roll2 == 1 {
                    println!("wow {} you are very unlucky", val.name.trim().bold().bright_green());
                } else if roll1 == 1 || roll2 == 1 {
                    println!("better luck next time. thanks for playing {}", val.name.trim().bright_yellow().on_bright_purple());
                } else if roll1 == roll2 {
                    println!("fuck yeah lets fucking go thats good thats real good keep doing that");
                    val.score += roll1*4;
                    println!("your score is {}, {} points away!", val.score.yellow().bold(), pvec[i].score - val.score);
                }
                val.score += roll1 + roll2;
                println!("your score is {}, {} points away!", val.score.yellow().bold(), pvec[i].score - val.score);

            }
            
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

  