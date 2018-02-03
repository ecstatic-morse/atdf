#[macro_use] extern crate failure;
extern crate minidom;

pub mod schema;

pub use self::schema::parse;
