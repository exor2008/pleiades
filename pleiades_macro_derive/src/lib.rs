use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::Expr::Let;
use syn::{
    parse, parse_macro_input, DeriveInput, Ident, ItemEnum, Token, Type, TypeParam, TypeParamBound,
    WherePredicate,
};
use to_snake_case::ToSnakeCase;

#[proc_macro_derive(Flush)]
pub fn pleiades_flush_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse(input).unwrap();

    // Build the trait implementation
    impl_pleiades_flush(&ast)
}

fn impl_pleiades_flush(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics Flush for #name #ty_generics #where_clause
        {
            async fn flush(&mut self, led: &mut Led) {
                led.flush().await;
            }
        }
    };
    gen.into()
}

struct Args {
    variants: Vec<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let variants = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            variants: variants.into_iter().collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn enum_world(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemEnum);
    let args = parse_macro_input!(attr as Args);
    let name = &item.ident;

    let (impl_generics, ty_generics, where_clause) = &item.generics.split_for_impl();

    let mut buff_type = None;
    if let Some(where_clause) = where_clause {
        for predicate in &where_clause.predicates {
            if let WherePredicate::Type(tp) = &predicate {
                for bound in &tp.bounds {
                    if let TypeParamBound::Trait(trt) = bound {
                        for seg in &trt.path.segments {
                            if seg.ident == "Buffer" {
                                buff_type = Some(&tp.bounded_ty);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut new_world_funcs = quote! {};
    let mut match_blocks = quote! {};
    let mut on_directions_funcs = quote! {};

    for variant in &args.variants {
        let snake = format_ident!("{}", variant.to_string().to_snake_case());
        let func_name = format_ident!("{}_new", snake);
        let func_code = quote! {
            pub fn #func_name () -> Self {
                let #snake = #snake::#variant::new();
                World::#variant(#snake)
            }
        };
        new_world_funcs.extend(func_code);

        let match_block = quote! {
            Self::#variant(ref mut #snake) => {
                #snake.tick(buffer).await;
                // #snake.flush().await;
            }
        };
        match_blocks.extend(match_block);

        let on_direction_func_code = quote! {
            Self::#variant(#snake) => #snake.on_direction(direction),
        };
        on_directions_funcs.extend(on_direction_func_code);
    }

    let gen = quote! {
        #item

        impl #impl_generics #name #ty_generics #where_clause
        {
            #new_world_funcs

            pub async fn tick(world: &mut World #ty_generics, buffer: &mut #buff_type) {
                match world {
                    #match_blocks
                }
            }
        }

        impl #impl_generics OnDirection for #name #ty_generics #where_clause
        {
            fn on_direction(&mut self, direction: Direction) {
                match self {
                    #on_directions_funcs
                }
            }
        }
    };

    gen.into()
}
