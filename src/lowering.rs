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
    let value_map = module
        .functions
        .iter()
        .flat_map(|fun| fun.arguments.iter().chain(fun.results.iter()))
        .filter_map(|value| {
            value
                .name
                .clone()
                .map(|name| (name, value.index.unwrap_or_default()))
        })
        .collect();

    let op_map = module
        .functions
        .iter()
        .flat_map(|fun| fun.regions.iter().flat_map(|region| region.ops.iter()))
        .filter_map(|op| {
            op.name
                .clone()
                .map(|name| (name, op.inputs.len() + op.outputs.len()))
        })
        .collect();

    let function_map = module
        .functions
        .iter()
        .filter_map(|fun| fun.name.clone().map(|name| (name, fun.regions.len())))
        .collect();

    let region_map = module
        .functions
        .iter()
        .flat_map(|fun| fun.regions.iter())
        .filter_map(|region| region.name.clone().map(|name| (name, region.ops.len())))
        .collect();

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
