use rootcause::{Report, bail};
use std::fmt::Debug;
use std::hash::Hash;
use uuid::{Uuid, Version};

pub trait AppId: Copy + Clone + Debug + Eq + Hash + Ord + PartialEq + PartialOrd {
    fn from_id(uuid: Id) -> Self;

    fn as_id(&self) -> &Id;

    fn new() -> Self {
        Self::from_id(Id::new())
    }

    fn parse(s: &str) -> Result<Self, Report> {
        Ok(Self::from_id(Id::parse(s)?))
    }

    fn to_hyphenated(&self) -> String {
        self.as_id().to_hyphenated()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Id {
    uuid: Uuid,
}

impl Id {
    pub fn new() -> Self {
        Id {
            uuid: Uuid::now_v7(),
        }
    }

    pub fn parse(string: &str) -> Result<Self, Report> {
        let uuid = Uuid::try_parse(string)?;

        if uuid.get_version() != Some(Version::SortRand) {
            bail!("ID UUID must be version 7");
        }

        Ok(Id { uuid })
    }

    pub fn to_hyphenated(&self) -> String {
        self.uuid.as_hyphenated().to_string()
    }
}
