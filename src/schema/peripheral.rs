use std::collections::BTreeSet;

use minidom::Element;

use super::{attr, child, named_child, parse_uint, ByteRange, Result, SchemaError};

#[derive(Debug)]
pub struct Peripheral {
    name: String,
    description: Option<String>,
    instances: Vec<Instance>,
}

impl Peripheral {
    pub fn parse(el: &Element, modules: &Element) -> Result<Self> {
        let name = attr(el, "name")?.to_owned();
        let description = attr(el, "caption").ok().map(ToOwned::to_owned);
        let module = named_child(modules, "module", &name)?;
        let instances = el.children()
            .filter(|el| el.name() == "instance")
            .map(|el| Instance::parse(el, module))
            .filter(|instance| {
                // Instances without register groups are not errors
                instance .as_ref()
                    .err()
                    .map_or(false, |err| {
                        if let Some(&SchemaError::MissingChild(ref outer, ref inner)) = err.downcast_ref() {
                            outer != "instance" || inner != "register-group"
                        } else {
                            true
                        }
                    })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Peripheral {
            name,
            description,
            instances,
        })
    }
}

#[derive(Debug)]
pub struct Instance {
    name: String,
    description: Option<String>,
    registers: Vec<Register>,
}

impl Instance {
    pub fn parse(el: &Element, module: &Element) -> Result<Self> {
        let name = attr(el, "name")?.to_owned();
        let description = attr(el, "caption").ok().map(ToOwned::to_owned);

        // TODO: account for multiple register-groups
        let mut registers = child(el, "register-group")?;

        // If register-group is empty, look up the corresponding element in the module.
        if registers.children().count() == 0 {
            let reg_name = attr(registers, "name-in-module")
                .or_else(|_| attr(registers, "name"))?;

            registers = named_child(module, "register-group", &reg_name)?;
        }

        let registers = registers.children()
            .filter(|el| el.name() == "register")
            .map(|el| Register::parse(el, module))
            .collect::<Result<Vec<_>>>()?;

        Ok(Instance {
            name,
            description,
            registers,
        })
    }
}

#[derive(Debug)]
pub struct Register {
    pub name: String,
    pub description: Option<String>,
    range: ByteRange,
    initial: u32,
    bitfields: Vec<Bitfield>,
}

impl Register {
    pub fn parse(el: &Element, module: &Element) -> Result<Self> {
        let name = attr(el, "name")?.to_owned();
        let description = attr(el, "caption").ok().map(ToOwned::to_owned);
        let range = ByteRange::parse(el, "offset")?;
        let initial = el.attr("initval").map_or(Ok(0), |i| parse_uint(i))?;
        let bitfields = el.children()
            .filter(|el| el.name() == "bitfield")
            .map(|el| Bitfield::parse(el, module))
            .collect::<Result<Vec<_>>>()?;

        Ok(Register {
            name,
            description,
            range,
            initial,
            bitfields
        })
    }
}

#[derive(Debug)]
pub struct Bitfield {
    pub name: String,
    pub description: Option<String>,
    pub mask: u32,
    pub values: Option<Vec<EnumerationValue>>,
}

impl Bitfield {
    pub fn parse(el: &Element, module: &Element) -> Result<Self> {
        let name = attr(el, "name")?.to_owned();
        let description = attr(el, "caption").ok().map(ToOwned::to_owned);
        let mask = parse_uint(attr(el, "mask")?)?;
        let values: Option<Result<_>> = attr(el, "values").ok()
            .map(|s| {
                let values = named_child(module, "value-group", s)?;
                let values = values.children()
                    .filter(|el| el.name() == "value")
                    .map(EnumerationValue::parse)
                    .collect::<Result<Vec<_>>>()?;

                Ok(values)
            });

        let values = match values {
            None => None,
            Some(res) => Some(res?),
        };

        Ok(Bitfield {
            name,
            description,
            mask,
            values,
        })
    }
}

#[derive(Debug)]
pub struct EnumerationValue {
    pub name: String,
    pub description: Option<String>,
    pub value: u32,
}

impl EnumerationValue {
    pub fn parse(el: &Element) -> Result<Self> {
        let name = attr(el, "name")?.to_owned();
        let description = attr(el, "caption").ok().map(ToOwned::to_owned);
        let value = parse_uint(attr(el, "value")?)?;

        Ok(EnumerationValue {
            name,
            description,
            value,
        })
    }
}
