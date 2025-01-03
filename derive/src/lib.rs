extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Generics, Ident, Index, LitBool, Path, PathSegment, PredicateType, Token, TraitBound, TraitBoundModifier, Type, TypeParamBound, WhereClause, WherePredicate};
use syn::punctuated::Punctuated;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;

struct AbomonateInput {
    path: Path,
    generics: Generics,
    field_types: Punctuated<Type, Token![,]>,
    has_source: bool,
}

impl AbomonateInput {
    fn out_name(&self) -> Ident {
        let last = self.path.segments.last().unwrap();
        Ident::new(&format!("{}Abomonated", last.ident), last.ident.span())
    }

    fn source_type(&self) -> proc_macro2::TokenStream {
        let path = &self.path;
        let (_, ty_generics, _) = self.generics.split_for_impl();

        // Source type (if any).
        if self.has_source {
            quote! { #path #ty_generics }
        } else {
            quote! { () }
        }
    }

    fn add_abomonate_bounds(&self) -> Generics{
        let generics = &self.generics;

        // Generics for impl block must also require all inner types : `Abomination`.
        let mut abomonate_generics = generics.clone();
        if abomonate_generics.where_clause.is_none() {
            abomonate_generics.where_clause = Some(WhereClause {
                where_token: Default::default(),
                predicates: Punctuated::new(),
            });
        }
        for field_type in &self.field_types {
            let mut bounds = Punctuated::new();
            bounds.push(abomonation_trait_bound());
            abomonate_generics.where_clause.as_mut().unwrap().predicates.push(
                WherePredicate::Type(
                    PredicateType {
                        lifetimes: None,
                        bounded_ty: (*field_type).clone(),
                        colon_token: Default::default(),
                        bounds,
                    }
                )
            );
        }

        abomonate_generics
    }
}

impl Parse for AbomonateInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        let _: Token![,] = input.parse()?;
        let generics = input.parse()?;
        let _: Token![,] = input.parse()?;

        let content;
        let _: Paren = syn::parenthesized!(content in input);
        let field_types = Punctuated::parse_terminated(&content)?;
        let mut has_source = true;
        if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            let literal: LitBool = input.parse()?;
            has_source = literal.value;
        }

        Ok(AbomonateInput {
            path,
            generics,
            field_types,
            has_source,
        })
    }
}

fn abomonation_trait_bound() -> TypeParamBound {
    TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: Path {
            leading_colon: Some(Default::default()),
            segments: Punctuated::from_iter([
                PathSegment::from(Ident::new("abomonation", Span::call_site())),
                PathSegment::from(Ident::new("external", Span::call_site())),
                PathSegment::from(Ident::new("ExternalAbomonation", Span::call_site())),
            ]),
        },
    })
}

fn generate(input: AbomonateInput) -> proc_macro2::TokenStream {
    // Name of output struct.
    let out_name = input.out_name();

    // Generics for impl block.
    let abomonate_generics = input.add_abomonate_bounds();
    let (impl_generics, ty_generics, where_clause) = abomonate_generics.split_for_impl();

    // Source fully-qualified type.
    let source = input.source_type();

    // Vectors of fields and their indices for offsets.
    let generics = &input.generics;
    let fields = input.field_types.clone().into_iter().collect::<Vec<_>>();
    let indices = (0..fields.len()).map(Index::from).collect::<Vec<_>>();

    quote! {
        pub struct #out_name #generics (
            #(pub #fields),*
        );

        impl #impl_generics ::abomonation::external::ExternalAbomonation for #out_name #ty_generics #where_clause {
            type Source = #source;
            type Description = (#(::abomonation::external::StructDescriptor<#fields>,)*);
            const DESCRIPTION: Self::Description = (
                #(::abomonation::external::StructDescriptor::new(::abomonation::external::offset_of!(Self, #indices)),)*
            );
        }
    }
}

#[proc_macro]
pub fn external_abomoniate(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as AbomonateInput);
    generate(input).into()
}