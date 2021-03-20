use logicmap::{Config, ExpResult};

use crate::Student;

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

#[derive(Debug)]
pub struct SubjectMatch {
    pub subject: String,
}

impl Config<Student> for SubjectMatch {
    fn expression_function(&self) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
        subject_match(self.subject.clone())
    }
    fn name(&self) -> String {
        // format!("{} {}", "Subject Match -", self.subject)
        format!("{:?}", self)
    }
}
