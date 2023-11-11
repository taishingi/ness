pub mod ness {
    use std::path::Path;
    use std::process::Command;

    pub struct Find {}

    impl Find {
        pub fn find(dir: &str, filename: &str) -> bool {
            let paths = Path::new(dir).read_dir().unwrap();
            for path in paths {
                let d: &String = &path.unwrap().path().to_str().unwrap().to_string();
                if Path::new(d.as_str()).is_dir() {
                    continue;
                } else if d.contains(filename) {
                    return true;
                }
            }
            false
        }

        pub fn get_path(dir: &str, x: &str) -> String {
            let mut p = String::new();
            p.push_str(dir);
            p.push('/');
            p.push_str(x);
            if Path::new(p.as_str()).is_file() && Path::new(p.as_str()).is_absolute() {
                return p;
            }
            String::new()
        }

        pub fn edit_file(dir: &str, filename: &str) -> bool {
            if Find::find(dir, filename) {
                let mut c = Command::new(env!("EDITOR"));
                c.arg(Find::get_path(dir, filename).as_str());
                return c
                    .spawn()
                    .expect("failed to edit file")
                    .wait()
                    .expect("")
                    .success();
            }
            false
        }
    }
}
