use proc_easy::EasyAttributes;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;

proc_easy::easy_token!(update);
proc_easy::easy_token!(replace);
proc_easy::easy_token!(append);
proc_easy::easy_token!(union);

proc_easy::easy_argument_group! {
    enum KindArg {
        Update(update),
        Replace(replace),
        Append(append),
    }
}

enum Kind {
    Default,
    Update,
    Replace,
    Append,
}

impl From<Option<KindArg>> for Kind {
    fn from(value: Option<KindArg>) -> Self {
        match value {
            Some(KindArg::Update(_)) => Kind::Update,
            Some(KindArg::Replace(_)) => Kind::Replace,
            Some(KindArg::Append(_)) => Kind::Append,
            None => Kind::Default,
        }
    }
}

impl ToTokens for Kind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_token_stream());
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            Kind::Update => quote::quote!(figa::private::Update),
            Kind::Replace => quote::quote!(figa::private::Replace),
            Kind::Append => quote::quote!(figa::private::Append),
            Kind::Default => quote::quote!(figa::private::Default),
        }
    }
}

proc_easy::easy_attributes! {
    @(figa)
    struct FigaAttributes {
        kind: Option<KindArg>,
    }
}

pub fn derive(input: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let ident = &input.ident;
    let ident_lit = syn::LitStr::new(&ident.to_string(), ident.span());
    match input.data {
        syn::Data::Enum(_) => Err(syn::Error::new_spanned(
            input,
            "Figa can only be derived for structs",
        )),
        syn::Data::Union(_) => Err(syn::Error::new_spanned(
            input,
            "Figa can only be derived for structs",
        )),
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Unit => Ok(quote::quote! {
                impl figa::Figa for #ident {
                    fn update<'de, D>(&mut self, deserializer: D) -> figa::private::Result<(), D::Error>
                    where
                        D: figa::private::Deserializer<'de>,
                    {
                        deserializer.deserialize_unit_struct(#ident_lit, figa::private::UnitStructVisitor)
                    }
                }
            }),
            syn::Fields::Named(fields) => {
                let field_names = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();

                let field_name_lits = fields
                    .named
                    .iter()
                    .map(|field| {
                        syn::LitStr::new(&field.ident.as_ref().unwrap().to_string(), field.span())
                    })
                    .collect::<Vec<_>>();

                let field_kinds = fields
                    .named
                    .iter()
                    .map(|field| -> syn::Result<_> {
                        let attrs = FigaAttributes::parse(&field.attrs, field.span())?;
                        Ok(Kind::from(attrs.kind))
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let field_next_seq_element = fields
                    .named
                    .iter()
                    .zip(&field_kinds)
                    .map(|(field, kind)|{
                        let ident = field.ident.as_ref().unwrap();
                        quote::quote_spanned! {field.span() => seq.next_element_seed(#kind(&mut me.#ident))}
                    })
                    .collect::<Vec<_>>();

                let field_next_map_value = fields
                    .named
                    .iter()
                    .zip(&field_kinds)
                    .map(|(field, kind)| {
                        let ident = field.ident.as_ref().unwrap();
                        quote::quote_spanned! {field.span() => map.next_value_seed(#kind(&mut me.#ident))}
                    })
                    .collect::<Vec<_>>();

                Ok(quote::quote! {
                    impl figa::Figa for #ident {
                        fn update<'de, D>(&mut self, deserializer: D) -> figa::private::Result<(), D::Error>
                        where
                            D: figa::private::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum FieldIdent {
                                #(
                                    #field_names,
                                )*
                            }

                            struct FieldIdentVisitor;

                            impl<'de> figa::private::Visitor<'de> for FieldIdentVisitor {
                                type Value = FieldIdent;

                                fn expecting(&self, formatter: &mut figa::private::Formatter) -> figa::private::FmtResult {
                                    formatter.write_str("field identifier")
                                }

                                fn visit_str<E>(self, value: &figa::private::str) -> figa::private::Result<FieldIdent, E>
                                where
                                    E: figa::private::DeError,
                                {
                                    match value {
                                        #(
                                            #field_name_lits => figa::private::Ok(FieldIdent::#field_names),
                                        )*
                                        _ => figa::private::Err(figa::private::DeError::unknown_field(value, &[#(#field_name_lits,)*])),
                                    }
                                }
                            }

                            impl<'de> figa::private::Deserialize<'de> for FieldIdent {
                                fn deserialize<D>(deserializer: D) -> figa::private::Result<Self, D::Error>
                                where
                                    D: figa::private::Deserializer<'de>,
                                {
                                    deserializer.deserialize_identifier(FieldIdentVisitor)
                                }
                            }

                            struct Visitor<'a>(&'a mut #ident);

                            impl<'de> figa::private::Visitor<'de> for Visitor<'_> {
                                type Value = ();

                                fn expecting(&self, formatter: &mut figa::private::Formatter) -> figa::private::FmtResult {
                                    formatter.write_str("struct")
                                }

                                fn visit_unit<E>(self) -> figa::private::Result<(), E> {
                                    figa::private::Ok(())
                                }

                                fn visit_seq<A>(self, mut seq: A) -> figa::private::Result<(), A::Error>
                                where
                                    A: figa::private::SeqAccess<'de>,
                                {
                                    let me = self.0;
                                    #(
                                        #field_next_seq_element?;
                                    )*
                                    figa::private::Ok(())
                                }

                                fn visit_map<A>(self, mut map: A) -> figa::private::Result<(), A::Error>
                                where
                                    A: figa::private::MapAccess<'de>,
                                {
                                    let me = self.0;
                                    while let Some(key) = map.next_key::<FieldIdent>()? {
                                        match key {
                                            #(
                                                FieldIdent::#field_names => {
                                                    #field_next_map_value?;
                                                }
                                            )*
                                        }
                                    }
                                    figa::private::Ok(())
                                }
                            }

                            deserializer.deserialize_struct(#ident_lit, &[#(#field_name_lits,)*], Visitor(self))?;

                            figa::private::Ok(())
                        }
                    }
                })
            }
            syn::Fields::Unnamed(fields) => {
                let fields_count = fields.unnamed.len();
                let field_kinds = fields
                    .unnamed
                    .iter()
                    .map(|field| -> syn::Result<_> {
                        let attrs = FigaAttributes::parse(&field.attrs, field.span())?;
                        Ok(Kind::from(attrs.kind))
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let field_next_seq_element = fields
                    .unnamed
                    .iter()
                    .zip(&field_kinds)
                    .enumerate()
                    .map(|(idx, (field, kind))|{
                        let index = syn::Index::from(idx);
                        quote::quote_spanned! {field.span() => seq.next_element_seed(#kind(&mut me.#index))}
                    })
                    .collect::<Vec<_>>();

                Ok(quote::quote! {
                    impl figa::Figa for #ident {
                        fn update<'de, D>(&mut self, deserializer: D) -> figa::private::Result<(), D::Error>
                        where
                            D: figa::private::Deserializer<'de>,
                        {
                            struct Visitor<'a>(&'a mut #ident);

                            impl<'de> figa::private::Visitor<'de> for Visitor<'_> {
                                type Value = ();

                                fn expecting(&self, formatter: &mut figa::private::Formatter) -> figa::private::FmtResult {
                                    formatter.write_str("struct")
                                }

                                fn visit_unit<E>(self) -> figa::private::Result<(), E> {
                                    figa::private::Ok(())
                                }

                                fn visit_seq<A>(self, mut seq: A) -> figa::private::Result<(), A::Error>
                                where
                                    A: figa::private::SeqAccess<'de>,
                                {
                                    let me = self.0;
                                    #(
                                        #field_next_seq_element?;
                                    )*
                                    figa::private::Ok(())
                                }
                            }

                            deserializer.deserialize_tuple_struct(#ident_lit, #fields_count, Visitor(self))?;

                            figa::private::Ok(())
                        }
                    }
                })
            }
        },
    }
}
