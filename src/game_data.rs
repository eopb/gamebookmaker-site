use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Project {
    name: String,
    chapters: Vec<Chapter>,
}

impl Project {
    fn example() -> Self {
        Self {
            name: "Example project".to_string(),
            chapters: vec![
                Chapter {
                    key: 1,
                    text: "Start of story".to_string(),
                    choices: vec![
                        Choice {
                            text: "continue".to_string(),
                            event: GoesTo(2),
                        },
                        Choice {
                            text: "End".to_string(),
                            event: GameEnd,
                        },
                    ],
                },
                Chapter {
                    key: 2,
                    text: "End".to_string(),
                    choices: vec![],
                },
            ],
        }
    }
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            chapters: Vec::new(),
        }
    }
    pub fn json_example() -> String {
        serde_json::to_string(&Self::example()).unwrap()
    }
}

#[derive(Deserialize, Debug, Serialize)]
struct Chapter {
    key: u32,
    text: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Choice {
    text: String,
    event: ChoiceEvent,
}

#[derive(Deserialize, Debug, Serialize)]
enum ChoiceEvent {
    GameEnd,
    GoesTo(u32),
}
use ChoiceEvent::{GameEnd, GoesTo};
