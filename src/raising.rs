use std::collections::HashMap;

use crate::{
    error::JeffResult,
    ops::{JeffFunction, JeffModule, JeffOp, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[derive(Clone, Debug, Default)]
pub struct RaisingContext {
    pub name_to_index: HashMap<String, usize>,
    pub value_to_index: HashMap<String, usize>,
    pub op_to_index: HashMap<String, usize>,
    pub function_to_index: HashMap<String, usize>,
}

pub fn serialize_jeff_module(module: &JeffModule) -> JeffResult<JeffModule> {
    raise_module_semantic(module)
}

pub fn deserialize_jeff_module(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn raise_module(items: &[JeffOp]) -> JeffResult<Vec<JeffOp>> {
    Ok(items.to_vec())
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
        _ => Err(crate::error::JeffPlironError::DeserializationFailed(format!(
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
    let mut raised = module.clone();
    let mut ctx = RaisingContext::default();

    for (function_idx, function) in raised.functions.iter_mut().enumerate() {
        if function.name.is_none() {
            function.name = Some(format!("func{function_idx}"));
        }
        ctx.function_to_index.insert(function.name.clone().unwrap(), function_idx);

        for (region_idx, region) in function.regions.iter_mut().enumerate() {
            if region.name.is_none() {
                region.name = Some(format!("region{region_idx}"));
            }
            ctx.name_to_index.insert(region.name.clone().unwrap(), region_idx);

            for (op_idx, op) in region.ops.iter_mut().enumerate() {
                if op.name.is_none() {
                    op.name = Some(format!("op{op_idx}"));
                }
                ctx.op_to_index.insert(op.name.clone().unwrap(), op_idx);

                for (value_idx, value) in op.inputs.iter_mut().chain(op.outputs.iter_mut()).enumerate() {
                    if value.name.is_none() {
                        value.name = Some(format!("v{value_idx}"));
                    }
                    ctx.value_to_index.insert(value.name.clone().unwrap(), value_idx);
                    value.index = Some(value_idx);
                }
            }
        }
    }

    Ok(raised)
}

pub fn raise_string_table(entries: &[String]) -> JeffResult<Vec<String>> {
    Ok(entries.to_vec())
}
