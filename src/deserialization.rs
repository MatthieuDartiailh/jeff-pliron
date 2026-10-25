use std::collections::HashMap;

use crate::{
    error::JeffResult,
    ops::{JeffFunction, JeffModule, JeffOp, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[derive(Clone, Debug, Default)]
pub struct DeserializationContext {
    pub name_to_index: HashMap<String, usize>,
    pub value_to_index: HashMap<String, usize>,
    pub op_to_index: HashMap<String, usize>,
    pub function_to_index: HashMap<String, usize>,
}

pub fn serialize_jeff_module(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn deserialize_jeff_module(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn deserialize_module(items: &[JeffOp]) -> JeffResult<Vec<JeffOp>> {
    Ok(items.to_vec())
}

pub fn deserialize_value(value: &JeffValue) -> JeffResult<JeffValue> {
    Ok(value.clone())
}

pub fn deserialize_op(op: &JeffOp) -> JeffResult<JeffOp> {
    Ok(op.clone())
}

pub fn deserialize_type(ty: &JeffType) -> JeffResult<JeffType> {
    Ok(ty.clone())
}

pub fn deserialize_float_precision(bits: u8) -> JeffResult<FloatPrecision> {
    match bits {
        32 => Ok(FloatPrecision::Float32),
        64 => Ok(FloatPrecision::Float64),
        _ => Err(crate::error::JeffPlironError::DeserializationFailed(format!(
            "unsupported float precision bits: {bits}"
        ))),
    }
}

pub fn deserialize_function(function: &JeffFunction) -> JeffResult<JeffFunction> {
    Ok(function.clone())
}

pub fn deserialize_region(region: &JeffRegion) -> JeffResult<JeffRegion> {
    Ok(region.clone())
}

pub fn deserialize_module_semantic(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn deserialize_string_table(entries: &[String]) -> JeffResult<Vec<String>> {
    Ok(entries.to_vec())
}
