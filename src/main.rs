extern crate rand;
use rand::{thread_rng, Rng};
use std::ops::Rem;
use std::time::{Instant};
use std::thread;


#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Black
}

static NTHREADS: i32 = 8;

fn main() {
    let n_games = 1_000_000 * NTHREADS;
    let start = Instant::now();

    let mut children = vec![];

    for _ in 0..NTHREADS {
        children.push(thread::spawn(move || {
            run_games(n_games/NTHREADS)
        }));
    }

    let mut red_wins = 0;
    let mut n_stopped = 0;
    for child in children {
        match child.join() {
            Ok((red_win, n_stop)) => {
                println!("Success");
                red_wins += red_win;
                n_stopped += n_stop;
            },
            Err(_) => print!("Failed thread")
        }
    }

    println!("Red won {:.2}% out of {} Stopped Games: {}%", to_percent(red_wins, n_games), n_games, to_percent(n_stopped, n_games));
    println!("Time: {} sec", start.elapsed().as_secs());
}

fn run_games(n_games: i32) -> (i32, i32) {
    let mut red_wins = 0;
    let mut n_stopped = 0;
    for _ in  0..n_games {

        let deck = shuffle_deck();
        let (winner, stopped) = play(deck);
        if stopped {
        n_stopped += 1;
        }
        match  winner {
        Color::Black => (),
        Color::Red => red_wins += 1,
        }
    }
    return (red_wins, n_stopped);
}

fn play(deck: Vec<Color>) -> (Color, bool) {
    let mut diff = 0;
    let mut stopped = false;
    for i in 0..deck.len()-1 {
        let drawn_card = &deck[i];
        match *drawn_card {
            Color::Red => diff -= 1,
            Color::Black => diff += 1,
        }
        if diff >= 5{
//            println!("winning odds: {} n:{} deck: {:?}", to_percent(count(Color::Red, &deck), deck.len() as i32), deck.len(), deck);
//            let mut str = String::new();
//            io::stdin().read_line(&mut str);
            stopped = true;
            break;
        }
    }
    match deck[deck.len()-1] {
        Color::Red => return (Color::Red, stopped),
        Color::Black => return (Color::Black, stopped),
    }
}

fn shuffle_deck() -> Vec<Color>{
    let mut deck: Vec<u32> = (0..52).collect();
    let deck_slice = deck.as_mut_slice();
    thread_rng().shuffle(deck_slice);

    let mut colors = Vec::new();

    for n in deck_slice {
        if n.rem(2) == 0 {
            colors.push(Color::Red);
        } else {
            colors.push(Color::Black);
        }
    }
    colors
}

fn to_percent(part: i32, whole: i32) -> f32 {
    ((part as f32)/(whole as f32))*(100 as f32)
}

#[allow(dead_code)]
fn count(clr: Color, list: &Vec<Color>) -> i32 {
    let mut res = 0;
    for i in list {
        if *i == clr {
            res += 1;
        }
    }
    res
}
