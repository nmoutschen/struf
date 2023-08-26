use darling::{ast, FromAttributes, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Attribute, DeriveInput, Generics, Ident, Type, Visibility};

#[derive(Debug, FromField)]
#[darling(forward_attrs(filter))]
struct FilterField {
    ident: Option<Ident>,
    ty: Type,
    attrs: Vec<Attribute>,
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(filter))]
struct FilterAttr {
    plural: Option<String>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_named))]
struct FilterReceiver {
    vis: Visibility,
    ident: Ident,
    generics: Generics,
    data: ast::Data<(), FilterField>,
}

impl ToTokens for FilterReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            ref vis,
            ref ident,
            ref generics,
            ref data,
        } = self;
        let fields = data
            .as_ref()
            .take_struct()
            .expect("should only be named struct")
            .fields;
        let fields = fields
            .into_iter()
            .filter_map(|field| {
                if !field.attrs.is_empty() {
                    Some((
                        field,
                        FilterAttr::from_attributes(&field.attrs)
                            .expect("should parse filter attribute"),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let filter_name = format_ident!("{ident}Filter");
        let mut filter_fields = Vec::new();
        let mut filter_methods = Vec::new();
        let mut filter_matches = Vec::new();

        for (field, attr) in fields {
            let singular = field.ident.as_ref().unwrap();
            let plural = if let Some(plural) = attr.plural.as_ref() {
                format_ident!("{plural}")
            } else {
                format_ident!("{singular}s")
            };
            let with_singular = format_ident!("with_{singular}");
            let with_plural = format_ident!("with_{plural}");
            let ty = &field.ty;
            filter_fields.push(quote! {
                pub #plural: ::std::collections::HashSet<#ty>
            });
            filter_methods.push(quote! {
                pub fn #with_singular<V>(mut self, #singular: V) -> Self
                where V: Into<#ty> {
                    self.#plural.insert(#singular.into());
                    self
                }

                pub fn #with_plural<I>(mut self, #plural: I) -> Self
                where I: ::std::iter::IntoIterator<Item = #ty> {
                    ::std::iter::Extend::extend(&mut self.#plural, #plural);
                    self
                }
            });
            filter_matches.push(quote! {
                if !self.#plural.is_empty() && !self.#plural.contains(&item.#singular) {
                    return false;
                }
            });
        }

        let filter = quote! {
            #[derive(Debug, Default)]
            #vis struct #filter_name #generics {
                #(#filter_fields),*
            }

            impl #generics #filter_name #generics {
                #(#filter_methods)*

                fn matches(&self, item: &#ident) -> bool {
                    #(#filter_matches)*

                    true
                }
            }

            impl #generics ::struf::Filter for #ident #generics {
                type Filter = #filter_name #generics;

                fn filter() -> Self::Filter {
                    ::std::default::Default::default()
                }
            }
        };

        tokens.extend(filter);
    }
}

#[proc_macro_derive(Filter, attributes(filter))]
pub fn filter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let receiver = FilterReceiver::from_derive_input(&input).unwrap();

    quote!(#receiver).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = syn::parse_str(
            r#"
            #[derive(Filter)]
            pub struct MyStruct<T: Hash> {
                a: T,
                #[filter]
                b: T,
            }"#,
        )
        .unwrap();

        let receiver = FilterReceiver::from_derive_input(&input).unwrap();

        let tokens = quote!(#receiver);

        println!("{tokens}");
    }
}
