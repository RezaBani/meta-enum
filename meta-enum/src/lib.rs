use core::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result},
    marker::PhantomData,
    str::FromStr,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ParseMetaEnumError<T> {
    marker: PhantomData<T>,
}

impl<T> ParseMetaEnumError<T> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<T> Display for ParseMetaEnumError<T>
where
    T: MetaEnum,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "provided string was not found for this MetaEnum, valid keys are {:?}",
            T::keys()
        )
    }
}

impl<T> Error for ParseMetaEnumError<T> where T: Debug + MetaEnum {}

pub trait MetaEnum: From<i32> + From<u8> + Into<i32> + Into<u8> + FromStr {
    fn count() -> usize;
    fn keys() -> Vec<String>;
    fn values() -> Vec<i32>;
    fn pairs() -> Vec<(String, i32)> {
        let keys = Self::keys();
        let values = Self::values();
        let mut pairs = Vec::new();
        for (key, value) in keys.into_iter().zip(values) {
            pairs.push((key, value));
        }
        pairs
    }
}

#[cfg(feature = "derive")]
pub use meta_enum_derive::MetaEnum;
