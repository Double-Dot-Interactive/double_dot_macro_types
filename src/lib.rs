use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::Ident;

#[derive(Debug, Clone)]
pub struct StateField(String, String, String);

impl StateField {
    pub fn new(field_name: String, lin_transitions: String, arb_transitions: String) -> Self {
        Self(field_name, lin_transitions, arb_transitions)
    }
}

impl ToTokens for StateField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // Convert the fields of StateField to tokens
        let field = &self.0;
        let linear = &self.1;
        // let arbitrary = self.2.iter().map(|arb| arb.clone());
        let arbitrary = &self.2;

        // tokens.extend(quote! {(
        //     (#field, #linear, #(#arbitrary,)*)
        //     )}
        // )
        tokens.extend(quote! {(
            (#field, #linear, #arbitrary)
            )}
        )
    }
}

pub fn make_ident(name: &str) -> Ident {
    syn::Ident::new(name, Span::call_site())
}