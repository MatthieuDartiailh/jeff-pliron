use std::collections::HashMap;

use crate::{
    attributes::StringTable,
    error::JeffResult,
    ops::{JeffFunction, JeffModule, JeffOp, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[derive(Clone, Debug, Default)]
pub struct LoweringContext {
    pub value_map: HashMap<String, usize>,
    pub value_order: Vec<String>,
    pub function_map: HashMap<String, usize>,
    pub op_map: HashMap<String, usize>,
    pub region_map: HashMap<String, usize>,
}

#[derive(Clone, Debug, Default)]
pub struct SemanticLowering {
    pub module: JeffModule,
    pub value_map: HashMap<String, usize>,
    pub op_map: HashMap<String, usize>,
    pub function_map: HashMap<String, usize>,
    pub region_map: HashMap<String, usize>,
}

pub fn lower_module(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn lower_type(ty: &JeffType) -> JeffResult<JeffType> {
    Ok(ty.clone())
}

pub fn lower_value(value: &JeffValue) -> JeffResult<JeffValue> {
    Ok(value.clone())
}

pub fn lower_op(op: &JeffOp) -> JeffResult<JeffOp> {
    Ok(op.clone())
}

pub fn lower_value_index(value: &JeffValue, ctx: &mut LoweringContext) -> JeffResult<usize> {
    let key = value
        .name
        .clone()
        .unwrap_or_else(|| format!("v{}", ctx.value_order.len()));
    let idx = match ctx.value_map.get(&key) {
        Some(&idx) => idx,
        None => {
            let idx = ctx.value_order.len();
            ctx.value_order.push(key.clone());
            ctx.value_map.insert(key, idx);
            idx
        }
    };
    Ok(idx)
}

pub fn lower_op_with_context(op: &JeffOp, ctx: &mut LoweringContext) -> JeffResult<usize> {
    let key = op
        .name
        .clone()
        .unwrap_or_else(|| format!("op{}", ctx.op_map.len()));
    let idx = match ctx.op_map.get(&key) {
        Some(&idx) => idx,
        None => {
            let idx = ctx.op_map.len();
            ctx.op_map.insert(key, idx);
            idx
        }
    };
    Ok(idx)
}

pub fn lower_float_precision(precision: FloatPrecision) -> JeffResult<u8> {
    Ok(precision.bits())
}

pub fn lower_function(function: &JeffFunction) -> JeffResult<JeffFunction> {
    Ok(function.clone())
}

pub fn lower_region(region: &JeffRegion) -> JeffResult<JeffRegion> {
    Ok(region.clone())
}

pub fn lower_module_semantic(module: &JeffModule) -> JeffResult<SemanticLowering> {
    let mut value_map = HashMap::new();
    let mut op_map = HashMap::new();
    let mut function_map = HashMap::new();
    let mut region_map = HashMap::new();

    for (function_idx, function) in module.functions.iter().enumerate() {
        if let Some(ref name) = function.name {
            function_map.insert(name.clone(), function_idx);
        }

        for (region_idx, region) in function.regions.iter().enumerate() {
            if let Some(ref name) = region.name {
                region_map.insert(name.clone(), region_idx);
            }

            for (op_idx, op) in region.ops.iter().enumerate() {
                if let Some(ref name) = op.name {
                    op_map.insert(name.clone(), op_idx);
                }
            }
        }

        for (value_idx, value) in function
            .arguments
            .iter()
            .chain(function.results.iter())
            .enumerate()
        {
            if let Some(ref name) = value.name {
                value_map.insert(name.clone(), value_idx);
            }
        }
    }

    Ok(SemanticLowering {
        module: module.clone(),
        value_map,
        op_map,
        function_map,
        region_map,
    })
}

pub fn lower_metadata_summary(op: &JeffOp) -> JeffResult<usize> {
    Ok(op.metadata.len())
}

pub fn lower_string_table(strings: &StringTable) -> JeffResult<StringTable> {
    Ok(strings.clone())
}
