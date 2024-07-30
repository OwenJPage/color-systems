use {
    proc_macro::{
        self,
        TokenStream,
    },
    quote::ToTokens,
    syn::{
        parse_macro_input,
        visit_mut::VisitMut,
        Item,
        LitStr,
    },
};

struct InternationalSpelling;

impl VisitMut for InternationalSpelling {
    fn visit_ident_mut(&mut self, ident: &mut syn::Ident) {
        let name = ident
            .to_string()
            .replace("color", "colour")
            .replace("Color", "Colour")
            .replace("COLOR", "COLOUR");
        let span = ident.span();

        *ident = syn::Ident::new(name.as_str(), span);
    }

    fn visit_lit_str_mut(&mut self, lit_str: &mut LitStr) {
        let name = lit_str
            .value()
            .replace("color", "colour")
            .replace("Color", "Colour")
            .replace("COLOR", "COLOUR");
        let span = lit_str.span();

        *lit_str = LitStr::new(name.as_str(), span);
    }
}

#[proc_macro_attribute]
pub fn international_spelling(_: TokenStream, body: TokenStream) -> TokenStream {
    let mut parsed_body = parse_macro_input!(body as Item);

    InternationalSpelling.visit_item_mut(&mut parsed_body);

    parsed_body.into_token_stream().into()
}
