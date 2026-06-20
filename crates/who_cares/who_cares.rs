use core::convert::Infallible;
use core::fmt::Debug;
use core::ops::FromResidual;
use std::process;

/// A unified test return type
/// where `?` panics for `Result::Err` and `Option::None`.
#[derive(Debug)]
pub struct WhoCares<T>(pub T);

impl<T> FromResidual<Option<Infallible>> for WhoCares<T> {
    #[inline]
    #[track_caller]
    fn from_residual(_: Option<Infallible>) -> Self {
        panic!("called WhoCares `?` on a `None` value")
    }
}

impl<T, E> FromResidual<Result<Infallible, E>> for WhoCares<T>
where
    E: Debug,
{
    #[inline]
    #[track_caller]
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Result::Ok(value) => match value {},
            Result::Err(error) => panic!("called WhoCares `?` on an `Err` value: {error:?}"),
        }
    }
}

impl<T> process::Termination for WhoCares<T>
where
    T: process::Termination,
{
    #[inline]
    fn report(self) -> process::ExitCode {
        self.0.report()
    }
}
