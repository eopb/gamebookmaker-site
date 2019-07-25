use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
struct Project {
    name: String,
    chapters: Vec<Chapter>,
}

impl Project {
    fn example() -> Self {
        Project {
            name: "Example project".to_string(),
            chapters: vec![Chapter {
                key: 1,
                text: "Start of story".to_string(),
                choices: vec![],
            }],
        }
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
