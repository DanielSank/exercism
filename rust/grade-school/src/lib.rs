// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
use std::collections::HashSet;

#[allow(clippy::new_without_default)]
pub struct School {
    students: Vec<(u32, String)>,
}

impl School {
    pub fn new() -> School {
        School{students: vec![]}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.push((grade, student.to_string()))
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = HashSet::new();
        for (grade, _student) in &self.students {
            grades.insert(grade.clone());
        }
        let mut grades_vec = vec![];
        for grade in grades {
            grades_vec.push(grade)
        }
        grades_vec.sort();
        grades_vec
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = vec![];
        for (this_grade, student) in &self.students {
            if *this_grade == grade {
                students.push(student.clone())
            }
        }
        students.sort();
        students
    }
}
