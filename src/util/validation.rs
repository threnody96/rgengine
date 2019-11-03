pub trait Validation {

    type Output;

    fn validate(self) -> Self::Output;

}

