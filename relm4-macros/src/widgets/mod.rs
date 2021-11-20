use proc_macro2::TokenStream as TokenStream2;
use syn::{punctuated::Punctuated, token, Expr, ExprClosure, Generics, Ident, Lit, Path};

use crate::args::Args;

mod gen;
mod parse;

#[derive(Debug, Default)]
pub struct TokenStreams {
    /// The tokens for the struct fields -> name: Type,
    pub struct_fields: TokenStream2,
    /// The tokens initializing the widgets.
    pub init_widgets: TokenStream2,
    /// The tokens connecting widgets.
    pub connect_widgets: TokenStream2,
    /// The tokens initializing the properties.
    pub init_properties: TokenStream2,
    /// The tokens for the returned struct fields -> name,
    pub return_fields: TokenStream2,
    /// The view tokens (watch! macro)
    pub view: TokenStream2,
    /// The view tokens (track! macro)
    pub track: TokenStream2,
    /// The tokens for connecting events.
    pub connect: TokenStream2,
    /// The tokens for connecting events to components.
    pub connect_components: TokenStream2,
    /// The tokens for using the parent stream.
    pub parent: TokenStream2,
}

#[derive(Debug)]
pub(super) struct Tracker {
    bool_fn: Expr,
    update_fns: Vec<Expr>,
}

#[derive(Debug)]
pub(super) enum PropertyType {
    Expr(Expr),
    Value(Lit),
    Track(Tracker),
    Parent(Expr),
    Args(Args<Expr>),
    Connect(ExprClosure),
    ConnectComponent(ExprClosure),
    Watch(TokenStream2),
    Factory(Expr),
    Widget(Widget),
}

#[derive(Debug)]
pub enum PropertyName {
    Ident(Ident),
    Path(Path),
}

#[derive(Debug)]
pub(super) struct Property {
    /// Either a path or just an ident
    pub name: PropertyName,
    pub ty: PropertyType,
    pub generics: Option<Generics>,
    /// Optional arguments like param_name(arg1, arg2, ...)
    pub args: Option<Args<Expr>>,
    /// Assign with an ?
    pub optional_assign: bool,
    /// Iterate through elements to generate tokens
    pub iterative: bool,
}

#[derive(Debug)]
pub(super) struct Properties {
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub(super) struct WidgetFunc {
    pub path_segments: Vec<Ident>,
    pub args: Option<Punctuated<Expr, token::Comma>>,
    pub ty: Option<Vec<Ident>>,
}

#[derive(Debug)]
pub(super) struct Widget {
    pub name: Ident,
    pub func: WidgetFunc,
    pub properties: Properties,
    pub wrapper: Option<Ident>,
    pub assign_as_ref: bool,
    pub returned_widget: Option<ReturnedWidget>,
}

#[derive(Debug)]
pub(super) struct ReturnedWidget {
    pub name: Ident,
    pub ty: Option<Path>,
    pub properties: Properties,
    pub is_optional: bool,
}
