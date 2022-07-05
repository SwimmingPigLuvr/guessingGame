use std::io;
use rand::thread_rng;
use rand::Rng;


pub struct Player {
    name: String,
    turn: u32,
    total_score: u32,
    turn_score: u32,
    roll_score: u32,
}

fn dice_roll() -> (u32, u32, u32) {
    // dice simulation
    let mut rng = thread_rng();
    let roll1: u32 = rng.gen_range(1..7);
    let roll2: u32 = rng.gen_range(1..7);
    let mut roll_score = roll1 + roll2;

    // check rules
    if roll1 == 1 {
        roll_score *= 0;
        break;
    }

    if roll2 == 1 {
        roll_score *= 0
    }
    
    if roll1 == roll2 {
        roll_score *= 2
    }

    println!("rolled {} & {}", roll1, roll2);
    println!("score: {}", roll_score);
    
    // return
    (roll1, roll2, roll_score)
}

// impl Game {
//     // ROLL DICE
//     fn dice_roll(&self) {
//         let mut rng = thread_rng();
//         let roll1: u64 = rng.gen_range(1..7);
//         let roll2: u64 = rng.gen_range(1..7);

//         println!("first die shows {}", roll1);
//         println!("second die shows {}", roll2);
//     }

//     fn turn(&self) {}
// }

pub struct Game {
    players: Vec<String>,
    turn: u8,
    totals: (u8, u8, u8),
}

impl Game {
    pub fn start(&mut self) {
        
    }
    
}

// game functions




fn main() {
    println!("----LET'S PLAY GREED----");
    
    let mut player1: Player = Player {
        name: String::new(),
        turn: 1,
        total_score: 0,
        turn_score: 0,
        roll_score: 0
    };
    let mut player2: Player = Player {
        name: String::new(),
        turn: 2,
        total_score: 0,
        turn_score: 0,
        roll_score: 0
    };
    let mut player3: Player = Player {
        name: String::new(),
        turn: 3,
        total_score: 0,
        turn_score: 0,
        roll_score: 0
    };

    println!("Players, state your names:");
    println!("Player 1:");
    io::stdin()
        .read_line(&mut player1.name)
        .expect("wtf was that");
    println!("Player 2:");
    io::stdin()
        .read_line(&mut player2.name)
        .expect("wtf was that");
    println!("Player 3:");
    io::stdin()
        .read_line(&mut player3.name)
        .expect("wtf was that");
    
    
    
    dice_roll();
    
    
    println!("roll again? [y,n]");
    let mut response: String = String::new();
    io::stdin().read_line(&mut response).expect("can't read");
    
    

}

