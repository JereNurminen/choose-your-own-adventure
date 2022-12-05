use std::{error::Error, io::Write};

use cyoa::{game, parsing};

fn read_file(path: &String) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn write(output: &String) {
    let mut stdout = std::io::stdout();
    stdout
        .write_all(format!("{}\n\n", output).as_bytes())
        .expect("unexpected error when writing to terminal");
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let file_arg = std::env::args().nth(1);
    let file_path = file_arg.expect("path to story file is missing");
    let file_content = read_file(&file_path).expect("reading story file failed");
    let story = parsing::parse_story(&file_content).expect("parsing story failed");

    let stdin = std::io::stdin();
    let mut user_input;
    let mut error_toast = String::new();

    let mut game = game::Game::new(&story).expect("starting game with this story failed");

    loop {
        let current_page = game.get_current_page().unwrap();

        user_input = "".to_string();
        clear_screen();
        write(&error_toast);
        error_toast = "".to_string();
        write(&current_page.content);
        std::io::stdout().flush().expect("io error");

        match &current_page.choices {
            choices if choices.len() > 0 => {
                for (i, choice) in choices.iter().enumerate() {
                    write(&format!("{}: {}", i + 1, &choice.text));
                }
            }
            _ => break,
        }

        stdin.read_line(&mut user_input).expect("io error");

        let input = match user_input.trim().parse::<usize>() {
            Ok(valid_int) => valid_int - 1,
            Err(_e) => {
                error_toast = "Give a valid number".to_string();
                continue;
            }
        };

        match game.make_choice(input) {
            Ok(_) => continue,
            Err(_) => error_toast = "Invalid choice".to_string(),
        };
    }
}
