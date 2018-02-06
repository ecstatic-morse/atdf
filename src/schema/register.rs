use super::{util};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub group: String,

    #[serde(with = "util::opt_int", default)]
    pub index: Option<u32>,

    pub pad: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterGroup {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    #[serde(rename = "register", default)]
    pub registers: Vec<Register>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    #[serde(with = "util::int")]
    pub offset: u32,

    #[serde(with = "util::int")]
    pub size: u32,

    #[serde(rename = "ocd-rw", default)]
    pub access: Option<Access>,

    #[serde(rename = "initval", with = "util::opt_int", default)]
    pub initial: Option<u32>,

    #[serde(rename = "bitfield", default)]
    pub bitfields: Vec<Bitfield>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Bitfield {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    #[serde(with = "util::int")]
    pub mask: u32,

    pub values: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ValueGroup {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    #[serde(rename = "value", default)]
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Value {
    pub name: String,

    #[serde(rename = "caption", default)]
    pub description: String,

    #[serde(with = "util::int")]
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Access {
    /// Registers for which reads and writes have different semantics.
    /// (e.g. USART Data Register)
    #[serde(rename = "")]
    Bidirectional,

    #[serde(rename = "R")]
    ReadOnly,

    #[serde(rename = "W")]
    WriteOnly,

    #[serde(rename = "RW")]
    ReadWrite,
}

impl Default for Access {
    fn default() -> Self {
        Access::ReadWrite
    }
}

