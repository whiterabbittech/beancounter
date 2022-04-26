use crate::AllocSize;
use serde::{Serialize, Deserialize};

const VERSION: &'static str = "v1";

#[derive(Serialize, Deserialize)]
pub struct JSONManifest {
    version: &'static str,
    data: JSONData,
}

#[derive(Serialize, Deserialize)]
pub struct JSONData {
    stack_sizes: Vec<StackInfo>,
    heap_sizes: Vec<HeapInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct StackInfo {
    type_name: String,
    // path: the full path to this time. Not always available.
    size: AllocSize,
}

#[derive(Serialize, Deserialize)]
pub struct HeapInfo {
    type_name: String,
}
