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

        // Parse all filterable fields
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
        let mut filter_defaults = Vec::new();
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
            filter_defaults.push(quote! {
                #plural: ::std::collections::HashSet::new()
            });
            filter_methods.push(quote! {
                pub fn #with_singular<V>(mut self, #singular: V) -> Self
                where
                    V: Into<#ty>,
                {
                    self.#plural.insert(#singular.into());
                    self
                }

                pub fn #with_plural<I, V>(mut self, #plural: I) -> Self
                where
                    I: ::std::iter::IntoIterator<Item = V>,
                    V: Into<#ty>,
                {
                    ::std::iter::Extend::extend(&mut self.#plural, #plural.into_iter().map(Into::into));
                    self
                }
            });
            filter_matches.push(quote! {
                if !self.#plural.is_empty() && !self.#plural.contains(&item.#singular) {
                    return false;
                }
            });
        }

        // Create PhantomDatas for each generic type
        //
        // Ideally, we should be able to derive which generics are actually used
        let mut generic_fields = Vec::new();
        let mut generic_defaults = Vec::new();
        let mut generics_where = Vec::new();
        for (i, param) in generics.params.iter().enumerate() {
            match param {
                syn::GenericParam::Lifetime(gen_lifetime) => {
                    let gen_field = format_ident!("_phantom_data{i}");
                    generic_fields.push(quote! {
                        #gen_field: ::std::marker::PhantomData<&#gen_lifetime ()>
                    });
                    generic_defaults.push(quote! {
                        #gen_field: ::std::marker::PhantomData
                    });
                }
                syn::GenericParam::Type(gen_ty) => {
                    let gen_field = format_ident!("_phantom_data{i}");
                    generic_fields.push(quote! {
                        #gen_field: ::std::marker::PhantomData<#gen_ty>
                    });
                    generic_defaults.push(quote! {
                        #gen_field: ::std::marker::PhantomData
                    });
                    generics_where.push(quote! {
                        #gen_ty: ::std::cmp::Eq + ::std::hash::Hash
                    });
                }
                syn::GenericParam::Const(_) => (),
            }
        }

        // Generate the Filter struct and implementations
        let filter = quote! {
            #[derive(Debug)]
            #[non_exhaustive]
            #vis struct #filter_name #generics
            where #(#generics_where,)*
            {
                #(#filter_fields,)*
                #(#generic_fields,)*
            }

            impl #generics Default for #filter_name #generics
            where #(#generics_where),*
            {
                fn default() -> Self {
                    Self {
                        #(#filter_defaults,)*
                        #(#generic_defaults,)*
                    }
                }
            }

            impl #generics #filter_name #generics
            where #(#generics_where,)*
            {
                #(#filter_methods)*

                fn matches(&self, item: &#ident #generics) -> bool {
                    #(#filter_matches)*

                    true
                }
            }

            impl #generics ::struf::Filter for #ident #generics
            where #(#generics_where),*
            {
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
