#[cfg(target_os = "windows")]
static DOUBLE_NEWLINE: &str = "\r\n\r\n";
#[cfg(target_os = "linux")]
static DOUBLE_NEWLINE: &str = "\n\n";

pub mod day1;
