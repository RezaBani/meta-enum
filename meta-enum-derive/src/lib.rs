use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(MetaEnum)]
pub fn derive_meta_enum(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let ident = ast.ident;
    let len = match &ast.data {
        syn::Data::Enum(data_enum) => Some(data_enum.variants.len()),
        _ => None,
    };
    let variants: Vec<_> = match &ast.data {
        syn::Data::Enum(data_enum) => data_enum.variants.iter().map(|v| &v.ident).collect(),
        _ => Vec::new(),
    };
    let keys: Vec<_> = match &ast.data {
        syn::Data::Enum(data_enum) => data_enum
            .variants
            .iter()
            .map(|v| v.ident.to_string())
            .collect(),
        _ => Vec::new(),
    };
    let mut last_value = -1;
    let values: Vec<i32> = match &ast.data {
        syn::Data::Enum(data_enum) => data_enum
            .variants
            .iter()
            .map(|v| match &v.discriminant {
                Some(discriminant) => match &discriminant.1 {
                    syn::Expr::Lit(literal) => match &literal.lit {
                        syn::Lit::Int(value) => {
                            last_value = value.base10_parse().ok().unwrap();
                            last_value
                        }
                        _ => -1,
                    },
                    _ => -1,
                },
                None => match v.fields {
                    syn::Fields::Unit => {
                        last_value += 1;
                        last_value
                    }
                    _ => -1,
                },
            })
            .collect(),
        _ => Vec::new(),
    };
    if let Some(len) = len {
        quote! {
            impl MetaEnum for #ident {
                fn count() -> usize {
                    #len
                }
                fn keys() -> Vec<String> {
                    vec![#(#keys),*].into_iter().map(|key|key.to_string()).collect()
                }
                fn values() -> Vec<i32> {
                    vec![#(#values),*]
                }
            }
            impl core::str::FromStr for #ident {
                type Err = ParseMetaEnumError<#ident>;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    for (key, variant) in vec![#(#keys),*].into_iter().zip(vec![#(#ident::#variants),*]) {
                        if key.to_ascii_lowercase() == s.to_ascii_lowercase() {
                            return Ok(variant);
                        }
                    }
                    Err(ParseMetaEnumError::new())
                }
            }
            impl From<i32> for #ident {
                fn from(value: i32) -> #ident {
                    match value {
                        #(x if x == #ident::#variants as i32 => #ident::#variants),*,
                        _ => unreachable!(),
                    }
                }
            }

            impl From<u8> for #ident {
                fn from(value: u8) -> #ident {
                    match value {
                        #(x if x == #ident::#variants as u8 => #ident::#variants),*,
                        _ => unreachable!(),
                    }
                }
            }
            impl Into<i32> for #ident {
                fn into(self) -> i32 {
                    self as i32
                }
            }
            impl Into<u8> for #ident {
                fn into(self) -> u8 {
                    self as u8
                }
            }
        }
        .into()
    } else {
        quote! {
            impl MetaEnum for #ident {
                fn count() -> usize {
                    compile_error!("MetaEnum Macro Only works on Enums");
                }
                fn keys() -> Vec<String> {
                    compile_error!("MetaEnum Macro Only works on Enums");
                }
                fn values() -> Vec<i32> {
                    compile_error!("MetaEnum Macro Only works on Enums");
                }
            }
        }
        .into()
    }
}
