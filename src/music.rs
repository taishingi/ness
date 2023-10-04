pub mod tess {
    use std::fs::{File, ReadDir};
    use std::io::BufReader;
    use std::path::Path;
    use rodio::{Decoder, OutputStream, source::Source};
    use std::process::exit;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Music {}

    impl Music {
        pub fn play_album(directory: &String)
        {
            let paths = std::fs::read_dir(directory.as_str()).unwrap();

            for path in paths {
                let track = &path.unwrap().path().to_str().unwrap().to_string();
                if Path::new(track).is_dir() { Music::play_album(track); } else {
                    println!("Playing {}", track);
                    Music::play(track);
                }
            }
        }


        pub fn save_albums(dir: ReadDir)
        {
            let paths = dir;

            for path in paths {
                let track = &path.unwrap().path().to_str().unwrap().to_string();
                if Path::new(track).is_dir() {
                    let album = track;

                    // sql query to save album
                    println!("Saving albums name : {}", album);
                    Music::save_albums(Path::new(track).read_dir().unwrap());
                } else {
                    // sql query to save track
                    println!("Saving track {}", track);
                }
            }
        }

        pub fn play(track: &String)
        {
            if track.is_empty() {
                println!("track name empty");
                exit(1);
            }
            let query = track;
            // Get a output stream handle to the default physical sound device
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open(query.as_str()).unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).expect("a");
            let t = source.total_duration().expect("").as_secs();
            // Play the sound directly on the device
            stream_handle.play_raw(source.convert_samples()).expect("");

            // The sound plays in a separate audio thread,
            // so we need to keep the main thread alive while it's playing.
            std::thread::sleep(std::time::Duration::from_secs(t))
        }
    }
}
