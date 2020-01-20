use dumb_gen::Dumb;

#[derive(Dumb, Debug)]
pub struct A {
    a: String,
    b: u8,
    c: u16,
}

impl A {

    pub fn new() -> Self {
        Self {
            a: "abc".to_string(),
            b: 0,
            c: 1,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{A, DumbA};

    #[test]
    fn it_works() {
        let a = A::new();
        let DumbA {
            a, b, c
        } = a.dumb();
        assert_eq!(a, "abc".to_string());
        assert_eq!(b, 0);
        assert_eq!(c, 1);
    }
}
