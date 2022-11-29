use std::{collections::HashMap, error::Error};

type PageId = String;

struct Story {
    pages: HashMap<PageId, Page>,
}

struct Page {
    id: PageId,
    content: String,
    choices: Vec<Choice>,
}

struct Choice {
    text: String,
    to: PageId,
}

fn read_file(path: &String) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn compile_lines_to_pages(lines: std::str::Lines) {
    let mut page_in_progress: Vec<String> = Vec::new();
    let pages: Vec<Vec<String>> = Vec::new();

    for line in lines {
        match line.chars()[0] {
            "#" => page_in_progress.push(line),
        }
    }
}

struct ParseError;

fn parse_story(source: &String) /*-> Result<Story, ParseError>*/
{
    let lines = source.lines();
    let pages = compile_lines_to_pages(lines);
}

fn main() {
    let file_arg = std::env::args().nth(1);
    let file_path = match file_arg {
        Some(path) => path,
        None => panic!("path to story file is missing!"),
    };
    let file_content = match read_file(&file_path) {
        Ok(content) => content,
        Err(e) => panic!("{}", e),
    };
    parse_story(&file_content);
    /*let story = match parse_story(&file_content) {
        Ok(story) => story,
        Err(e) => panic!("{}", e),
    };*/
    println!("{}", file_path);
}
