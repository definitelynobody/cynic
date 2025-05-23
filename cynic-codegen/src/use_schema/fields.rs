use super::argument::ArgumentOutput;

use {
    quote::{quote, ToTokens, TokenStreamExt},
    syn::parse_quote,
};

use crate::schema::types::Field;

pub struct FieldOutput<'a> {
    pub(super) field: &'a Field<'a>,
    pub(super) parent_marker: &'a proc_macro2::Ident,
}

impl ToTokens for FieldOutput<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let parent_marker = self.parent_marker;
        let field_marker = &self.field.marker_ident().to_rust_ident();
        let field_name_literal = proc_macro2::Literal::string(self.field.name.as_str());

        let field_type_marker = self
            .field
            .field_type
            .marker_type()
            .to_path(&parse_quote! { super::super });

        tokens.append_all(quote! {
            pub struct #field_marker;

            impl cynic::schema::Field for #field_marker{
                type Type = #field_type_marker;

                const NAME: &'static ::core::primitive::str = #field_name_literal;
            }

            impl cynic::schema::HasField<#field_marker> for super::super::#parent_marker {
                type Type = #field_type_marker;
            }
        });

        if !self.field.arguments.is_empty() {
            let argument_module = self.field.argument_module().ident();
            let arguments = self
                .field
                .arguments
                .iter()
                .map(|argument| ArgumentOutput::field_argument(argument, field_marker));

            tokens.append_all(quote! {
                pub mod #argument_module {
                    #(#arguments)*
                }
            });
        }
    }
}
