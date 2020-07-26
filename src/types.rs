use crate::sql_types::*;
use diesel::backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::{Eq, PartialEq};
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::ops::Deref;
use std::str::FromStr;

#[cfg(feature = "with-actix-web")]
use actix_web::dev::FromParam;

/// `CiString` is a CaseInsensitive String type that can be used as the key for
/// a hashmap as well as be written to the page. It implements a variety of traits
/// to make it easy to convert from and to &str and String types.
#[derive(Clone, Debug, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[serde(transparent)]
#[sql_type = "Citext"]
pub struct CiString {
    value: String,
}

impl CiString {
    pub fn new() -> Self {
        CiString {
            value: String::new(),
        }
    }
}

/// CiString can implement the FromParam trait from ActixWeb if "with-actix-web" is
/// turned on in the Cargo.toml file.
#[cfg(feature = "with-actix-web")]
impl FromParam for CiString {
    type Err = actix_web::error::UrlParseError;
    fn from_param(s: &str) -> Result<Self, Self::Err> {
        Ok(CiString {
            value: s.to_owned(),
        })
    }
}

impl fmt::Display for CiString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for CiString {
    fn eq(&self, other: &CiString) -> bool {
        self.value.to_lowercase() == other.value.to_lowercase()
    }
}

impl PartialEq<String> for CiString {
    fn eq(&self, other: &String) -> bool {
        self.value.to_lowercase() == other.to_lowercase()
    }
}

impl PartialEq<&str> for CiString {
    fn eq(&self, other: &&str) -> bool {
        self.value.to_lowercase() == other.to_lowercase()
    }
}

impl Eq for CiString {}

impl Hash for CiString {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.value.hash(hasher);
    }
}

impl AsRef<str> for CiString {
    fn as_ref(&self) -> &str {
        &*self.value
    }
}

impl Borrow<str> for CiString {
    fn borrow(&self) -> &str {
        &*self.value
    }
}

impl Deref for CiString {
    type Target = String;

    fn deref(&self) -> &String {
        &self.value
    }
}

impl FromStr for CiString {
    type Err = fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CiString {
            value: s.to_owned(),
        })
    }
}

impl Into<String> for CiString {
    fn into(self) -> String {
        self.value
    }
}

impl From<String> for CiString {
    fn from(value: String) -> Self {
        CiString { value }
    }
}

impl From<&str> for CiString {
    fn from(value: &str) -> Self {
        CiString {
            value: value.to_owned(),
        }
    }
}

impl FromSql<Citext, Pg> for CiString {
    fn from_sql(bytes: Option<backend::RawValue<Pg>>) -> deserialize::Result<Self> {
        use std::str;
        let some_bytes = not_none!(bytes);
        let string = str::from_utf8(some_bytes.as_bytes())?;
        Ok(CiString {
            value: string.to_owned(),
        })
    }
}

impl ToSql<Citext, Pg> for CiString {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.value.as_bytes())
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
    }
}

impl FromSql<Citext, Pg> for String {
    fn from_sql(bytes: Option<backend::RawValue<Pg>>) -> deserialize::Result<Self> {
        use std::str;
        let some_bytes = not_none!(bytes);
        let string = str::from_utf8(some_bytes.as_bytes())?;
        Ok(string.to_owned())
    }
}

impl ToSql<Citext, Pg> for String {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.as_bytes())
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
    }
}

impl ToSql<Citext, Pg> for str {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.as_bytes())
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
    }
}
