mod meteo;
mod music;
use crate::music::ness::Music;

use dirs::audio_dir;
use meteo::show_meteo;
use std::env::args;
use std::process::exit;

fn first(args: &Vec<String>, expected: &String) -> bool {
    args[1].eq(expected)
}

fn help(args: &Vec<String>) -> i32 {
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

async fn parse(args: Vec<String>) {
    match args.len() {
        1 => {
            exit(help(&args));
        }
        2 => {
            if first(&args, &"--save-albums".to_string()) {
                Music::save_albums(audio_dir().expect("").read_dir().expect(""));
            } else if first(&args, &"--listen".to_string()) {
                Music::loops(
                    audio_dir()
                        .expect("failed to find audio dir")
                        .to_str()
                        .expect(""),
                );
            } else if first(&args, &"--meteo".to_string()) {
                show_meteo().await;
            }
            {}
        }
        3 => {
            if first(&args, &"--listen".to_string()) {
                Music::loops(&args[2].as_str());
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
    parse(args).await;
}
