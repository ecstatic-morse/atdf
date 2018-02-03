use minidom::Element;

use super::{attr, Result};

#[derive(Debug)]
pub struct Interrupts(Vec<Interrupt>);

impl Interrupts {
    pub fn parse(el: &Element) -> Result<Self> {
        let list = el.children()
            .filter(|el| el.name() == "interrupt")
            .map(Interrupt::parse)
            .collect::<Result<Vec<_>>>()?;

        Ok(Interrupts(list))
    }
}

#[derive(Debug)]
pub struct Interrupt {
    name: String,
    description: Option<String>,
    index: u32,
}

impl Interrupt {
    pub fn parse(el: &Element) -> Result<Self> {
         let name = attr(el, "name")?.to_owned();
         let description = attr(el, "caption").ok().map(ToOwned::to_owned);
         let index = attr(el, "index")?.parse()?;

         Ok(Interrupt {
             name,
             description,
             index,
         })
    }
}
