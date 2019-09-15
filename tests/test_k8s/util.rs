
use std::fmt::{self, Display, Write};

use std::marker::PhantomData;
use std::str::FromStr;

pub trait Delimiting {
    const DELIMITER: char;
}

pub trait Allowed {}

macro_rules! impl_delim {
    ($ty:ident => $delim:expr) => {
        #[derive(Debug, Clone)]
        pub struct $ty;

        impl Delimiting for $ty {
            const DELIMITER: char = $delim;
        }
    };
}

impl_delim!(Csv => ',');
impl Allowed for Csv {}

impl_delim!(Ssv => ' ');
impl Allowed for Ssv {}

impl_delim!(Tsv => '\t');
impl Allowed for Tsv {}

impl_delim!(Pipes => '|');
impl Allowed for Pipes {}

// NOTE: We use ampersand only for convenience. Multiple instances are allowed only
// in form data and query, and we need something for parsing stuff from CLI. But, we
// also cannot allow serializing this container in the same way as others.
// That's why `Multi` doesn't implement `Allowed`.
impl_delim!(Multi => '&');

#[derive(Debug, Clone)]
pub struct Delimited<T, D>(Vec<T>, PhantomData<D>);

impl<T, D> From<Vec<T>> for Delimited<T, D> {
    fn from(v: Vec<T>) -> Self {
        Delimited(v, PhantomData)
    }
}

impl<T: FromStr, D: Delimiting> FromStr for Delimited<T, D> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Result<Vec<_>, _> = s.split(D::DELIMITER).map(|s| s.parse::<T>()).collect();
        Ok(Delimited(vec?, PhantomData))
    }
}

impl<T: Display, D: Delimiting + Allowed> Display for Delimited<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_char(D::DELIMITER)?;
            }

            v.fmt(f)?;
        }

        Ok(())
    }
}
