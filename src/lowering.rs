use crate::{
    attributes::StringTable,
    error::JeffResult,
    ops::{JeffFunction, JeffModule, JeffOp, JeffOpKind, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[derive(Clone, Debug, Default)]
pub struct LoweringContext {
    pub value_map: std::collections::HashMap<String, usize>,
    pub value_order: Vec<String>,
    pub function_map: std::collections::HashMap<String, usize>,
    pub op_map: std::collections::HashMap<String, usize>,
}

pub fn lower_module(module: &str) -> JeffResult<JeffOp> {
    Ok(JeffOp::new(JeffOpKind::FuncCall)
        .with_name(module)
        .with_metadata("module", module))
}

pub fn lower_type(ty: &JeffType) -> JeffResult<String> {
    Ok(ty.to_string())
}

pub fn lower_value(value: &JeffValue) -> JeffResult<String> {
    let ty = lower_type(&value.ty)?;
    Ok(format!(
        "{}: {}",
        value.name.clone().unwrap_or_else(|| "anon".to_string()),
        ty
    ))
}

pub fn lower_op(op: &JeffOp) -> JeffResult<String> {
    Ok(format!(
        "{}: inputs={}, outputs={}, metadata={}",
        op.semantic_name(),
        op.inputs.len(),
        op.outputs.len(),
        op.metadata.len()
    ))
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
    let _ = op;
    Ok(idx)
}

pub fn lower_float_precision(precision: FloatPrecision) -> JeffResult<u8> {
    Ok(precision.bits())
}

pub fn lower_function(function: &JeffFunction) -> JeffResult<String> {
    let name = function.name.as_deref().unwrap_or("anon_fn");
    let args = function
        .arguments
        .iter()
        .map(|v| v.ty.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let results = function
        .results
        .iter()
        .map(|v| v.ty.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    Ok(format!(
        "function {name}({args}) -> ({results}) with {} region(s)",
        function.regions.len()
    ))
}

pub fn lower_region(region: &JeffRegion) -> JeffResult<String> {
    Ok(format!(
        "region {} with {} ops",
        region.name.as_deref().unwrap_or("anon_region"),
        region.ops.len()
    ))
}

pub fn lower_module_semantic(module: &JeffModule) -> JeffResult<String> {
    let name = module.name.as_deref().unwrap_or("anon_module");
    let functions = module
        .functions
        .iter()
        .map(|f| f.name.as_deref().unwrap_or("anon_fn"))
        .collect::<Vec<_>>()
        .join(", ");
    Ok(format!(
        "module {name} with {} functions [{}] and {} strings",
        module.functions.len(),
        functions,
        module.strings.entries.len()
    ))
}

pub fn lower_metadata_summary(op: &JeffOp) -> JeffResult<usize> {
    Ok(op.metadata.len())
}

pub fn lower_string_table(strings: &StringTable) -> JeffResult<Vec<String>> {
    Ok(strings.entries.clone())
}
