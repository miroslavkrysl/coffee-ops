use crate::domain::id::{AppId, Id};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coffee {
    id: CoffeeId,
    brand: CoffeeBrand,
    name: CoffeeName,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoffeeId {
    id: Id,
}

impl AppId for CoffeeId {
    fn from_id(id: Id) -> Self {
        CoffeeId { id }
    }

    fn as_id(&self) -> &Id {
        &self.id
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoffeeBrand {
    string: String,
}

impl CoffeeBrand {
    pub fn new(string: String) -> Self {
        todo!()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoffeeName {
    string: String,
}

impl CoffeeName {
    pub fn new(string: String) -> Self {
        todo!()
    }
}
