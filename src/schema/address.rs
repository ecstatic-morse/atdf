use super::{Access, util};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "address-space")]
pub struct AddressSpace {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    pub endianness: Endianness,

    #[serde(with = "util::int")]
    pub start: u32,

    #[serde(with = "util::int")]
    pub size: u32,

    #[serde(rename = "value-group", default)]
    pub segments: Vec<MemorySegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MemorySegment {
    pub name: String,

    #[serde(with = "util::int")]
    pub offset: u32,

    #[serde(with = "util::int")]
    pub size: u32,

    #[serde(rename = "rw", default)]
    pub access: Access
}
