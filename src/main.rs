mod find;
mod music;
mod weather;

use crate::music::ness::Music;

use dirs::audio_dir;
use std::env::{args, current_dir};
use std::process::exit;

use crate::find::ness::Find;
use crate::weather::show_weather;

fn first(args: &[String], expected: &String) -> bool {
    args[1].eq(expected)
}

fn help(args: &[String]) -> i32 {
    println!(
        "{} init            : Init the {} database",
        args[0],
        std::env::var("NESS_DBNAME")
            .expect("failed to find ness database name")
            .as_str()
    );
    println!(
        "{} --re-init       : re init the {} database",
        args[0],
        std::env::var("NESS_DBNAME")
            .expect("failed to find ness database name")
            .as_str()
    );
    println!(
        "{} --listen        : Listen the content of the {} directory",
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

    println!(
        "{} --listen-track  : Listen the track in the {} directory",
        args[0],
        audio_dir()
            .expect("failed to find audio dir")
            .to_str()
            .expect("")
    );

    println!(
        "{} --listen-album  : Listen the album in the {} directory",
        args[0],
        audio_dir()
            .expect("failed to find audio dir")
            .to_str()
            .expect("")
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
            } else if first(args, &"--weather".to_string()) {
                show_weather().await;
            } else if first(args, &"init".to_string()) {
                Music::create_database();
                exit(0);
            } else if first(args, &"--re-init".to_string()) {
                Music::re_init_database();
                exit(0);
            }
        }
        3 => {
            if first(args, &"--listen-track".to_string()) {
                Music::search_and_play_track(args[2].as_str());
            } else if first(args, &"--listen-album".to_string()) {
                Music::search_and_play_album(args[2].as_str());
            } else if first(args, &"--edit".to_string()) {
                if Find::edit_file(".", format!("./{}", &args[2]).as_str()) {
                    println!(
                        "{} has been successfully modified in the {} directory",
                        &args[2],
                        current_dir().expect("").display()
                    );
                } else {
                    println!(
                        "{} has not been successfully modified in the {} directory",
                        &args[2],
                        current_dir().expect("").display()
                    );
                }
            }
        }
        4 => {
            if first(args, &"--edit".to_string()) {
                if Find::edit_file(&args[2].to_string(), &args[3]) {
                    println!(
                        "{} has been successfully modified in the {} directory",
                        &args[3],
                        &args[2],

                    );
                } else {
                    println!(
                        "{} has not been successfully modified in the {} directory",
                        &args[3],
                        &args[2],
                    );
                }
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
