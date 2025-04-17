use crate::utils;

pub fn execute(str: &str, mut writer: impl std::io::Write) {
    writeln!(writer, "{}", utils::environment::replace_environment(str));
}