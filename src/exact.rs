use logicmap::{Config, ExpResult};

use crate::Student;

pub fn exact_match(mysub: String, nbr: usize) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
    Box::new(move |input: Student| {
        let mut indexes: Vec<usize> = Vec::new();
        let mut did_complete = false;
        for (index, cls) in input.classes.into_iter().enumerate() {
            if cls.class.subject == mysub.clone() { // check if subject matches
                if cls.class.level == nbr.clone() { // check if number matches
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

#[derive(Debug)]
pub struct ExactMatch {
    pub subject: String,
    pub level: usize,
}

impl Config<Student> for ExactMatch {
    fn expression_function(&self) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
        exact_match(self.subject.clone(), self.level.clone())
    }
    fn name(&self) -> String {
        // format!("{} {} {}", "Exact Match -", self.subject, self.level)
        format!("{:?}", self)
    }
}
