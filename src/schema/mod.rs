//! The schema used in atdf files.
//!
//! TODO: Pluralized types (e.g. `Devices`) are needed because it's difficult to interpret a single
//! XML tag as a newtype. Related to [https://github.com/RReverser/serde-xml-rs/issues/38].

mod address;
mod register;
mod util;

use std::io;

use serde_xml_rs as xml;

pub use self::address::*;
pub use self::register::*;

pub fn parse<R>(r: R) -> Result<Atdf, xml::Error>
    where R: io::Read,
{
    let atdf: Result<Atdf, _> = xml::deserialize(r);
    atdf
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atdf {
    devices: Devices,
    modules: Modules,
}

impl Atdf {
    pub fn devices(&self) -> &[Device] {
        &self.devices.devices
    }

    pub fn modules(&self) -> &[Module] {
        &self.modules.modules
    }
}

/// All devices present in an atdf file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Devices {
    #[serde(rename = "device", default)]
    devices: Vec<Device>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Modules {
    #[serde(rename = "module", default)]
    modules: Vec<Module>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Device {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    pub architecture: String,
    pub family: String,

    #[serde(default)]
    address_spaces: AddressSpaces,

    #[serde(default)]
    peripherals: Peripherals,

    #[serde(default)]
    interrupts: Interrupts,
}

impl Device {
    pub fn address_spaces(&self) -> &[AddressSpace] {
        &self.address_spaces.address_spaces
    }

    pub fn peripherals(&self) -> &[PeripheralFamily] {
        &self.peripherals.peripherals
    }

    pub fn interrupts(&self) -> &[Interrupt] {
        &self.interrupts.interrupts
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AddressSpaces {
    #[serde(rename = "address-space", default)]
    address_spaces: Vec<AddressSpace>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Interrupts {
    #[serde(rename = "interrupt", default)]
    interrupts: Vec<Interrupt>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Peripherals {
    #[serde(rename = "module", default)]
    peripherals: Vec<PeripheralFamily>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Interrupt {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    #[serde(with = "util::int")]
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeripheralFamily {
    pub name: String,

    #[serde(rename = "instance", default)]
    pub instances: Vec<Peripheral>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peripheral {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    #[serde(rename = "register-group")]
    pub registers: Option<RegisterGroupLink>,

    #[serde(default)]
    signals: Signals,
}

impl Peripheral {
    pub fn signals(&self) -> &[Signal] {
        &self.signals.signals
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Signals {
    #[serde(rename = "signal")]
    signals: Vec<Signal>,
}

/// The name of a register-group in the modules section.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegisterGroupLink {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    #[serde(default)]
    pub name_in_module: String,

    /// The name of the address_space in which this register group lives.
    pub address_space: String,

    /// The offset of this register group within the address space.
    #[serde(with = "util::int")]
    pub offset: u32,
}

/// A list of register groups and enumerated values used for peripherals of the given type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    #[serde(rename = "register-group", default)]
    pub register_groups: Vec<RegisterGroup>,

    /// All value-groups must follow register-groups due to
    /// https://github.com/RReverser/serde-xml-rs/issues/5
    #[serde(rename = "value-group", default)]
    pub value_groups: Vec<ValueGroup>,
}
