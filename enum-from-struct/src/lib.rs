use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(EnumFromStruct)]
pub fn enum_from_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let enum_name = syn::Ident::new(&format!("{}Enum", struct_name), struct_name.span());

    let fields = match input.data {
        Data::Struct(data_struct) => {
            if let Fields::Named(fields) = data_struct.fields {
                fields.named.into_iter()
            } else {
                panic!("Only named fields are supported");
            }
        }
        _ => panic!("Only struct types are supported"),
    };

    let enum_variants = fields.map(|field| {
        let field_name = field.ident.unwrap();
        quote! { #field_name }
    });

    let expanded = quote! {
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        enum #enum_name {
            #(#enum_variants,)*
        }

        impl From<#struct_name> for #enum_name {
            fn from(_s: #struct_name) -> Self {
                unimplemented!()
            }
        }
    };

    expanded.into()
}
