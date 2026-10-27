use pliron::{
    context::Context,
    dialect::{Dialect, DialectName},
};

pub const JEFF_DIALECT_NAME: &str = "jeff";

pub fn register_jeff_dialect(ctx: &mut Context) -> &mut Dialect {
    let dialect_name = DialectName::try_new(JEFF_DIALECT_NAME).expect("valid JEFF dialect name");
    Dialect::register(ctx, &dialect_name)
}

pub fn jeff_dialect_name() -> DialectName {
    DialectName::try_new(JEFF_DIALECT_NAME).expect("valid JEFF dialect name")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialect_name_is_jeff() {
        assert_eq!(JEFF_DIALECT_NAME, "jeff");
        assert_eq!(jeff_dialect_name().to_string(), "jeff");
    }
}
