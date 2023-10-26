mod music;
use crate::music::ness::Music;

use dirs::audio_dir;
use std::env::args;
use std::process::exit;

fn first(args: &[String], expected: &String) -> bool {
    args[1].eq(expected)
}

fn help(args: &[String]) -> i32 {
    println!(
        "{} --listen        : Listen the {} content",
        args[0],
        audio_dir()
            .expect("failed to find audio dir")
            .to_str()
            .expect("")
    );
    println!(
        "{} --save-albums   : Update the {} database",
        args[0],
        std::env::var("NESS_DBNAME").expect("Failed to fin ness dbname")
    );
    1
}

async fn parse(args: &[String]) {
    match args.len() {
        1 => {
            exit(help(args));
        }
        2 => {
            if first(args, &"--save-albums".to_string()) {
                Music::save_albums(audio_dir().expect("").read_dir().expect(""));
            } else if first(args, &"--listen".to_string()) {
                Music::listen(
                    audio_dir()
                        .expect("failed to find audio dir")
                        .to_str()
                        .expect(""),
                );
            }
        }
        3 => {
            if first(args, &"--listen-track".to_string()) {
                Music::search_and_play_track(args[2].as_str());
            } else if first(args, &"--listen-album".to_string()) {
                Music::search_and_play_album(args[2].as_str());
            }
        }
        _ => {
            panic!("")
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();
    parse(&args).await;
}
