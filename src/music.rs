pub mod ness {
    use mysql::prelude::*;
    use mysql::*;
    use rodio::{source::Source, Decoder, OutputStream};
    use serde::{Deserialize, Serialize};
    use std::fs::{File, ReadDir};
    use std::io::BufReader;
    use std::path::Path;
    use std::process::exit;
    use std::string::String;
    use dirs::audio_dir;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Albums {
        artist: String,
        album: String,
        track: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Music {}

    impl Music {
        ///
        /// # Return the length of founded results
        ///
        /// - `directory`  The directory to count   
        ///
        pub fn founded(directory: &str) -> usize {
            let paths = std::fs::read_dir(directory).unwrap();

            let length = paths.count();

            if length > 1 {
                println!("Found {} results", length);
            } else {
                println!("Found {} result", length);
            }
            length
        }

        ///
        /// # Play album
        ///
        /// - `directory`   The album dir
        ///
        pub fn play_album(directory: &str) {
            let paths = std::fs::read_dir(directory).unwrap();
            Music::founded(directory);
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

        ///
        /// # Connection to the database
        ///
        pub fn con() -> PooledConn {
            let url = format!(
                "mysql://{}:{}@localhost:3306/{}",
                std::env::var("NESS_USERNAME").expect("failed to find NESS_USERNAME"),
                std::env::var("NESS_DBNAME").expect("Failed to find NESS_DBNAME"),
                std::env::var("NESS_PASSWORD").expect("Failed to find NESS_PASSWORD"),
            );
            Opts::try_from(url.as_str()).expect("failed to connect to the database");
            let pool = Pool::new(url.as_str()).expect("");
            pool.get_conn().expect("")
        }

        ///
        /// # Connection to the database
        ///
        pub fn root() -> PooledConn {
            let url = format!(
                "mysql://{}:{}@localhost:3306",
                std::env::var("ROOT_USERNAME").expect("failed to find ROOT_USERNAME"),
                std::env::var("ROOT_PASSWORD").expect("Failed to find ROOT_PASSWORD"),
            );
            Opts::try_from(url.as_str()).expect("failed to connect to the database");
            let pool = Pool::new(url.as_str()).expect("");
            pool.get_conn().expect("")
        }

        pub fn create_database() {
            let mut con = Music::root();
            let database = format!(
                "CREATE DATABASE IF NOT EXISTS {}  COLLATE = 'utf8mb4_unicode_ci';",
                std::env::var("NESS_DBNAME").expect("failed to find ness dbname")
            );
            let user = format!(
                "CREATE USER '{}'@'localhost' IDENTIFIED BY '{}';",
                std::env::var("NESS_USERNAME").expect("failed to find ness dbname"),
                std::env::var("NESS_PASSWORD").expect("failed to get ness user password")
            );
            let grant = format!(
                "GRANT ALL PRIVILEGES ON {}.* TO '{}'@localhost IDENTIFIED BY '{}';",
                std::env::var("NESS_DBNAME").expect("Failed to get ness dbname"),
                std::env::var("NESS_USERNAME").expect("failed to find ness dbname"),
                std::env::var("NESS_PASSWORD").expect("failed to get ness user password")
            );
            let flush = "FLUSH PRIVILEGES;".to_string();
            con.query_drop(database.leak()).expect("");

            let _ = &con.query_drop(user.leak()).expect("");
            let _ = &con.query_drop(grant.leak()).expect("");
            let _ = &con.query_drop(flush.leak()).expect("");
        }

        pub fn re_init_database() -> bool {
            Music::root().query_drop(
                format!(
                    "DROP DATABASE {}",
                    std::env::var("NESS_DBNAME").expect("Failed to find dbname")
                )
                    .leak(),
            ).expect("failed to drop database");
            Music::root().query_drop(
                format!(
                    "DROP USER '{}'@'localhost'",
                    std::env::var("NESS_USERNAME").expect("Failed to find dbname")
                )
                    .leak(),
            ).expect("failed to delete user");
            Music::create_database();
            Music::save_albums(audio_dir().unwrap().read_dir().expect("failed to get audio dir"));
            true
        }

        ///
        /// # Find track
        ///
        /// - `track` The track name
        ///
        pub fn find_track(track: &str) -> Vec<Albums> {
            Music::con()
                .query_map(
                    format!(
                        "SELECT artist,album,track FROM albums WHERE track LIKE '%{}%'",
                        track
                    ),
                    |(artist, album, track)| Albums {
                        artist,
                        album,
                        track,
                    },
                )
                .expect("")
        }

        ///
        /// # Find album
        ///
        /// - `album`   The album name
        ///
        pub fn find_album(album: &str) -> Vec<Albums> {
            Music::con()
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
                .expect("")
        }

        ///
        /// # Play a track
        ///
        /// - `track`   The track to play
        ///
        pub fn search_and_play_track(track: &str) {
            for x in Music::find_track(track).iter() {
                println!("Listen : {}", &x.track);
                Music::play(&x.track.to_string());
            }
        }

        ///
        /// # Play all album in a directory
        ///
        /// - `albums` The albums directory
        ///
        pub fn search_and_play_album(albums: &str) {
            for mc in Music::find_album(albums).iter() {
                if Path::new(&mc.album.as_str()).is_dir() {
                    Music::play_album(&mc.album);
                }
            }
        }

        ///
        /// # Listen an album
        ///
        /// - `albums` the albums path   
        ///
        pub fn listen(albums: &str) {
            for mc in Music::find_album(albums).iter() {
                Music::play_album(&mc.album);
            }
        }
        ///
        /// # Save albums in the database
        ///
        /// - `dir` The Music dir path
        ///
        pub fn save_albums(dir: ReadDir) {
            let mut con = Music::con();
            con.query_drop(
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

            con.exec_batch(
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

        ///
        /// # Play a track
        ///
        /// - `track`   The track to play
        ///
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
