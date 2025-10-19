use std::io::Write;

pub fn ask_bool(msg: &str) -> bool {
    loop {
        print!("{} (yes/no): ", msg);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_lowercase();
        match input.as_str() {
            "yes" => break true,
            "no" => break false,
            _ => {
                println!("Некорректный ввод.");
            }
        }
    }
}
