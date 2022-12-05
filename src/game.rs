use serde::Deserialize;
use std::collections::HashMap;

pub type PageId = String;
pub type FlagId = String;

#[derive(Deserialize, Debug, Clone)]
pub struct Story {
    pub start: PageId,
    pub pages: HashMap<PageId, Page>,
    #[serde(default = "HashMap::new")]
    pub flags: HashMap<FlagId, FlagDefinition>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FlagDefinition {
    pub id: FlagId,
    pub default: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FlagState {
    pub id: FlagId,
    pub value: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Page {
    pub content: String,
    #[serde(default = "Vec::new")]
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum ActionType {
    EnableFlag(FlagId),
    DisableFlag(FlagId),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Action {
    action_type: ActionType,
}

#[derive(Deserialize, Debug, Clone)]
pub enum ConditionType {
    Flag(String, bool),
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
    flags: HashMap<String, FlagState>,
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
                flags: HashMap::from_iter(story.flags.iter().map(|flag| -> (String, FlagState) {
                    (
                        flag.0.to_string(),
                        FlagState {
                            id: flag.0.to_string(),
                            value: flag.1.default,
                        },
                    )
                })),
            },
        })
    }

    fn get_page(&self, page_id: &PageId) -> Option<&Page> {
        self.story.pages.get(page_id)
    }

    fn get_flag(&self, flag_name: &String) -> Option<&FlagState> {
        self.state.flags.get(flag_name)
    }

    fn is_choice_visible(&self, choice: &Choice) -> bool {
        for condition in &choice.conditions {
            match &condition.condition_type {
                ConditionType::Flag(flag_name, value) => {
                    let flag = self.get_flag(&flag_name);
                    match flag {
                        Some(f) => {
                            if f.value != *value {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
            }
        }
        true
    }

    fn get_choices(&self) -> Vec<&Choice> {
        let choices = match &self.get_page(&self.state.current_page) {
            Some(p) => &p.choices,
            None => return vec![],
        };
        let valid_choices: Vec<&Choice> = choices
            .iter()
            .filter(|choice| self.is_choice_visible(choice))
            .collect();
        valid_choices
    }

    pub fn make_choice(&mut self, input: usize) -> Result<Page, GameError> {
        let choice = input;
        let choices = self.get_choices();
        let next_page_id = match choices.len() {
            l if l > 0 => {
                let choice = choices
                    .get(choice)
                    .ok_or(GameError::ChoiceNotFound(choice))?;
                Ok(choice.to.clone())
            }
            _ => return Err(GameError::ChoiceNotFound(choice)),
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

    pub fn get_current_page(&self) -> Option<Page> {
        let source_page = self.get_page(&self.state.current_page)?;
        let visible_choices = self.get_choices();
        Some(Page {
            content: source_page.content.clone(),
            choices: visible_choices.into_iter().cloned().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_story() -> Story {
        Story {
            start: String::from("page1"),
            flags: HashMap::from([(
                "flag1".to_string(),
                FlagDefinition {
                    id: "flag1".to_string(),
                    default: false,
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
                                actions: vec![Action {
                                    action_type: ActionType::EnableFlag("flag1".to_string()),
                                }],
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
                        choices: vec![Choice {
                            to: "page4".to_string(),
                            text: "to page 4".to_string(),
                            actions: vec![],
                            conditions: vec![],
                        }],
                    },
                ),
                (
                    "page4".to_string(),
                    Page {
                        content: "page 4".to_string(),
                        choices: vec![Choice {
                            to: "page5".to_string(),
                            text: "to page 5".to_string(),
                            actions: vec![],
                            conditions: vec![Condition {
                                condition_type: ConditionType::Flag("flag1".to_string(), true),
                            }],
                        }],
                    },
                ),
                (
                    "page5".to_string(),
                    Page {
                        content: "page 5".to_string(),
                        choices: vec![],
                    },
                ),
            ]),
        }
    }

    #[test]
    fn test_basic_story_progression() -> Result<(), GameError> {
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

    #[test]
    fn test_story_progression_with_flag() -> Result<(), GameError> {
        let story = get_test_story();
        let mut game = Game::new(&story)?;

        assert_eq!(game.get_current_page().unwrap().content, "page 1");
        assert_eq!(game.get_current_page().unwrap().choices.len(), 2);

        game.make_choice(1)?;

        assert_eq!(game.get_current_page().unwrap().content, "page 3");
        assert_eq!(game.get_current_page().unwrap().choices.len(), 1);

        game.make_choice(0)?;

        assert_eq!(game.get_current_page().unwrap().content, "page 4");
        assert_eq!(game.get_current_page().unwrap().choices.len(), 0);

        Ok(())
    }
}
