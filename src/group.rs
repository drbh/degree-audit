use logicmap::{Config, ExpResult};

use crate::Student;

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

#[derive(Debug)]
pub struct GroupMatch {
    pub group: String,
}

impl Config<Student> for GroupMatch {
    fn expression_function(&self) -> Box<dyn Fn(Student) -> ExpResult + 'static> {
        group_match(self.group.clone())
    }
    fn name(&self) -> String {
        // format!("{} {}", "Group Match -", self.group)
        format!("{:?}", self)
    }
}
