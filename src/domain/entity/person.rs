use serde::{ser::SerializeStruct, Serialize, Serializer};

use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub document_type: String,
    pub document_number: String
}


impl Person {
    pub fn new(name: &str, document_type: &str, document_number: &str) -> Person {
        let uuid = Uuid::new_v4().to_string();
        Person {
            id: String::from(uuid),
            name: String::from(name),
            document_type: String::from(document_type),
            document_number: String::from(document_number)
        }
    }
}

impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Person", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("document_type", &self.document_type)?;
        state.serialize_field("document_number", &self.document_number)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::entity::person::Person;

    #[test]
    fn test_new() {
        let person: Person = Person::new("John Smith", "1", "0123456789");
        assert_eq!(person.name, "John Smith");
    }
}