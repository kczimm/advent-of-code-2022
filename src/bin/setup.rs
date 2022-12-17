use std::{fs::File, io::Write, path::PathBuf};

fn main() {
    const NUM_DAYS: usize = 25;

    let mut lib_file = File::create("src/lib.rs").expect("create lib file");

    for day in 1..=NUM_DAYS {
        lib_file
            .write_all(format!("pub mod day{day};\n").as_bytes())
            .expect("write pub use mod");
        let directory = PathBuf::from(format!("src/day{day}"));
        if !directory.is_dir() && !directory.exists() {
            std::fs::create_dir(&directory).expect("create directory");
            let mut mod_file = File::create(&directory.join("mod.rs")).expect("create file");
            mod_file
                .write_all(SOURCE_TEMPLATE.as_bytes())
                .expect("write mod file");
            File::create(&directory.join("input.txt")).expect("create input.txt");
        }
    }
}

static SOURCE_TEMPLATE: &str = r#"static INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        todo!()
    }
}
"#;
