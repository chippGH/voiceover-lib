use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub color: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Replica {
    pub person: usize,
    pub text: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceOver {
    pub name: String,
    pub persons: Vec<Person>,
    pub replicas: Vec<Replica>
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.color)
    }
}

impl Person {
    pub fn new(name: String, color: String) -> Person {
        Person {
            name,
            color
        }
    }
}

impl Display for VoiceOver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for replica in self.replicas.iter() {
            match writeln!(f, "{}: {}", self.persons[replica.person], replica.text) {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        Ok(())
    }
}

impl Replica {
    pub fn new(person: usize, text: String) -> Replica {
        Replica {
            person,
            text
        }
    }
}

impl VoiceOver {
    pub fn new(name: String, persons: Vec<Person>, replicas: Vec<Replica>) -> VoiceOver {
        VoiceOver {
            name,
            persons,
            replicas
        }
    }
}