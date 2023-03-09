#[derive(Debug, PartialEq)]
pub struct Student {
    name: String,
    age: u32,
    school: String,
}

impl Student {
    pub fn new(name: String, age: u32, school: String) -> Self {
        Self { name, age, school }
    }

    pub fn builder() -> StudentBuilder {
        StudentBuilder::default()
    }
}

#[derive(Debug, PartialEq)]
pub struct StudentBuilder {
    name: String,
    age: u32,
    school: String,
}

impl Default for StudentBuilder {
    fn default() -> Self {
        Self {
            name: Default::default(),
            age: Default::default(),
            school: "csuft".into(),
        }
    }
}

impl StudentBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = age;
        self
    }

    pub fn school(mut self, school: String) -> Self {
        self.school = school;
        self
    }

    pub fn build(self) -> Student {
        let Self { name, age, school } = self;

        Student::new(name, age, school)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // s -> student
        let s1 = Student::new("xiaoming".into(), 12, "csuft".into());

        let s2 = Student::builder().name("xiaoming".into()).age(12).build();

        assert_eq!(s1, s2);
    }
}
