use crate::{
    error::{JeffPlironError, JeffResult},
    ops::{JeffOp, JeffOpKind, JeffValue},
    types::{JeffType, FloatPrecision},
};

#[derive(Clone, Debug, Default)]
pub struct LoweringContext {
    pub value_map: Vec<usize>,
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
    Ok(format!("{}: {}", value.name.clone().unwrap_or_else(|| "anon".to_string()), ty))
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
    let _ = value;
    Ok(idx)
}

pub fn lower_op_with_context(op: &JeffOp, ctx: &mut LoweringContext) -> JeffResult<usize> {
    let _ = ctx;
    Err(JeffPlironError::UnsupportedOp(format!(
        "not implemented for op kind: {:?}",
        op.kind
    )))
}

pub fn lower_float_precision(precision: FloatPrecision) -> JeffResult<u8> {
    Ok(precision.bits())
}
