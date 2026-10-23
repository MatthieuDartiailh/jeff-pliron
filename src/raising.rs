use crate::{
    error::JeffResult,
    ops::{JeffOp, JeffValue},
    types::{JeffType, FloatPrecision},
};

#[derive(Clone, Debug, Default)]
pub struct RaisingContext {
    pub name_to_index: std::collections::HashMap<String, usize>,
}

pub fn raise_module(_items: &[JeffOp]) -> JeffResult<Vec<JeffOp>> {
    Ok(Vec::new())
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
