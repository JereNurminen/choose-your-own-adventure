use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, stdout, Read, Write};
use toml;

type PageId = String;

#[derive(Deserialize, Debug)]
struct Story {
    pages: HashMap<PageId, Page>,
}

#[derive(Deserialize, Debug)]
struct Page {
    content: String,
    choices: Option<Vec<Choice>>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
    to: PageId,
}

struct GameState {
    current_page: PageId,
}

fn read_file(path: &String) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn parse_story(source: &String) -> Result<Story, toml::de::Error> {
    toml::from_str(source)
}

fn validate_paths(story: &Story) -> Result<(), String> {
    for page in &story.pages {
        match &page.1.choices {
            Some(referenced_pages) => {
                for referenced_page in referenced_pages {
                    if !story.pages.contains_key(&referenced_page.to) {
                        return Err(format!(
                            "page {} references nonexistent page {}",
                            page.0, referenced_page.to
                        ));
                    }
                }
            }
            None => continue,
        }
    }
    Ok(())
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
    let story = parse_story(&file_content).expect("parsing the file failed");
    match validate_paths(&story) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }

    let mut state = GameState {
        current_page: "start".to_string(),
    };
    let mut user_input = String::new();
    let stdin = std::io::stdin();
    let mut error_msg = String::new();

    loop {
        let page = story
            .pages
            .get(&state.current_page)
            .expect("page not found");

        user_input = "".to_string();
        clear_screen();
        write(&error_msg);
        write(&page.content);

        // no choices means we've reached the story's last page
        let choices = match &page.choices {
            Some(options) => options,
            None => {
                break;
            }
        };

        for (i, choice) in choices.iter().enumerate() {
            write(&format!("{}. {}", i + 1, choice.text))
        }

        std::io::stdout().flush();
        stdin.read_line(&mut user_input);

        //write(&format!("'{}'", user_input));
        let input = match user_input.trim().parse::<usize>() {
            Ok(valid_int) => {
                if valid_int > 0 && valid_int <= choices.len() {
                    valid_int
                } else {
                    error_msg = "Choose from the options given".to_string();
                    continue;
                }
            }
            Err(e) => {
                error_msg = "Give a valid number".to_string();
                continue;
            }
        };

        state.current_page = choices[input - 1].to.clone();
    }
}
