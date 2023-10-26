mod find;
mod music;
mod meteo;

use crate::find::tess::Find;
use crate::music::tess::Music;
use crate::meteo::show_meteo;
use dirs::audio_dir;
use std::env::args;
use std::process::exit;


#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        exit(1);
    }


    if args.len() == 2 && args[1].eq("--meteo") {
        show_meteo().await;
        exit(0);   
    }
    if args.len() == 2 && args[1].eq("--save-albums") {
        Music::save_albums(audio_dir().expect("").read_dir().expect(""));
    }

    if args.len() == 4 && args[1].eq("--find") {
        if args[2].is_empty() {
            exit(1);
        } else {
            let file = Find::search_file(&args[2], &args[3]);
            println!("{}", file.display());
        }
    }

    if args.len() == 2 {
        if args[1].eq("--listen") {
            Music::loops(
                audio_dir()
                    .expect("failed to find audio dir")
                    .to_str()
                    .expect(""),
            )
        }
    }

    if args.len() == 3 {
        if args[1].eq("--find-album") {
            if args[2].is_empty() {
                exit(1);
            } else {
                Music::find_album(&args[2]);
            }
        }
        if args[1].eq("--listen") {
            if args[2].is_empty() {
                exit(1);
            } else {
                Music::loops(&args[2]);
            }
        }
    }
}
