extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HasSprite)]
pub fn has_sprite_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_has_sprite(&ast);
    gen.parse().unwrap()
}

fn impl_has_sprite(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HasSprite for #name {
            fn get_sprite(&self) -> SpriteName {
                self.sprite
            }
        }
    }
}

#[proc_macro_derive(HasTile)]
pub fn has_tile_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_has_tile(&ast);
    gen.parse().unwrap()
}

fn impl_has_tile(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HasTile for #name {
            fn get_tile(&self) -> Tile {
                self.tile
            }
        }
    }
}
