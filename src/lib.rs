use serde::{Deserialize, Serialize};
use logicmap::{Brick, Card, Config, Statement};

mod exact;
pub use crate::exact::{exact_match, ExactMatch};

mod subject;
pub use crate::subject::{subject_match, SubjectMatch};

mod group;
pub use crate::group::{group_match, GroupMatch};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Class {
    pub subject: String,
    pub level: usize,
    pub hours: usize,
    pub group: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassExperience {
    pub class: Class,
    pub when: usize,
    pub grade: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub majors: Vec<String>,
    pub classes: Vec<ClassExperience>,
}

// this wraps up alot of the complexity
// this also allows for alot of flexibility
// if you want to block classes from multiple dips
// you cant use this function
pub fn build_degree(
    input: Student,
    config: Vec<Vec<Vec<Box<dyn Config<Student>>>>>,
) -> Vec<Card<Student>> {
    let mut all_cards = Vec::new();
    for (_index, card) in config.into_iter().enumerate() {
        let mut first_card: Card<Student> = Card {
            statements: Vec::new(),
        };
        for (j, statement_config) in card.into_iter().enumerate() {
            let mut stmt = Statement {
                title: String::new(),
                contents: Vec::new(),
            };
            stmt.title = format!("Statement {}", j);
            let mut bricks_for_statement = Vec::new();
            for (_i, brick) in statement_config.into_iter().enumerate() {
                bricks_for_statement.push(Brick {
                    title: format!("{:#?}", brick),
                    input: input.clone(), // this group match can't use any class above...
                    expcheck: brick.expression_function(),
                });
                stmt.contents.append(&mut bricks_for_statement);
            }
            first_card.statements.push(stmt);
        }
        all_cards.push(first_card);
    }
    all_cards
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
