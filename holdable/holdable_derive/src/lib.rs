use proc_macro::TokenStream;
use quote::quote;
use std::any::Any;
use syn;

#[proc_macro_derive(Holdable)]
pub fn holdable_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();
	impl_holdable(&ast)
}

fn impl_holdable(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let gen = quote! {
	impl Holdable for #name {
		fn as_any(&self) -> &dyn Any {
			self
		}

		fn as_any_mut(&mut self) -> &mut dyn Any {
			self
		}
	}
	};
	gen.into()
}
