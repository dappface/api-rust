use super::block::Block;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Payload<'a> {
    channel: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocks: Option<Vec<Block<'a>>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // thread_ts: Option<&'a str>, // [TODO]
    // mrkdwn: Option<bool>, // [TODO]
}

impl<'a> Default for Payload<'a> {
    fn default() -> Payload<'a> {
        Payload {
            channel: "general",
            text: Default::default(),
            blocks: Default::default(),
            // thread_ts: Default::default(), // [TODO]
            // mrkdwn: Default::default(),    // [TODO]
        }
    }
}

#[derive(Debug)]
pub struct PayloadBuilder<'a> {
    inner: Result<Payload<'a>, &'a str>,
}

impl<'a> Default for PayloadBuilder<'a> {
    fn default() -> PayloadBuilder<'a> {
        PayloadBuilder {
            inner: Ok(Default::default()),
        }
    }
}

impl<'a> PayloadBuilder<'a> {
    pub fn new() -> PayloadBuilder<'a> {
        Default::default()
    }

    pub fn channel(self, val: &'a str) -> PayloadBuilder<'a> {
        match self.inner {
            Ok(mut inner) => {
                inner.channel = val;
                PayloadBuilder { inner: Ok(inner) }
            }
            Err(_) => self,
        }
    }

    pub fn text(self, val: &'a str) -> PayloadBuilder<'a> {
        match self.inner {
            Ok(mut inner) => {
                inner.text = Some(val);
                PayloadBuilder { inner: Ok(inner) }
            }
            Err(_) => self,
        }
    }

    pub fn blocks(self, vals: Vec<Block<'a>>) -> PayloadBuilder<'a> {
        match self.inner {
            Ok(mut inner) => {
                inner.blocks = Some(vals);
                PayloadBuilder { inner: Ok(inner) }
            }
            Err(_) => self,
        }
    }

    pub fn build(self) -> Result<Payload<'a>, &'a str> {
        self.inner
    }
}
