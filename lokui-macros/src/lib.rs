use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parser, parse_macro_input, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn component(_: TokenStream, tokens: TokenStream) -> TokenStream {
	let mut input: ItemStruct = parse_macro_input!(tokens);
	let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();
	let ident = &input.ident;

	if let Fields::Named(ref mut fields) = &mut input.fields {
		fields.named.push(
			syn::Field::parse_named
				.parse2(quote!(entity: Option<usize>))
				.unwrap(),
		);
		let implementation: TokenStream = quote! {
			impl #impl_generics Component for #ident #ty_generics #where_clause {
				fn set_entity(&mut self, entity: usize) {
					self.entity = Some(entity);
				}
				fn to_any(self) -> Box<dyn std::any::Any> {
					Box::new(self) as Box<dyn std::any::Any>
				}
			}
		}
		.into();
		let mut structure: TokenStream = input.into_token_stream().into();
		structure.extend(implementation);
		structure
	} else {
		panic!("Invalid target for component, only structs are supported")
	}
}
