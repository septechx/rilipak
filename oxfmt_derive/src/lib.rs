use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Meta, Path};

fn has_repr_u8(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("repr") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Ok(nested) =
                    meta_list.parse_args_with(Punctuated::<Path, syn::Token![,]>::parse_terminated)
                {
                    for path in nested.iter() {
                        if path.is_ident("u8") {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

#[proc_macro_derive(Serializable, attributes(oxfmt))]
pub fn serializable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Parse #[oxfmt(header = ..., version = ...)]
    let mut header: Option<syn::Lit> = None;
    let mut version: Option<syn::Lit> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("oxfmt") {
            let _ = attr.parse_args_with(|input: syn::parse::ParseStream| {
                while !input.is_empty() {
                    let lookahead = input.lookahead1();
                    if lookahead.peek(syn::Ident) {
                        let ident: syn::Ident = input.parse()?;
                        let _eq: syn::Token![=] = input.parse()?;
                        let lit: syn::Lit = input.parse()?;
                        match ident.to_string().as_str() {
                            "header" => header = Some(lit),
                            "version" => version = Some(lit),
                            _ => {}
                        }
                        if input.peek(syn::Token![,]) {
                            let _: syn::Token![,] = input.parse()?;
                        }
                    } else {
                        break;
                    }
                }
                Ok(())
            });
        }
    }

    let expanded = match &input.data {
        Data::Struct(DataStruct { fields, .. }) => {
            let add_fields = fields.iter().map(|f| {
                let fname = &f.ident;
                quote! { .add(&self.#fname)? }
            });
            if let (Some(header), Some(version)) = (header, version) {
                quote! {
                    impl #impl_generics oxfmt::Serializable for #name #ty_generics #where_clause {
                        fn serialize(&self) -> anyhow::Result<Box<[u8]>> {
                            let result = oxfmt::BinaryBuilder::new(
                                #header.as_bytes(),
                                #version as u16
                            )
                                #(#add_fields)*
                                .build();
                            Ok(result)
                        }
                    }
                }
            } else {
                quote! {
                    impl #impl_generics oxfmt::Serializable for #name #ty_generics #where_clause {
                        fn serialize(&self) -> anyhow::Result<Box<[u8]>> {
                            let result = oxfmt::BinaryBuilder::new_no_meta()
                                #(#add_fields)*
                                .build();
                            Ok(result)
                        }
                    }
                }
            }
        }
        Data::Enum(DataEnum { .. }) => {
            if !has_repr_u8(&input.attrs) {
                return syn::Error::new_spanned(
                    &input.ident,
                    "Serializable can only be derived for #[repr(u8)] enums",
                )
                .to_compile_error()
                .into();
            }
            quote! {
                impl #impl_generics oxfmt::Serializable for #name #ty_generics #where_clause {
                    fn serialize(&self) -> anyhow::Result<Box<[u8]>> {
                        Ok(Box::new([*self as u8]))
                    }
                }
            }
        }
        _ => syn::Error::new_spanned(
            &input.ident,
            "Serializable can only be derived for structs and #[repr(u8)] enums",
        )
        .to_compile_error(),
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Deserializable, attributes(oxfmt))]
pub fn deserializable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = match &input.data {
        Data::Struct(DataStruct { fields, .. }) => {
            let mut structure_fields = Vec::new();
            let mut construct_fields = Vec::new();
            for field in fields {
                let field_ident = field.ident.as_ref().expect("Expected named field");
                let mut mapping = None;
                let mut from_ty = None;
                for attr in &field.attrs {
                    if attr.path().is_ident("oxfmt") {
                        let _ = attr.parse_args_with(|input: syn::parse::ParseStream| {
                            while !input.is_empty() {
                                let lookahead = input.lookahead1();
                                if lookahead.peek(syn::Ident) {
                                    let ident: syn::Ident = input.parse()?;
                                    let _eq: syn::Token![=] = input.parse()?;
                                    let value: syn::Expr = input.parse()?;
                                    match ident.to_string().as_str() {
                                        "mapping" => mapping = Some(quote! { #value }),
                                        "from" => from_ty = Some(quote! { #value }),
                                        _ => {}
                                    }
                                    if input.peek(syn::Token![,]) {
                                        let _: syn::Token![,] = input.parse()?;
                                    }
                                } else {
                                    break;
                                }
                            }
                            Ok(())
                        });
                    }
                }
                let mapping = mapping.unwrap_or_else(|| {
                    syn::Error::new_spanned(
                        &field,
                        "Missing #[oxfmt(mapping = ...)] attribute on field",
                    )
                    .to_compile_error()
                });
                let field_ty = &field.ty;
                let from_ty = from_ty.unwrap_or_else(|| quote! { #field_ty });
                // Only use 'as' if the types differ
                let construct_field = if quote! { #field_ty }.to_string() != from_ty.to_string() {
                    quote! { #field_ident: #field_ty as #from_ty }
                } else {
                    quote! { #field_ident: #field_ty as #field_ty }
                };
                structure_fields.push(mapping);
                construct_fields.push(construct_field);
            }
            quote! {
                impl #impl_generics oxfmt::Deserializable for #name #ty_generics #where_clause {
                    fn get_structure() -> oxfmt::Structure {
                        oxfmt::structure!( #( #structure_fields ),* )
                    }
                    fn construct(mut fields: Vec<Box<dyn std::any::Any>>) -> anyhow::Result<Self> {
                        oxfmt::construct!(fields, #( #construct_fields ),* )
                    }
                }
            }
        }
        _ => syn::Error::new_spanned(
            &input.ident,
            "Deserializable can only be derived for structs with #[oxfmt(mapping = ...)] attributes on fields",
        )
        .to_compile_error(),
    };
    TokenStream::from(expanded)
}
