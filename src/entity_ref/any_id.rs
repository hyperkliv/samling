use crate::{
    Attribute, AttributeType, Category, Collection, Color, Group, Id, Image, Organization, Price,
    PriceList, Size, Style, User,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum AnyId {
    User(Id<User>),
    Organization(Id<Organization>),
    AttributeType(Id<AttributeType>),
    Attribute(Id<Attribute>),
    Group(Id<Group>),
    Category(Id<Category>),
    Collection(Id<Collection>),
    Color(Id<Color>),
    Style(Id<Style>),
    Image(Id<Image>),
    PriceList(Id<PriceList>),
    Price(Id<Price>),
    Size(Id<Size>),
}

impl From<Id<User>> for AnyId {
    fn from(id: Id<User>) -> Self {
        AnyId::User(id)
    }
}

impl From<Id<Organization>> for AnyId {
    fn from(id: Id<Organization>) -> Self {
        AnyId::Organization(id)
    }
}

impl From<Id<AttributeType>> for AnyId {
    fn from(id: Id<AttributeType>) -> Self {
        AnyId::AttributeType(id)
    }
}

impl From<Id<Attribute>> for AnyId {
    fn from(id: Id<Attribute>) -> Self {
        AnyId::Attribute(id)
    }
}

impl From<Id<Group>> for AnyId {
    fn from(id: Id<Group>) -> Self {
        AnyId::Group(id)
    }
}

impl From<Id<Category>> for AnyId {
    fn from(id: Id<Category>) -> Self {
        AnyId::Category(id)
    }
}

impl From<Id<Collection>> for AnyId {
    fn from(id: Id<Collection>) -> Self {
        AnyId::Collection(id)
    }
}

impl From<Id<Color>> for AnyId {
    fn from(id: Id<Color>) -> Self {
        AnyId::Color(id)
    }
}

impl From<Id<Style>> for AnyId {
    fn from(id: Id<Style>) -> Self {
        AnyId::Style(id)
    }
}

impl From<Id<Image>> for AnyId {
    fn from(id: Id<Image>) -> Self {
        AnyId::Image(id)
    }
}

impl From<Id<PriceList>> for AnyId {
    fn from(id: Id<PriceList>) -> Self {
        AnyId::PriceList(id)
    }
}

impl From<Id<Price>> for AnyId {
    fn from(id: Id<Price>) -> Self {
        AnyId::Price(id)
    }
}

impl From<Id<Size>> for AnyId {
    fn from(id: Id<Size>) -> Self {
        AnyId::Size(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ensure_types_with_same_id_number_are_distinct() {
        debug_assert_ne!(
            AnyId::PriceList(Id::<PriceList>::new(2)),
            AnyId::Style(Id::<Style>::new(2))
        )
    }

    #[test]
    fn ensure_same_type_with_different_ids_are_distinct() {
        debug_assert_ne!(
            AnyId::PriceList(Id::<PriceList>::new(2)),
            AnyId::PriceList(Id::<PriceList>::new(4))
        )
    }
}
