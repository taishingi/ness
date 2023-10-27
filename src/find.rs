pub mod ness {
    use std::path::Path;
    use std::process::Command;

    pub struct Find {}

    impl Find {
        pub fn edit_file(dir: &String, filename: &String) -> bool {
            let paths = Path::new(dir.as_str()).read_dir().unwrap();

            for path in paths {
                let d: &String = &path.unwrap().path().to_str().unwrap().to_string();
                if Path::new(d).is_dir() {
                    // sql query to save album
                    continue;
                } else if d.contains(filename) {
                    let mut c = Command::new(env!("EDITOR"));
                    c.arg(Path::new(d.as_str()));
                    return c.spawn().expect("failed to edit file").wait().expect("").success();
                } else {
                    continue;
                }
            }
            if dir.eq(".") {
                println!(
                    "The filename {} has not been founded in the current directory",
                    filename
                );
                false
            } else {
                println!(
                    "The filename {} has not been founded in the {} directory",
                    filename, dir
                );
                false
            }
        }
    }
}
