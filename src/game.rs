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
    pub fn new(story: &Story) -> Result<Game, String> {
        story
            .pages
            .get(&story.start)
            .ok_or("given starting page id not found")?;

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

    pub fn make_choice(&mut self, input: &usize) -> Result<Page, GameError> {
        let choice = *input;
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
