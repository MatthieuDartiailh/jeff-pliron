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

#[deprecated(note = "use DeserializationContext instead")]
pub type RaisingContext = DeserializationContext;

pub fn serialize_jeff_module(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn deserialize_jeff_module(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn raise_module(items: &[JeffOp]) -> JeffResult<Vec<JeffOp>> {
    Ok(items.to_vec())
}

pub fn deserialize_module(items: &[JeffOp]) -> JeffResult<Vec<JeffOp>> {
    raise_module(items)
}

pub fn raise_value(value: &JeffValue) -> JeffResult<JeffValue> {
    Ok(value.clone())
}

pub fn deserialize_value(value: &JeffValue) -> JeffResult<JeffValue> {
    raise_value(value)
}

pub fn raise_op(op: &JeffOp) -> JeffResult<JeffOp> {
    Ok(op.clone())
}

pub fn deserialize_op(op: &JeffOp) -> JeffResult<JeffOp> {
    raise_op(op)
}

pub fn raise_type(ty: &JeffType) -> JeffResult<JeffType> {
    Ok(ty.clone())
}

pub fn deserialize_type(ty: &JeffType) -> JeffResult<JeffType> {
    raise_type(ty)
}

pub fn raise_float_precision(bits: u8) -> JeffResult<FloatPrecision> {
    match bits {
        32 => Ok(FloatPrecision::Float32),
        64 => Ok(FloatPrecision::Float64),
        _ => Err(crate::error::JeffPlironError::DeserializationFailed(format!(
            "unsupported float precision bits: {bits}"
        ))),
    }
}

pub fn raise_function(function: &JeffFunction) -> JeffResult<JeffFunction> {
    Ok(function.clone())
}

pub fn deserialize_function(function: &JeffFunction) -> JeffResult<JeffFunction> {
    raise_function(function)
}

pub fn raise_region(region: &JeffRegion) -> JeffResult<JeffRegion> {
    Ok(region.clone())
}

pub fn deserialize_region(region: &JeffRegion) -> JeffResult<JeffRegion> {
    raise_region(region)
}

pub fn raise_module_semantic(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn deserialize_module_semantic(module: &JeffModule) -> JeffResult<JeffModule> {
    raise_module_semantic(module)
}

pub fn raise_string_table(entries: &[String]) -> JeffResult<Vec<String>> {
    Ok(entries.to_vec())
}
