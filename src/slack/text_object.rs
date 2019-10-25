use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct TextObject<'a> {
    r#type: &'a str,
    text: &'a str,
}

impl<'a> Default for TextObject<'a> {
    fn default() -> TextObject<'a> {
        TextObject {
            r#type: "mrkdwn",
            text: Default::default(),
        }
    }
}

impl<'a> From<&'a str> for TextObject<'a> {
    fn from(v: &'a str) -> TextObject {
        TextObject::new(v)
    }
}

impl<'a> TextObject<'a> {
    pub fn new(text: &'a str) -> TextObject<'a> {
        TextObject {
            text,
            ..Default::default()
        }
    }
}
