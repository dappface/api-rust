use super::text_object::TextObject;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Section<'a> {
    r#type: &'a str,
    text: TextObject<'a>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // block_id: Option<TextObject<'a>>, // [TODO]
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<TextObject<'a>>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // accessory: Option<ElementObject<'a>>, // [TODO]
}

impl<'a> Default for Section<'a> {
    fn default() -> Section<'a> {
        Section {
            r#type: "section",
            text: Default::default(),
            // block_id: Default::default(), // [TODO]
            fields: Default::default(),
            // accessory: Default::default(), // [TODO]
        }
    }
}

impl<'a> From<Section<'a>> for Block<'a> {
    fn from(v: Section<'a>) -> Block<'a> {
        Block::Section(v)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Block<'a> {
    // Actions(Actions), // [TODO]
    // Context(Context), // [TODO]
    // Divider(Divider), // [TODO]
    // File(File), // [TODO]
    // Image(Image), // [TODO]
    // Input(Input), // [TODO]
    Section(Section<'a>),
}

#[derive(Debug, Serialize)]
pub struct SectionBuilder<'a> {
    inner: Result<Section<'a>, &'a str>,
}

impl<'a> Default for SectionBuilder<'a> {
    fn default() -> SectionBuilder<'a> {
        SectionBuilder {
            inner: Ok(Default::default()),
        }
    }
}

impl<'a> SectionBuilder<'a> {
    pub fn new<S: Into<TextObject<'a>>>(text: S) -> SectionBuilder<'a> {
        SectionBuilder {
            inner: Ok(Section {
                text: text.into(),
                ..Default::default()
            }),
        }
    }

    pub fn fields(self, vals: Vec<&'a str>) -> SectionBuilder<'a> {
        match self.inner {
            Ok(mut inner) => {
                inner.fields = Some(vals.into_iter().map(|val| TextObject::new(val)).collect());
                SectionBuilder { inner: Ok(inner) }
            }
            Err(_) => self,
        }
    }

    pub fn build(self) -> Result<Section<'a>, &'a str> {
        self.inner
    }
}
