use std::{thread, time::Duration};

use rand::Rng;

#[derive(Debug)]
struct Player {
    name: char,
    position: u32,
}

impl Player {
    fn roll(&mut self) {
        self.position += random_dice()
    }

    fn decrement(&mut self) {
        self.position -= random_dice()
    }
}

/**
 * Clear screen https://stackoverflow.com/a/62101709/7975543
 */
fn clear_screen() {
    print!("{}[2J", 27 as char)
}

fn print_winner(player: char) {
    println!("Player \"{}\" is the winner", player)
}

fn print_board(players: &Vec<Player>, total_track: u32) {
    let mut content = String::new();

    for player in players {
        for track in 1..=total_track {
            if track == player.position {
                content += &format!("|{}", player.name);
            } else {
                content += "| "
            }
        }
        content += "\n"
    }

    println!("{}", content)
}

fn roll_players(players: &mut Vec<Player>, total_track: u32) {
    players.iter_mut().for_each(|p| {
        p.roll();

        if p.position > total_track {
            p.decrement();
        }
    });
}

fn get_winner(players: &mut Vec<Player>, total_track: u32) -> bool {
    let mut total_winner: Vec<&mut Player> = vec![];

    for player in players {
        if player.position >= total_track {
            total_winner.push(player);
        }
    }

    if total_winner.len() == 1 {
        return true;
    }

    /* Avoid multiple winner by decrement */
    if total_winner.len() > 1 {
        total_winner.iter_mut().for_each(|p| p.decrement())
    }

    return false;
}

fn wait(second: u64) {
    thread::sleep(Duration::from_secs(second));
}

fn random_dice() -> u32 {
    rand::thread_rng().gen_range(1..3)
}

fn create_race(initial_player: u32, track_length: u32) {
    if initial_player < 2 {
        println!("Minimum player should be 2");
        return;
    }

    if initial_player > 26 {
        println!("Maximum player should be 26");
        return;
    }

    if track_length < 15 {
        println!("Minimum track length should be 15");
        return;
    }

    let mut players: Vec<Player> = Vec::new();

    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
    // Fill player identity
    for i in 0..initial_player {
        let player = Player {
            name: alphabet.to_owned().nth(i as usize).unwrap_or_default(),
            position: 1,
        };

        players.push(player);
    }

    print_board(&players, track_length);

    loop {
        roll_players(&mut players, track_length);
        // players.iter_mut().for_each(|p| p.roll());

        wait(1);
        clear_screen();

        print_board(&players, track_length);

        if get_winner(&mut players, track_length) {
            break;
        }
    }

    wait(1);
    clear_screen();

    // println!("{:?}", players);

    print_winner(
        players
            .iter()
            .find(|p| p.position == track_length)
            .to_owned()
            .unwrap_or(&Player {
                name: 'x',
                position: 0,
            })
            .name,
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Minimum argument should be 2");
        return;
    }

    println!("Race will start...");
    wait(1);
    clear_screen();

    let total_player = u32::from_str_radix(&args[1], 10).unwrap_or_default();
    let total_track = u32::from_str_radix(&args[2], 10).unwrap_or_default();

    create_race(total_player, total_track)
}
