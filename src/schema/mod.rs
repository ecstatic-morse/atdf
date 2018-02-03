mod address;
mod interrupt;
mod peripheral;
mod parse;

pub use self::address::*;
pub use self::interrupt::*;
pub use self::peripheral::*;
use self::parse::*;

use failure::Error;
use minidom::Element;

type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum SchemaError {
    #[fail(display = "Missing attribute: <{} {}=? ... />", _0, _1)]
    MissingAttribute(String, String),

    #[fail(display = "Missing child: <{} ...><{}?><{}/>", _0, _1, _0)]
    MissingChild(String, String),

    #[fail(display = "Missing child: <{} ...><{} name={}><{}/>", _0, _1, _2, _0)]
    MissingNamedChild(String, String, String),
}

pub fn parse(el: &Element) -> Result<Vec<Device>> {
    assert!(el.name() == "avr-tools-device-file");

    let modules = child(el, "modules")?;

    let devices = child(el, "devices")?
        .children()
        .filter(|el| el.name() == "device")
        .map(|el| Device::parse(el, modules))
        .collect::<Result<Vec<_>>>()?;

    Ok(devices)
}

#[derive(Debug)]
pub struct Device {
    pub name: String,
    address_spaces: Vec<AddressSpace>,
    peripherals: Vec<Peripheral>,
    interrupts: Interrupts,
}

impl Device {
    pub fn parse(device: &Element, modules: &Element) -> Result<Self> {
        assert!(device.name() == "device");
        assert!(modules.name() == "modules");

        let name = attr(device, "name")?.to_owned();

        let interrupts = child(device, "interrupts")?;
        let interrupts = Interrupts::parse(interrupts)?;

        let peripherals = child(device, "peripherals")?
            .children()
            .filter(|el| el.name() == "module")
            .map(|el| Peripheral::parse(el, modules))
            .collect::<Result<Vec<_>>>()?;

        let address_spaces = child(device, "address-spaces")?
            .children()
            .filter(|el| el.name() == "address-space")
            .map(AddressSpace::parse)
            .collect::<Result<Vec<_>>>()?;

        Ok(Device {
            name,
            interrupts,
            peripherals,
            address_spaces,
        })
    }
}

/// A range of bytes.
#[derive(Debug)]
pub struct ByteRange {
    pub offset: u32,
    pub len: u32,
}

impl ByteRange {
    pub fn parse(el: &Element, offset: &str) -> Result<Self> {
        let offset = parse_uint(attr(el, offset)?)?;
        let len = parse_uint(attr(el, "size")?)?;

        Ok(ByteRange { offset, len })
    }
}

