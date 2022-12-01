use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path, why),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => (),
        Err(why) => panic!("couldn't read {}: {}", path, why),
    };

    return s;
}