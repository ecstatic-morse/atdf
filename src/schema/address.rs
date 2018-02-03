use std::str::FromStr;

use minidom::Element;

use super::{attr, attr_bool, Error, Result, ByteRange};

/// A set of contiguous addresses.
#[derive(Debug)]
pub struct AddressSpace {
    pub name: String,
    pub endianness: Endianness,
    pub range: ByteRange,
    pub segments: Vec<Segment>,
}

impl AddressSpace {
    pub fn parse(el: &Element) -> Result<Self> {
        let name = attr(el, "name")?.to_owned();
        let range = ByteRange::parse(el, "start")?;
        let endianness = Endianness::parse(el)?;

        let segments = el.children()
            .filter(|el| el.name() == "memory-segment")
            .map(Segment::parse)
            .collect::<Result<Vec<_>>>()?;

        Ok(AddressSpace {
            name,
            endianness,
            range,
            segments,
        })
    }
}

/// Read-write permissions for a given memory segment.
#[derive(Debug)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl Access {
    pub fn parse(el: &Element) -> Result<Self> {
        use self::Access::*;

        let rw = attr(el, "rw")?;
        let rw = match rw.to_lowercase().as_ref() {
            "r" => ReadOnly,
            "w" => WriteOnly,
            "rw" => ReadWrite,
            _ => bail!("Invalid access specifier \"{}\"", rw),
        };

        Ok(rw)
    }
}

/// The endianness of an address space.
#[derive(Debug)]
pub enum Endianness {
    Big,
    Little,
}

impl FromStr for Endianness {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let e = match s.to_lowercase().as_ref() {
            "big" => Endianness::Big,
            "little" => Endianness::Little,
            _ => bail!("Unknown endianness: {}", s),
        };

        Ok(e)
    }
}

impl Endianness {
    pub fn parse(el: &Element) -> Result<Self> {
        let end = attr(el, "endianness")?.parse()?;
        Ok(end)
    }
}

/// The purpose of a given segment of memory.
#[derive(Debug)]
pub enum SegmentKind {
    Flash,
    Signatures,
    Fuses,
    Lockbits,
    Io,
    Ram,
    SysReg,
    Other(String),
}

impl SegmentKind {
    pub fn parse(el: &Element) -> Result<Self> {
        use self::SegmentKind::*;

        let kind = attr(el, "type")?;
        let kind = match kind.to_lowercase().as_ref() {
            "flash" => Flash,
            "signatures" => Signatures,
            "lockbits" => Lockbits,
            "fuses" => Fuses,
            "io" => Io,
            "ram" => Ram,
            "sysreg" => SysReg,
            _ => Other(kind.to_owned())
        };

        Ok(kind)
    }
}

#[derive(Debug)]
pub struct Segment {
    pub name: String,
    pub range: ByteRange,
    pub kind: SegmentKind,
    pub access: Access,
    pub exec: bool,
}

impl Segment {
    pub fn parse(el: &Element) -> Result<Self> {
        let name = attr(el, "name")?.to_owned();
        let range = ByteRange::parse(el, "start")?;
        let kind = SegmentKind::parse(el)?;
        let access = Access::parse(el).unwrap_or(Access::ReadOnly);
        let exec = attr_bool(el, "exec")?;

        Ok(Segment {
            name,
            range,
            kind,
            access,
            exec,
        })
    }
}

