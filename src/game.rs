use serde::Deserialize;
use std::collections::HashMap;

pub type PageId = String;

#[derive(Deserialize, Debug, Clone)]
pub struct Story {
    pub start: PageId,
    pub pages: HashMap<PageId, Page>,
    #[serde(default = "HashMap::new")]
    pub flags: HashMap<String, Flag>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Flag {
    pub id: String,
    pub default: bool,
    pub value: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Page {
    pub content: String,
    #[serde(default = "Vec::new")]
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum ActionType {
    EnableFlag,
    DisableFlag,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Action {
    action_type: ActionType,
}

#[derive(Deserialize, Debug, Clone)]
pub enum ConditionType {
    Flag(bool),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Condition {
    condition_type: ConditionType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Choice {
    pub text: String,
    pub to: PageId,
    #[serde(default = "Vec::new")]
    pub actions: Vec<Action>,
    #[serde(default = "Vec::new")]
    pub conditions: Vec<Condition>,
}

#[derive(Debug)]
pub struct GameState {
    current_page: PageId,
    flags: HashMap<String, Flag>,
}

#[derive(Debug)]
pub enum GameError {
    CurrentPageNotFound(String),
    NextPageNotFound(String),
    PageNotFound(String),
    NoNextPage(String),
    ChoiceNotFound(usize),
}

pub struct Game {
    story: Story,
    state: GameState,
}

impl Game {
    pub fn new(story: &Story) -> Result<Game, GameError> {
        story
            .pages
            .get(&story.start)
            .ok_or(GameError::PageNotFound(
                "given starting page id not found".to_string(),
            ))?;

        Ok(Game {
            story: story.clone(),
            state: GameState {
                current_page: story.start.clone(),
                flags: story.flags.clone(),
            },
        })
    }

    fn get_page(&self, page_id: &PageId) -> Option<&Page> {
        self.story.pages.get(page_id)
    }

    fn get_choices(&self) -> Option<&Vec<Choice>> {
        let choices = &self.get_page(&self.state.current_page)?.choices;
        let choice_count = choices.len();
        match choice_count {
            l if l > 0 => Some(&choices),
            _ => None,
        }
    }

    pub fn make_choice(&mut self, input: usize) -> Result<Page, GameError> {
        let choice = input;
        let next_page_id = match self.get_choices() {
            Some(choices) => {
                let choice = choices
                    .get(choice)
                    .ok_or(GameError::ChoiceNotFound(choice))?;
                Ok(choice.to.clone())
            }
            None => return Err(GameError::ChoiceNotFound(choice)),
        }?;
        match self.get_page(&next_page_id) {
            Some(page) => {
                let page_content = page.clone();
                self.state.current_page = next_page_id.into();
                Ok(page_content)
            }
            None => Err(GameError::NextPageNotFound(next_page_id.to_string())),
        }
    }

    pub fn get_current_page(&self) -> Option<&Page> {
        self.get_page(&self.state.current_page)
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_test_story() -> Story {
        Story {
            start: String::from("page1"),
            flags: HashMap::from([(
                "flag1".to_string(),
                Flag {
                    id: "flag1".to_string(),
                    default: false,
                    value: Option::Some(false),
                },
            )]),
            pages: HashMap::from([
                (
                    "page1".to_string(),
                    Page {
                        content: "page 1".to_string(),
                        choices: vec![
                            Choice {
                                to: "page2".to_string(),
                                text: "to page 2".to_string(),
                                actions: vec![],
                                conditions: vec![],
                            },
                            Choice {
                                to: "page3".to_string(),
                                text: "to page 3".to_string(),
                                actions: vec![],
                                conditions: vec![],
                            },
                        ],
                    },
                ),
                (
                    "page2".to_string(),
                    Page {
                        content: "page 2".to_string(),
                        choices: vec![Choice {
                            to: "page4".to_string(),
                            text: "to page 4".to_string(),
                            actions: vec![],
                            conditions: vec![],
                        }],
                    },
                ),
                (
                    "page3".to_string(),
                    Page {
                        content: "page 3".to_string(),
                        choices: vec![],
                    },
                ),
                (
                    "page4".to_string(),
                    Page {
                        content: "page 4".to_string(),
                        choices: vec![],
                    },
                ),
            ]),
        }
    }

    #[test]
    fn test_story_progression() -> Result<(), GameError> {
        let story = get_test_story();
        let mut game = Game::new(&story)?;

        assert_eq!(game.get_current_page().unwrap().content, "page 1");
        assert_eq!(game.get_current_page().unwrap().choices.len(), 2);

        game.make_choice(0)?;

        assert_eq!(game.get_current_page().unwrap().content, "page 2");
        assert_eq!(game.get_current_page().unwrap().choices.len(), 1);

        game.make_choice(0)?;

        assert_eq!(game.get_current_page().unwrap().content, "page 4");
        assert_eq!(game.get_current_page().unwrap().choices.len(), 0);

        Ok(())
    }
}
