use proc_macro::TokenStream;
use syn::{parse, ItemStruct, Field};
use quote::quote;

#[proc_macro_attribute]
pub fn ptr_project(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let s = parse::<ItemStruct>(item).expect("#[ptr_project] expect struct");
  let ident = &s.ident;
  let (impl_generics, ty_generics, where_clause) = s.generics.split_for_impl();
  let ptr_fields = s.fields.iter().map(|Field { vis, ident, ty, .. }|
    quote! { #vis #ident: *const #ty });
  let ptr_mut_fields = s.fields.iter().map(|Field { vis, ident, ty, .. }|
    quote! { #vis #ident: *mut #ty });
  let fields1 = s.fields.iter().map(|Field { ident, .. }| ident);
  let (fields2, fields3, fields4) = (fields1.clone(), fields1.clone(), fields1.clone());
  let field_refs = s.fields.iter().map(|Field { ident, .. }| quote! { &x.#ident; });
  let res = quote! {
    #s

    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::used_underscore_binding)]
    const _: () = {
      #[allow(dead_code)]
      #[allow(clippy::type_repetition_in_bounds)]
      struct PtrProjection #ty_generics {
        #(#ptr_fields),*
      }
      #[allow(dead_code)]
      #[allow(clippy::type_repetition_in_bounds)]
      struct MutPtrProjection #ty_generics {
        #(#ptr_mut_fields),*
      }
      impl #impl_generics #ident #ty_generics #where_clause {
        pub fn project(self: *const Self) -> PtrProjection #ty_generics {
          unsafe {
            let Self { #(#fields1),* } = &*self;
            PtrProjection { #(#fields2),* }
          }
        }
        pub fn project_mut(self: *mut Self) -> MutPtrProjection #ty_generics {
          unsafe {
            let Self { #(#fields3),* } = &mut *self;
            MutPtrProjection { #(#fields4),* }
          }
        }
      }
      #[deny(safe_packed_borrows)]
      fn __assert_not_repr_packed #impl_generics (x: &#ident #ty_generics) #where_clause {
        #(#field_refs)*
      }
    };
  };
  res.into()
}