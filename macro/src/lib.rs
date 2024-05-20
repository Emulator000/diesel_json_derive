use heck::ToSnakeCase;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(DieselJsonb)]
pub fn diesel_jsonb_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident;
    let mod_name = format!("{}_diesel_jsonb", type_name.to_string().to_snake_case());
    let mod_name = Ident::new(&mod_name, type_name.span());

    (quote! {
        mod #mod_name {
            use super::#type_name;

            use diesel::deserialize::{self, FromSql};
            use diesel::pg::{Pg, PgValue};
            use diesel::serialize::{self, ToSql};
            use diesel::sql_types::*;
            use std::io::Write;

            impl ToSql<Jsonb, Pg> for #type_name {
                fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
                    out.write_all(&[1])?;
                    serde_json::to_writer(out, &self)?;
                    Ok(serialize::IsNull::No)
                }
            }

            impl FromSql<Jsonb, Pg> for #type_name {
                fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
                    let bytes = bytes.as_bytes();
                    if bytes[0] != 1 {
                        return Err("Unsupported JSONB encoding version".into());
                    }
                    serde_json::from_slice(&bytes[1..]).map_err(|_| "Invalid Json".into())
                }
            }
        }
    }).into()
}
