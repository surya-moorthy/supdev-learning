use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields};


#[proc_macro_derive(SerializeNumberSturct)] 
pub  fn serialize_number_struct(input : TokenStream) -> TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = &ast.ident;

   let serialize_fields = match &ast.data {
    Data::Struct(data_struct) => {
        match &data_struct.fields {
           Fields::Named(fields)=> {
            let field_serlializations = fields.named.iter().map(|field| {
                     let field_name = &field.ident;
                     quote! {
                        result.extend_from_slice(&self.#field_name.to_be_bytes());
                     }
                    
            });
            quote! {#(#field_serlializations)*}
           },
            _ => panic!("only structs can be implemented") 
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
     };
   };
   generated.into()
} 

#[proc_macro_derive(DeSerializeNumberSturct)] 
pub  fn deserialize_number_struct(input : TokenStream) -> TokenStream {
        let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    let named = &ast.ident;

    let (deserialized_fields,field_assignments,total_size) = match &ast.data {
         Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                     let mut offset: usize = 0;
                     let mut field_deserializations = Vec::new();
                     let mut field_assignments = Vec::new();

                     for field in &fields.named {
                        let field_name = &field.ident;
                        let field_size: usize = 4;
                        let start_offset = offset;
                        let end_offset = start_offset + field_size;
                        
                        field_deserializations.push(
                           quote! {
                              let #field_name = {
                              let bytes: [u8; 4] = base[#start_offset..#end_offset]
                                       .try_into()
                                       .map_err(|_| Error)?;
                              u32::from_be_bytes(bytes)
                           };
                           }
                        );

                        field_assignments.push(
                           quote! {
                               #field_name
                           }
                        );

                        offset+= field_size;
                     } 
                     
                 (field_deserializations, field_assignments, offset)
                }
                 _ => panic!("only structs can be implemented")
            }
         }
         _ => panic!("only structs can be implemented")
    };
    let generated = quote! {
      impl Deserialize for #named {
         fn deserialize(base : &[u8]) -> Result<Self,Error> {
            if base.len() < #total_size {
               return Err(Error);
            }

            #(#deserialized_fields)*
                
            Ok(#named {
               #(#field_assignments),*
            })
         }
      };
    };

    generated.into()
}