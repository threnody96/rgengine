pub trait FuzzyArg<T> {

    fn take(&self) -> T;

}

impl FuzzyArg<String> for String {

    fn take(&self) -> String {
        self.clone()
    }

}

impl FuzzyArg<String> for &str {

    fn take(&self) -> String {
        self.to_string()
    }

}

impl FuzzyArg<String> for &String {

    fn take(&self) -> String {
        self.to_string()
    }

}

impl FuzzyArg<u16> for u16 {

    fn take(&self) -> u16 {
        *self
    }

}

impl <T, E> FuzzyArg<Option<E>> for Option<T> where T: FuzzyArg<E> {

    fn take(&self) -> Option<E> {
        if self.is_none() { return None; }
        Some(self.as_ref().unwrap().take())
    }

}

#[derive(Clone, Copy)]
pub struct NoOption {}
