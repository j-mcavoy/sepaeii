use convert_case::{Case, Casing};
use proc_macro::{self, Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, DataEnum, DataUnion, DeriveInput,
    Field, FieldsNamed, FieldsUnnamed, Variant,
};

#[proc_macro_derive(SpriteplexM)]
pub fn spriteplexm(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let states: Vec<syn::Ident> = match data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            variants.iter().map(|v| v.ident.clone()).collect()
        }
        _ => panic!("Spriteplex only works on Enum"),
    };

    let animations: Vec<syn::Ident> = states
        .clone()
        .iter()
        .map(|s| s.to_string().to_case(Case::Snake))
        .map(|s| syn::Ident::new(&s, ident.span()))
        .collect();

    let struct_name: String = ident.to_string().replace("State", "Spriteplex");
    let struct_ident = syn::Ident::new(&struct_name, ident.span());
    let output = quote! {
      #[derive(Debug, Clone, Default)]
      struct #struct_ident {
          #(pub #animations: AnimationStrip,)*
          pub state: #ident
      }
      impl Spriteplex for #struct_ident {
          fn get_animation_strip(&self) -> AnimationStrip {
              match self.state {
                  #(#ident::#states => self.#animations.clone()),*
              }
          }
          fn reset_animation_strip(&mut self) {
              let strip = match self.state {
                  #(#ident::#states => &mut self.#animations),*
              };
              strip.reset()
          }
      }
    };
    println!("{}", output.to_string());
    output.into()
}
