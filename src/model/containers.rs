/*!
Provides some basic container enums that are used by the Policy model.
*/

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A container used by a number of elements where the JSON serialization may be  a single
/// string,  or an array of string values.
///
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum OneOrAll<T> {
    /// A single statement.
    One(T),
    /// A vector of statements.
    All(Vec<T>),
}

///
/// A container used by a number of elements where the JSON serialization may be a wild-card
/// value, a single string,  or an array of string values.
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(untagged)]
pub enum OneOrAny<T: Clone = String>
where
    T: Clone,
{
    /// The wildcard value, may be Any or All depending on use.
    #[serde(rename = "*")]
    Any,
    /// One element of type `T`
    One(T),
    /// A JSON array with elements of type `T`.
    AnyOf(Vec<T>),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T> OneOrAny<T>
where
    T: Clone,
{
    ///
    /// Returns `true` if the option is an `Any` value.
    ///
    pub fn is_any(&self) -> bool {
        match self {
            OneOrAny::Any => true,
            _ => false,
        }
    }

    ///
    /// Returns `true` if the option is an `One` value.
    ///
    pub fn is_one(&self) -> bool {
        match self {
            OneOrAny::One(_) => true,
            _ => false,
        }
    }

    ///
    /// Returns `true` if the option is an `AnyOf` value.
    ///
    pub fn is_any_of(&self) -> bool {
        match self {
            OneOrAny::AnyOf(_) => true,
            _ => false,
        }
    }

    ///
    /// Converts from OneOrAny<T> to Option<T>.
    ///
    /// Converts `self` into an `Option<T>`, consuming `self`, and discarding either `Any` or
    /// `AnyOf` values.
    ///
    pub fn one(self) -> Option<T> {
        match self {
            OneOrAny::One(value) => Some(value),
            _ => None,
        }
    }

    ///
    /// Converts from OneOrAny<T> to Option<T>.
    ///
    /// Converts `self` into an `Option<T>`, consuming `self`, and discarding either `Any` or
    /// `One` values.
    ///
    pub fn any_of(self) -> Option<Vec<T>> {
        match self {
            OneOrAny::AnyOf(values) => Some(values.clone()),
            _ => None,
        }
    }
}
