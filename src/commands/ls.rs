use std::fs;
use std::path::Path;

pub fn execute(maybe_path: &Option<String>, mut writer: impl std::io::Write) {
    let path_s = get_path(maybe_path);
    let path = Path::new(&path_s);

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    writeln!(writer, "{}", entry.file_name().to_str().unwrap()).unwrap();
                }
            }
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn get_path(maybe_path: &Option<String>) -> String {
    match maybe_path {
        Some(p) => return p.to_string(),
        None => return "./".to_string()
    }
}