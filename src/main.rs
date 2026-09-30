use clap::Parser;
use rand::Rng;
use rand::seq::IndexedRandom;
use zxcvbn::zxcvbn;

#[derive(Parser, Debug)]
#[command(version, about = "Password generator and evaluator")]
struct Args {
    #[arg(short, long)]
    password: Option<String>,
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%&*_+-=";
const LEN: usize = 15;

fn generate(len: usize, rng: &mut impl Rng) -> String {
    let bytes: Vec<u8> = (0..len).map(|_| *CHARSET.choose(rng).unwrap()).collect();
    String::from_utf8(bytes).unwrap()
}

fn is_valid(p: &str) -> bool {
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;

    for c in p.chars() {
        if c.is_ascii_uppercase() {
            has_upper = true;
        } else if c.is_ascii_lowercase() {
            has_lower = true;
        } else if c.is_ascii_digit() {
            has_digit = true;
        } else {
            has_special = true;
        }

        if has_upper && has_lower && has_digit && has_special {
            return true;
        }
    }

    false
}

fn main() {
    let args = Args::parse();

    match args.password {
        Some(password) => {
            let estimate = zxcvbn(&password, &[]);
            println!(
                "Your password {password} has score {} (from 0 to 4)",
                estimate.score()
            );
        }
        None => {
            let mut rng = rand::rng();
            loop {
                let password = generate(LEN, &mut rng);
                if !is_valid(&password) {
                    continue;
                }
                let estimate = zxcvbn(&password, &[]);
                if u8::from(estimate.score()) == 4 {
                    println!("Password: {password}\nScore: 4");
                    break;
                }
            }
        }
    }
}
