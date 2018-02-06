#[macro_use] extern crate failure;
extern crate minidom;
extern crate serde;
extern crate serde_xml_rs;
#[macro_use] extern crate serde_derive;
extern crate svd_parser;

pub mod svd;
pub mod schema;

pub use self::schema::parse;
pub use self::svd::device;
