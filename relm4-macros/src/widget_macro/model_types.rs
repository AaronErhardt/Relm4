use syn::{spanned::Spanned, AngleBracketedGenericArguments, Error, GenericArgument, Result, Type};

//const MODEL_IDENT: &str = "Model";
const GENERICS_ERROR: &str = "Expected at least two generic types for Model and ParentModel";

#[derive(Debug)]
pub(super) struct ModelTypes {
    pub model: Type,
    pub parent_model: Type,
}

impl ModelTypes {
    pub fn new(generics: &AngleBracketedGenericArguments) -> Result<Self> {
        let mut indent_iter = generics.args.clone().into_pairs().filter_map(|pair| {
            let generic = pair.into_value();
            if let GenericArgument::Type(ty) = generic {
                Some(ty)
            } else {
                None
            }
        });

        Ok(ModelTypes {
            model: indent_iter
                .next()
                .ok_or_else(|| Error::new(generics.span(), GENERICS_ERROR))?,
            parent_model: indent_iter
                .next()
                .ok_or_else(|| Error::new(generics.span(), GENERICS_ERROR))?,
        })
    }
}
