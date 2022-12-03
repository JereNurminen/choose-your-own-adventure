mod validation {
    use crate::core::Story;

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
        return Ok(());
    }

    /** Validates that the story is playable from start to finish,
    returns a Result with the unchanged story if OK and an error otherwise */
    pub fn validate_story(story: &Story) -> Result<(), String> {
        validate_paths(story)
    }
}

use crate::core::Story;
use toml;

pub fn parse_story(source: &String) -> Result<Story, String> {
    match toml::from_str(source) {
        Ok(story) => {
            validation::validate_story(&story).clone()?;
            return Ok(story);
        }
        Err(e) => Err(e.to_string()),
    }
}
