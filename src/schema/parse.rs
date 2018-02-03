//! Utility functions for parsing XML documents.

use minidom::Element;

use super::{Result, SchemaError};

pub fn attr<'a>(el: &'a Element, s: &str) -> Result<&'a str> {
    el.attr(s)
        .ok_or_else(|| SchemaError::MissingAttribute(el.name().to_owned(), s.to_owned()))
        .map_err(Into::into)
}

pub fn child<'a>(el: &'a Element, s: &str) -> Result<&'a Element> {
    el.children()
        .find(|el| el.name() == s)
        .ok_or_else(|| SchemaError::MissingChild(el.name().to_owned(), s.to_owned()))
        .map_err(Into::into)
}

pub fn named_child<'a>(el: &'a Element, s: &str, name: &str) -> Result<&'a Element> {
    el.children()
        .filter(|el| el.name() == s)
        .find(|el| el.attr("name").map_or(false, |n| n == name))
        .ok_or_else(|| SchemaError::MissingNamedChild(el.name().to_owned(),
                                                      s.to_owned(),
                                                      name.to_owned()))
        .map_err(Into::into)
}

pub fn attr_bool<'a>(el: &'a Element, s: &str) -> Result<bool> {
    let s = match el.attr(s) {
        Some("") => return Ok(true),
        Some(s) => s,
        None => return Ok(false),
    };

    let b = s.parse()
        .or_else(|e| {
            match s.parse() {
                Ok(0_u32) => Ok(false),
                Ok(1) => Ok(true),
                _ => Err(e)
            }
        })?;

    Ok(b)
}

pub fn parse_uint(s: &str) -> Result<u32> {
    if s == "0" {
        return Ok(0);
    }

    fn matches(s: &str, base: u32, p: &[&str]) -> Option<(u32, usize)> {
        p.iter()
            .find(|&p| s.starts_with(p))
            .map(|p| (base, p.len()))
    }

    let (base, skip) = matches(s, 16, &["0x", "0X"])
        .or_else(|| matches(s, 8, &["0", "0o", "0O"]))
        .or_else(|| matches(s, 2, &["0b", "0B"]))
        .unwrap_or((10, 0));

    let (_, s) = s.split_at(skip);
    let ret = u32::from_str_radix(s, base)?;
    Ok(ret)
}
