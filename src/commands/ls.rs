use std::fs;
use std::path::Path;

pub fn execute(maybe_path: &Option<String>, all: bool, mut writer: impl std::io::Write) {
    let path_s = get_path(maybe_path);
    let path = Path::new(&path_s);

    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let unwrapped_entry = entry.unwrap();
        let filename = unwrapped_entry.file_name();
        let f = filename.to_str().unwrap();
        if !f.starts_with(".") || all {
            writeln!(writer, "{}", f).unwrap();
        }
    }
}

fn get_path(maybe_path: &Option<String>) -> String {
    match maybe_path {
        Some(p) => return p.to_string(),
        None => return "./".to_string()
    }
}