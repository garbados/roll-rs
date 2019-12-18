extern crate rand;
extern crate regex;

use rand::Rng;
use regex::Regex;

fn d_n(n: i16) -> i16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, n+1)
}

fn roll(n: i16, m: i16, x: Option<i16>, best: Option<i16>, worst: Option<i16>) -> i16 {
    if n == 0 || m == 0 {
        return 0;
    }
    let mut rolls = vec![0; n as usize].iter().map(|_| { d_n(m) }).collect::<Vec<i16>>();
    rolls.sort();
    if let Some(b) = best {
        if b <= n {
            rolls = rolls[b as usize..].to_vec();
        }
    } else if let Some(w) = worst {
        if w <= n {
            rolls = rolls[0..w as usize].to_vec();
        }
    }
    let sum = rolls.iter().sum::<i16>();
    if let Some(modifier) = x {
        sum + modifier
    } else {
        sum
    }
}

#[test]
fn test_dice_roll() {
    let x = roll(1, 1, Some(2), None, None);
    assert_eq!(x, 3)
}

fn parse(expression: String) -> i16 {
    let exp_re = Regex::new(r"^(?P<n>\d+?)?d(?P<m>\d+)(b(?P<b>\d+?))?(w(?P<w>\d+?))?(?P<x>\-\d+?)?$").unwrap();
    let num_re = Regex::new(r"^(-?\d+)$").unwrap();
    fn str_to_int_option(capture: Option<regex::Match>) -> Option<i16> {
        if let Some(unwrapped) = capture {
            let value = unwrapped.as_str().parse::<i16>().unwrap();
            Some(value)
        } else {
            None
        }
    }
    expression
        .split('+')
        .map(|sub_exp| {
            if exp_re.is_match(&sub_exp) {
                let captures = exp_re.captures(sub_exp).unwrap();
                let n = str_to_int_option(captures.name("n")).unwrap_or(1);
                let m = captures.name("m").unwrap().as_str().parse::<i16>().unwrap();
                let x = str_to_int_option(captures.name("x"));
                let b = str_to_int_option(captures.name("b"));
                let w = str_to_int_option(captures.name("w"));
                roll(n, m, x, b, w)
            } else if num_re.is_match(&sub_exp) {
                let captures = num_re.captures(sub_exp).unwrap();
                str_to_int_option(captures.get(1)).unwrap()
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn test_parse() {
    let roll1 = parse(String::from("1d8"));
    assert!(roll1 >= 1);
    assert!(roll1 <= 8);
    let roll2 = parse(String::from("2d7+2"));
    assert!(roll2 >= 4);
    assert!(roll2 <= 16);
    let roll3 = parse(String::from("1d8+1d6-2"));
    assert!(roll3 > 0);
    assert!(roll3 < 12);
    let roll4 = parse(String::from("3"));
    assert!(roll4 == 3);
    let roll5 = parse(String::from("d8"));
    assert!(roll5 > 1);
    assert!(roll5 <= 8);
}

#[test]
fn test_parse_bad() {
    let roll1 = parse(String::from("hello world"));
    assert!(roll1 == 0);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            println!("Enter at least 1 dice expression, as NdM[+-]XbB?wW?");
            println!("N: Number of dice to roll.");
            println!("M: Number of faces on each die, ex: 6 => d6, 20 => d20.");
            println!("X: Numerical modifier, added to or subtracted from the roll.");
            println!("B: If given, take only the best B dice rolls.");
            println!("W: If given, and B is not given, take only the worst W rolls.");
            println!("");
            println!("Ex: 1d20 => Roll one 20-sided die.");
            println!("Ex: 2d4+2 => Roll two 4-sided dice and add two.");
            println!("Ex: 5d6w4 => Roll five 6-sided dice and only take the highest four.");
            println!("");
            println!("You can enter multiple dice expressions separated by a space.");
            println!("They will each be evaluated separately. For example:");
            println!("");
            println!("  $ {} 1d20+2 1d8+1", &args[0]);
            println!("  1d20+2 => 16");
            println!("  1d8+1 => 4");
        },
        _ => {
            let roll_args = &args[1..];
            for roll_arg in roll_args {
                let roll_value = parse(roll_arg.to_string());
                println!("{} => {}", roll_arg, roll_value);
            }
        }
    }
}
