use crate::{
    error::JeffResult,
    ops::{JeffFunction, JeffModule, JeffOp, JeffOpKind, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[derive(Clone, Debug, Default)]
pub struct LoweringContext {
    pub value_map: Vec<usize>,
    pub function_map: std::collections::HashMap<String, usize>,
}

pub fn lower_module(module: &str) -> JeffResult<JeffOp> {
    let _ = module;
    Ok(JeffOp::new(JeffOpKind::FuncCall))
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
    let name = match &op.kind {
        JeffOpKind::QubitAlloc => "jeff.qubit_alloc",
        JeffOpKind::FuncCall => "jeff.func_call",
        JeffOpKind::ScfSwitch => "jeff.scf.switch",
        _ => "jeff.unknown",
    };
    Ok(format!("{} ({})", name, op.inputs.len()))
}

pub fn lower_value_index(value: &JeffValue, ctx: &mut LoweringContext) -> JeffResult<usize> {
    let idx = ctx.value_map.len();
    ctx.value_map.push(idx);
    if let Some(name) = value.name.as_ref()
        && !name.is_empty()
    {
        ctx.function_map.entry(name.clone()).or_insert(idx);
    }
    Ok(idx)
}

pub fn lower_op_with_context(op: &JeffOp, ctx: &mut LoweringContext) -> JeffResult<usize> {
    let idx = ctx.value_map.len();
    ctx.value_map.push(idx);
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
    Ok(format!(
        "module {name} with {} functions and {} strings",
        module.functions.len(),
        module.strings.entries.len()
    ))
}

pub fn lower_metadata_summary(op: &JeffOp) -> JeffResult<usize> {
    Ok(op.metadata.len())
}
