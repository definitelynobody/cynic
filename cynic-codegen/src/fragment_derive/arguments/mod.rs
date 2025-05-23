pub(super) mod analyse;
pub(super) mod output;
mod parsing;

use proc_macro2::Span;

use crate::{
    error::Errors,
    schema::{Schema, Unvalidated},
};

pub use self::{
    output::Output,
    parsing::{arguments_from_field_attrs, CynicArguments, FieldArgument, FieldArgumentValue},
};

pub(super) use self::parsing::ArgumentLiteral;

pub fn process_arguments<'a>(
    schema: &'a Schema<'a, Unvalidated>,
    literals: Vec<parsing::FieldArgument>,
    field: &crate::schema::types::Field<'a>,
    schema_module: syn::Path,
    variables_fields: Option<&syn::Path>,
    span: Span,
) -> Result<Output<'a>, Errors> {
    let analysed =
        analyse::analyse_field_arguments(schema, literals, field, variables_fields, span)?;

    Ok(Output {
        analysed,
        schema_module,
    })
}

#[cfg(test)]
mod tests;
