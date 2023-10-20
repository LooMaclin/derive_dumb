use derive_dumb::Dumb;

#[derive(Dumb, Debug)]
pub struct A<'a, T> {
    a: String,
    b: u8,
    c: &'a T,
}

impl<'a, T> A<'a, T> {
    pub fn new(c: &'a T) -> Self {
        Self {
            a: "abc".to_string(),
            b: 0,
            c,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DumbA, A};

    #[test]
    fn it_works() {
        let a = A::new(&1);
        let DumbA { a, b, c } = a.dumb();
        assert_eq!(a, "abc".to_string());
        assert_eq!(b, 0);
        assert_eq!(c, &1);
    }
}
