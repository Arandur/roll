#[macro_use] extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate rustyline;

use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};

use regex::Regex;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::env::current_exe;
use std::hint::unreachable_unchecked;

fn parse_roll(text: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?ix)^(?P<n_dice>\d+)d(?P<die>\d+)(?:(?P<op>\+|-)(?P<addend>\d+))?$").unwrap();
    }

    if let Some(caps) = RE.captures(text) {
        let n_dice: usize = caps.name("n_dice").unwrap().as_str().parse().unwrap();
        let die: usize = caps.name("die").unwrap().as_str().parse().unwrap();
        let op = caps.name("op").map_or("+", |m| m.as_str());
        let addend: usize = caps.name("addend").map_or("0", |m| m.as_str()).parse().unwrap();

        let dist = Uniform::from(1..=die);
        let mut rng = thread_rng();
        let mut rolls = Vec::new();

        for _ in 0..n_dice {
            rolls.push(dist.sample(&mut rng));
        }

        rolls.sort();

        let mut sum: usize = rolls.iter().sum();

        match op {
            "+" => sum += addend,
            "-" => sum -= addend,
            _ => unsafe { unreachable_unchecked() }
        }

        println!("{:?} {} {}: total {}", rolls, op, addend, sum);
    } else {
        println!("Cannot parse: {}", text);
    }
}

fn main() {
    let history_path = match current_exe() {
        Ok(mut exe_path) => {
            exe_path.set_file_name(".history");
            Some(exe_path)
        },
        Err(e) => {
            println!("Cannot load history: {}", e);
            None
        }
    };

    let mut rl = Editor::<()>::new();

    if let Some(ref history_path) = history_path {
        if rl.load_history(&history_path).is_err() {
            // We don't actually care
        }
    }

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                parse_roll(&line);
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

    if let Some(history_path) = history_path {
        rl.save_history(&history_path).unwrap();
    }
}
