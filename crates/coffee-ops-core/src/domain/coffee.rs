use crate::domain::common::{contains_control_chars, normalize_nfc};
use derive_more::AsRef;
use rootcause::prelude::*;
use uuid::{Uuid, Version};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coffee {
    id: CoffeeId,
    brand: CoffeeBrand,
    name: CoffeeName,
}

impl Coffee {
    pub fn new(id: CoffeeId, brand: CoffeeBrand, name: CoffeeName) -> Self {
        Self { id, brand, name }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NewCoffee {
    brand: CoffeeBrand,
    name: CoffeeName,
}

impl NewCoffee {
    pub fn new(brand: CoffeeBrand, name: CoffeeName) -> Self {
        Self { brand, name }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoffeeId {
    uuid: Uuid,
}

impl CoffeeId {
    pub fn generate() -> Self {
        Self {
            uuid: Uuid::now_v7(),
        }
    }

    fn from_uuid(uuid: Uuid) -> Result<Self, Report> {
        if uuid.get_version() != Some(Version::SortRand) {
            bail!("ID UUID must be version 7");
        }

        Ok(Self { uuid })
    }
}

#[derive(AsRef, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoffeeBrand {
    #[as_ref(str)]
    string: String,
}

impl CoffeeBrand {
    pub const MAX_LENGTH: usize = 64;

    pub fn new(mut string: String) -> Result<Self, Report> {
        string = string.trim().into();
        string = normalize_nfc(&string).into();

        if string.is_empty() {
            bail!("coffee brand must not be empty");
        }

        if string.chars().count() > Self::MAX_LENGTH {
            bail!(
                "coffee brand must be at most {} characters",
                Self::MAX_LENGTH
            );
        }

        if contains_control_chars(&string) {
            bail!("coffee brand must not contain control characters");
        }

        Ok(Self { string })
    }
}

#[derive(AsRef, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoffeeName {
    #[as_ref(str)]
    string: String,
}

impl CoffeeName {
    pub const MAX_LENGTH: usize = 64;

    pub fn new(mut string: String) -> Result<Self, Report> {
        string = string.trim().into();
        string = normalize_nfc(&string).into();

        if string.is_empty() {
            bail!("coffee name must not be empty");
        }

        if string.chars().count() > Self::MAX_LENGTH {
            bail!(
                "coffee name must be at most {} characters",
                Self::MAX_LENGTH
            );
        }

        if contains_control_chars(&string) {
            bail!("coffee name must not contain control characters");
        }

        Ok(Self { string })
    }
}
