pub mod tess {
    use mysql::prelude::*;
    use mysql::*;
    use rodio::{source::Source, Decoder, OutputStream};
    use serde::{Deserialize, Serialize};
    use std::fs::{File, ReadDir};
    use std::io::BufReader;
    use std::path::Path;
    use std::process::exit;
    use std::string::String;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Albums {
        artist: String,
        album: String,
        track: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Music {}

    impl Music {
        pub fn play_album(directory: &String) {
            let paths = std::fs::read_dir(directory.as_str()).unwrap();

            for path in paths {
                let track = &path.unwrap().path().to_str().unwrap().to_string();
                if Path::new(track).is_dir() {
                    Music::play_album(track);
                } else {
                    println!("Playing {}", track);
                    Music::play(track);
                }
            }
        }

        pub fn find_album(album: &String) -> Vec<Albums> {
            let url = "mysql://tess:tess@localhost:3306/tess";
            Opts::try_from(url).expect("failed to connect to the database");
            let pool = Pool::new(url).expect("");
            let mut conn = pool.get_conn().expect("");

            let find = conn
                .query_map(
                    format!(
                        "SELECT artist,album,track FROM albums WHERE album LIKE '%{}%'",
                        album
                    ),
                    |(artist, album, track)| Albums {
                        artist,
                        album,
                        track,
                    },
                )
                .expect("");
            find
        }

        pub fn search_and_play(p: &String) {
            for mc in Music::find_album(&p).iter() {
                Music::play_album(&mc.album);
            }
        }

        pub fn save_albums(dir: ReadDir) {
            let url = "mysql://tess:tess@localhost:3306/tess";
            Opts::try_from(url).expect("failed to connect to the database");
            let pool = Pool::new(url).expect("");

            let mut conn = pool.get_conn().expect("");
            conn.query_drop(
                r"CREATE TABLE IF NOT EXISTS albums (
                    id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
                    artist  LONGTEXT NOT NULL,
                    album   LONGTEXT NOT NULL,
                    track   LONGTEXT)
                    ",
            )
            .expect("");

            let paths = dir;

            let mut music: Vec<Albums> = vec![];
            for path in paths {
                let track = &path.unwrap().path().to_str().unwrap().to_string();

                if Path::new(track).is_dir() {
                    let album = track;
                    println!("Saving albums name : {}", album);
                    Music::save_albums(Path::new(track).read_dir().unwrap());
                } else {
                    if track.contains(".flac") {
                        let art = Path::new(track)
                            .parent()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .to_str()
                            .expect("");
                        let cd = Path::new(track).parent().unwrap().to_str().expect("");
                        music.push(Albums {
                            artist: art.to_string(),
                            album: cd.to_string(),
                            track: track.to_string(),
                        });
                    }
                    // sql query to save track
                    println!("Saving track {}", track);
                }
            }

            conn.exec_batch(
                r"INSERT INTO albums (artist, album, track)
          VALUES (:artist, :album,:track)",
                music.iter().map(|p| {
                    params! {
                        "artist" => &p.artist,
                        "album" => &p.album ,
                        "track" => &p.track,
                    }
                }),
            )
            .expect("");
        }

        pub fn play(track: &String) {
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

            std::thread::sleep(std::time::Duration::from_secs(t))
        }
    }
}
