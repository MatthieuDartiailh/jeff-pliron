use crate::{
    error::JeffResult,
    ops::{JeffFunction, JeffModule, JeffOp, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[derive(Clone, Debug, Default)]
pub struct RaisingContext {
    pub name_to_index: std::collections::HashMap<String, usize>,
    pub value_to_index: std::collections::HashMap<String, usize>,
    pub op_to_index: std::collections::HashMap<String, usize>,
}

pub fn raise_module(_items: &[JeffOp]) -> JeffResult<Vec<JeffOp>> {
    Ok(_items.to_vec())
}

pub fn raise_value(value: &JeffValue) -> JeffResult<JeffValue> {
    Ok(value.clone())
}

pub fn raise_op(op: &JeffOp) -> JeffResult<JeffOp> {
    Ok(op.clone())
}

pub fn raise_type(ty: &JeffType) -> JeffResult<JeffType> {
    Ok(ty.clone())
}

pub fn raise_float_precision(bits: u8) -> JeffResult<FloatPrecision> {
    match bits {
        32 => Ok(FloatPrecision::Float32),
        64 => Ok(FloatPrecision::Float64),
        _ => Err(crate::error::JeffPlironError::RaisingFailed(format!(
            "unsupported float precision bits: {bits}"
        ))),
    }
}

pub fn raise_function(function: &JeffFunction) -> JeffResult<JeffFunction> {
    Ok(function.clone())
}

pub fn raise_region(region: &JeffRegion) -> JeffResult<JeffRegion> {
    Ok(region.clone())
}

pub fn raise_module_semantic(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn raise_string_table(entries: &[String]) -> JeffResult<Vec<String>> {
    Ok(entries.to_vec())
}
