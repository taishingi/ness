pub mod tess {
    use std::path::{Path, PathBuf};
    use std::process::exit;

    pub struct Find {}

    impl Find {
        pub fn search_file(dir: &String, filename: &String) -> PathBuf {
            let paths = Path::new(dir.as_str()).read_dir().unwrap();

            for path in paths {
                let d: &String = &path.unwrap().path().to_str().unwrap().to_string();
                if Path::new(d).is_dir() {
                    // sql query to save album
                    continue;
                } else {
                    if d.contains(filename) {
                        return Path::new(d.as_str()).to_path_buf();
                    } else {
                        continue;
                    }
                }
            }
            if dir.eq(".") {
                println!(
                    "The filename {} has not been founded in the {} directory",
                    filename, "current"
                );
            } else {
                println!(
                    "The filename {} has not been founded in the {} directory",
                    filename, dir
                );
            }
            exit(1);
        }
    }
}
