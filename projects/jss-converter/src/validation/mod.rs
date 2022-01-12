#[derive(Debug)]
pub enum Validation<T, E> {
    Success { result: T, errors: Vec<E> },
    Failure { fatal: E, errors: Vec<E> },
}
impl<T, E> Validation<T, E> {
    #[inline]
    pub fn success(result: T, errors: Vec<E>) -> Validation<T, E> {
        Self::Success { result, errors }
    }
    #[inline]
    pub fn failure(fatal: E, errors: Vec<E>) -> Validation<T, E> {
        Self::Failure { fatal, errors }
    }
}

impl<T, E> Validation<T, E> {
    #[inline]
    pub fn errors(&self) -> &Vec<E> {
        match self {
            Self::Success { errors, .. } => errors,
            Self::Failure { errors, .. } => errors,
        }
    }
    #[inline]
    pub fn result(self) -> Result<T, E> {
        match self {
            Self::Success { result, .. } => Ok(result),
            Self::Failure { fatal, .. } => Err(fatal),
        }
    }
    #[inline]
    pub fn option(self) -> Option<T> {
        match self {
            Self::Success { result, .. } => Some(result),
            Self::Failure { .. } => None,
        }
    }
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::Success { result, .. } => result,
            Self::Failure { .. } => {
                panic!("called `Validation::unwrap()` on an `Failure` value")
            }
        }
    }
}
