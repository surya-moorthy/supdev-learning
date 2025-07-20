use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, Field, Fields};

#[proc_macro_derive(SerializeNumberSturct)] 
pub  fn serialize_number_struct(input : TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let name = &ast.ident;

   let serialize_fields = match &ast.data {
    Data::Struct(data_struct) => {
        match &data_struct.fields {
           Fields::Named(fields)=> {
            let field_serlializations = fields.named.iter().map(|field| {
                     let field_name = &field.ident;
                     quote! {
                        result.extend_from_slice(&self.#field_name.to_le_bytes());
                     }
                    
            });
            quote! {field_serlializations}
           },
           Fields::Unnamed(fields)=> {
                let field_serlializations = fields.named.iter().map(|field| {
                     let field_name = &field.ident;
                     quote! {
                        result.extend_from_slice(&self.#field_name.to_le_bytes());
                     }
            });
              
              quote! {field_serlializations}
           },
           Fields::Unit => {
               quote! {}
           }  
        }     
    }
    _ => panic!("no such type implementation")
   };

   let generated = quote! {
        impl Serialize for #name  {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::new();
                 #serialize_fields
                 result
            }
     }
     generated.into()
   };
} 

#[proc_macro_derive(DeSerializeNumberSturct)] 
pub  fn serialize_number_struct(input : TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let named = &ast.ident;

    let (deserialized_fields,field_assignments,total_size) = match &ast.data {
         Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                     
                }
                Fields::Unnamed(fields) => {

                }
                Fields::Unit() => {
                   quote! {}
                }
            }
         }
         _ => panic!("only structs can be implemented")
    };
}