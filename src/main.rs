extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");

    loop {
        let comp_throw = rand::thread_rng().gen_range(1, 3);

        let rock_win = "Unfortunately, rock beats scissors. Try again.";
        let paper_win = "Unfortunately, paper beats rock. Try again.";
        let scissors_win = "Unfortunately, scissors beats paper. Try again.";
        let hum_win = "Congratulations, you've won!";
        let tie = "Darn, a tie! Try again.";

        println!("Please decide upon your shape.");
        println!("1 = Rock, 2 = Paper, 3 = Scissors");

        let mut hum_throw = String::new();
        io::stdin().read_line(&mut hum_throw)
        	.expect("Failed to read line.");

        let hum_throw: u32 = match hum_throw.trim().parse() {
        	Ok(num)	=> num,
        	Err(_)	=> continue,
        };

        match comp_throw {
            1	=> {
            	if hum_throw == 1 {
            		println!("{}", tie);
            	} else if hum_throw == 3 {
            		println!("{}", rock_win);
            	} else {
            		println!("{}", hum_win);
            		break;
            	}
            }
            2	=> {
            	if hum_throw == 1 {
            	    println!("{}", paper_win);
            	} else if hum_throw == 2 {
            		println!("{}", tie);
            	} else {
            		println!("{}", hum_win);
            		break;
            	}
            }
            3	=> {
            	if hum_throw == 2 {
            		println!("{}", scissors_win);
            	} else if hum_throw == 3 {
            		println!("{}", tie);
            	} else {
            		println!("{}", hum_win);
            		break;
            	}
            }
            _	=> {;}
        }
    }
}
