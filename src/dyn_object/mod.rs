pub trait Readable {
    fn read(&self) -> usize;
}

pub struct A<'a> {
    name: &'a str,
}

impl Readable for A<'_> {
    fn read(&self) -> usize {
        println!("name {}", self.name);
        self.name.len()
    }
}

pub struct B<'a> {
    slice: &'a [i32],
}

impl Readable for B<'_> {
    fn read(&self) -> usize {
        println!("len {}", self.slice.len());
        self.slice.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arg = "-";
        let readable: &dyn Readable = if arg == "-" {
            &A { name: "hello" }
        } else {
            &B { slice: &[1, 2, 3] }
        };
        assert_eq!(readable.read(), 5);
    }
}
