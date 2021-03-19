use serde::{Deserialize, Serialize};

use logicmap::{Brick, Card, ExpResult, Statement};

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

// // check upper levels
// pub fn upper_level_min_hours(input: Student) -> ExpResult {
//     let mut indexes = Vec::new();
//     let mut upper_hours = 0;
//     for (index, cls) in input.classes.into_iter().enumerate() {
//         if cls.class.level > 200 {
//             indexes.push(index);
//             upper_hours += cls.class.hours;
//         }
//     }
//     let mut resp = ExpResult {
//         descr: String::from("Min hours - upper level"),
//         path: vec![String::from("classes")],
//         index: indexes,
//         met_flag: false,
//     };
//     if upper_hours > 40 {
//         resp.met_flag = true
//     }
//     resp
// }

pub fn exact_match(mysub: String, nbr: usize) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
    Box::new(move |input: Student| {
        let mut indexes: Vec<usize> = Vec::new();
        let mut did_complete = false;
        for (index, cls) in input.classes.into_iter().enumerate() {
            if cls.class.subject == mysub.clone() {
                if cls.class.level == nbr.clone() {
                    indexes.push(index);
                    did_complete = true;
                }
            }
        }
        ExpResult {
            descr: format!("Exact - {} {}", mysub, nbr),
            path: vec![String::from("classes")],
            index: indexes,
            met_flag: did_complete,
        }
    })
}

pub fn subject_match(mysub: String) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
    Box::new(move |input: Student| {
        let mut indexes: Vec<usize> = Vec::new();
        let mut did_complete = false;
        for (index, cls) in input.classes.into_iter().enumerate() {
            if cls.class.subject == mysub.clone() {
                indexes.push(index);
                did_complete = true;
            }
        }
        ExpResult {
            descr: format!("Subject - {}", mysub),
            path: vec![String::from("classes")],
            index: indexes,
            met_flag: did_complete,
        }
    })
}

pub fn group_match(group: String) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
    Box::new(move |input: Student| {
        let mut indexes: Vec<usize> = Vec::new();
        let mut did_complete = false;
        for (index, cls) in input.classes.into_iter().enumerate() {
            if cls.class.group.contains(&group) {
                indexes.push(index);
                did_complete = true;
            }
        }
        ExpResult {
            descr: format!("Group - {}", group),
            path: vec![String::from("classes")],
            index: indexes,
            met_flag: did_complete,
        }
    })
}

// CONFIG AND MATCHING

pub trait Config {
    fn expression_function(&self) -> Box<dyn Fn(Student) -> ExpResult + 'static>;
    fn name(&self) -> String;
}

#[derive(Debug)]
pub struct ExactMatch {
    pub subject: String,
    pub level: usize,
}

#[derive(Debug)]
pub struct SubjectMatch {
    pub subject: String,
}

#[derive(Debug)]
pub struct GroupMatch {
    pub group: String,
}

impl Config for ExactMatch {
    fn expression_function(&self) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
        exact_match(self.subject.clone(), self.level.clone())
    }
    fn name(&self) -> String {
        // format!("{} {} {}", "Exact Match -", self.subject, self.level)
        format!("{:?}", self)
    }
}

impl Config for SubjectMatch {
    fn expression_function(&self) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
        subject_match(self.subject.clone())
    }
    fn name(&self) -> String {
        // format!("{} {}", "Subject Match -", self.subject)
        format!("{:?}", self)
    }
}

impl Config for GroupMatch {
    fn expression_function(&self) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
        group_match(self.group.clone())
    }
    fn name(&self) -> String {
        // format!("{} {}", "Group Match -", self.group)
        format!("{:?}", self)
    }
}

impl std::fmt::Debug for dyn Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// this wraps up alot of the complexity
// this also allows for alot of flexibility
// if you want to block classes from multiple dips
// you cant use this function
pub fn build_degree(input: Student, config: Vec<Vec<Vec<Box<dyn Config>>>>) -> Vec<Card<Student>> {
    let mut all_cards = Vec::new();
    for (_index, card) in config.into_iter().enumerate() {
        let mut first_card: Card<Student> = Card {
            statements: Vec::new(),
        };
        // in_one_of_groups_and_meet_min_hours(vec![String::from("HU"),String::from("L")], 20);
        // println!("{}", _index);

        for (j, statement_config) in card.into_iter().enumerate() {
            // println!("\t{}", j);

            let mut stmt = Statement {
                title: String::new(),
                contents: Vec::new(),
            };

            stmt.title = format!("Statement {}", j);
            let mut bricks_for_statement = Vec::new();
            for (_i, brick) in statement_config.into_iter().enumerate() {
                // add a second input that handles removal of specific classes
                // or pinning of specific to a brick

                // let early_res = exact_match_english_100(input.clone());
                // println!("{:?}", early_res);
                // println!("{:?}", c);

                // println!("\t\t{}", _i);

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
