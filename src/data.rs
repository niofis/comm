use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestStruct {
    pub name: String,
    pub line: Vec<u32>,
}
