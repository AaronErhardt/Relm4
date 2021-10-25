use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Error, ExprCall, ExprField, ExprPath, Ident, Member, Path, Type};

use super::{Expr, Generics, Property, PropertyName, PropertyType, Tracker, Widget, WidgetFunc};

/// Helper function for the tracker macro.
fn expr_field_from_expr_call(call_expr: &ExprCall) -> Option<&ExprField> {
    let first_expr = call_expr.args.iter().next()?;
    if let Expr::Field(expr_field) = first_expr {
        Some(expr_field)
    } else {
        None
    }
}

fn generate_tracker_from_expression(
    expression: &Expr,
    model_name: &Type,
) -> Result<TokenStream2, (Span2, String)> {
    let error_fn = move |span, msg: &str| {
        let error_msg =
                    "Unable to generate tracker function. Please pass a tracker function as the first parameter to the `track!` macro.\n\
                    Usage: track!(TRACK_CONDITION: bool, FIRST_ARG, SECOND_ARG, ...)";
        Err((span, format!("{}\nHint:  {}", error_msg, msg)))
    };

    let unref_expr: &Expr = if let Expr::Reference(expr_ref) = expression {
        &expr_ref.expr
    } else {
        expression
    };

    let expr_field_opt = match unref_expr {
        Expr::Call(call_expr) => expr_field_from_expr_call(call_expr),
        Expr::MethodCall(expr_method_call) => {
            if let Expr::Field(ref expr_field) = *expr_method_call.receiver {
                Some(expr_field)
            } else {
                None
            }
        }
        _ => None,
    };

    let expr_field = if let Some(expr_field) = expr_field_opt {
        expr_field
    } else {
        return error_fn(
            unref_expr.span(),
            "Couldn't find find a call or method expression.",
        );
    };

    let base_is_model = if let Expr::Path(expr_path) = &*expr_field.base {
        if let Some(ident) = expr_path.path.get_ident() {
            ident == "model"
        } else {
            false
        }
    } else {
        false
    };

    if !base_is_model {
        return error_fn(
            expr_field.base.span(),
            "Couldn't find a reference to `model`.",
        );
    }

    let ident = if let Member::Named(ident) = &expr_field.member {
        ident.clone()
    } else {
        return error_fn(expr_field.member.span(), "Expected a named member");
    };

    let bool_stream =
        quote_spanned! { expr_field.span() => model.changed( #model_name::#ident() ) };
    Ok(bool_stream)
}

fn component_ident(path: &ExprPath) -> TokenStream2 {
    if path.path.segments.len() == 1 {
        let ident = &path.path.segments.first().unwrap().ident;
        quote_spanned! { path.span() => components.#ident.root_widget() }
    } else {
        path.to_token_stream()
    }
}

fn component_tokens(expr: &Expr) -> TokenStream2 {
    match expr {
        Expr::Call(call) => {
            if let Expr::Path(path) = &*call.func {
                if let Some(segs) = path.path.segments.first() {
                    if segs.ident == "Some" {
                        if call.args.len() == 1 {
                            if let Expr::Path(args_path) = call.args.first().unwrap() {
                                let arg_tokens = component_ident(args_path);
                                quote_spanned! { path.span() => Some(#arg_tokens) }
                            } else {
                                expr.to_token_stream()
                            }
                        } else {
                            expr.to_token_stream()
                        }
                    } else {
                        expr.to_token_stream()
                    }
                } else {
                    expr.to_token_stream()
                }
            } else {
                expr.to_token_stream()
            }
        }
        Expr::Path(path) => component_ident(path),
        _ => expr.to_token_stream(),
    }
}

impl PropertyType {
    fn init_assign_tokens(&self) -> Option<TokenStream2> {
        match self {
            PropertyType::Expr(expr) => Some(expr.to_token_stream()),
            PropertyType::Value(lit) => Some(lit.to_token_stream()),
            PropertyType::Watch(tokens) => Some(tokens.to_token_stream()),
            PropertyType::Args(args) => Some(args.to_token_stream()),
            PropertyType::Track(Tracker {
                bool_fn,
                update_fns,
            }) => Some(if update_fns.is_empty() {
                quote! { #bool_fn }
            } else {
                quote! { #(#update_fns),* }
            }),
            _ => None,
        }
    }

    fn view_assign_tokens(&self) -> Option<TokenStream2> {
        match self {
            PropertyType::Watch(token_stream) => Some(token_stream.clone()),
            _ => None,
        }
    }

    fn connect_assign_tokens(&self) -> Option<TokenStream2> {
        if let PropertyType::Connect(closure) = self {
            Some(closure.to_token_stream())
        } else {
            None
        }
    }

    fn track_tokens(&self, model_name: &Type) -> Option<(TokenStream2, TokenStream2)> {
        if let PropertyType::Track(Tracker {
            update_fns,
            bool_fn,
        }) = self
        {
            // Only one parameter passed. Try to generate tracker.
            if update_fns.is_empty() {
                let bool_stream = match generate_tracker_from_expression(bool_fn, model_name) {
                    Ok(bool_tokens) => bool_tokens,
                    Err((span, msg)) => {
                        return Some((
                            Error::new(span, &msg).to_compile_error(),
                            TokenStream2::new(),
                        ));
                    }
                };
                Some((bool_stream, bool_fn.to_token_stream()))
            } else {
                let update_stream = quote! { #(#update_fns),* };
                let bool_stream = bool_fn.to_token_stream();

                // TODO: Uncomment this and add a warning once proc-macro warning are stable
                /*if update_fns.len() == 1 {
                    if let Ok(auto_bool_stream) =
                        generate_tracker_from_expression(&update_fns[0], model_name)
                    {
                        if auto_bool_stream.to_string() == bool_stream.to_string() {
                            let error_msg = "Consider removing the first parameter because the macro would generate the same code.\n";
                            /*println!(Some((
                                Error::new(bool_fn.span(), error_msg).to_compile_error(),
                                update_stream,
                            ));*/
                        }
                    }
                }*/

                Some((bool_stream, update_stream))
            }
        } else {
            None
        }
    }

    fn factory_expr(&self) -> Option<TokenStream2> {
        if let PropertyType::Factory(expr) = self {
            Some(expr.to_token_stream())
        } else {
            None
        }
    }

    fn component_tokens(&self) -> Option<TokenStream2> {
        match self {
            PropertyType::Widget(widget) => Some(widget.widget_assignment()),
            PropertyType::Component(expr) => Some(component_tokens(expr)),
            _ => None,
        }
    }

    fn connect_component_tokens(&self) -> Option<TokenStream2> {
        if let PropertyType::ConnectComponent(closure) = self {
            Some(closure.to_token_stream())
        } else {
            None
        }
    }
}

impl Property {
    fn args_stream(&self) -> TokenStream2 {
        if let Some(args) = &self.args {
            quote! { ,#args }
        } else {
            TokenStream2::new()
        }
    }
}

impl WidgetFunc {
    pub fn type_token_stream(&self) -> TokenStream2 {
        let mut tokens = TokenStream2::new();

        // If type was specified, use it
        let segments = if let Some(ty) = &self.ty {
            &ty[..]
        } else if self.args.is_some() {
            // If for example gtk::Box::new() was used, ignore ::new()
            // and use gtk::Box as type.
            let len = self.path_segments.len();
            if len == 0 {
                return Error::new(self.span().unwrap().into(), "Expected path here.")
                    .into_compile_error();
            } else if len == 1 {
                return Error::new(self.span().unwrap().into(), &format!("You need to specify a type of your function. Use this instead: {}() -> type {{", self.path_segments.first().unwrap())).into_compile_error();
            } else {
                let last_index = len - 1;
                &self.path_segments[0..last_index]
            }
        } else {
            &self.path_segments[..]
        };

        let mut seg_iter = segments.iter();
        let first = if let Some(first) = seg_iter.next() {
            first
        } else {
            return Error::new(
                self.span().unwrap().into(),
                "No path segments in WidgetFunc.",
            )
            .into_compile_error();
        };
        tokens.extend(first.to_token_stream());

        for segment in seg_iter {
            tokens.extend(quote! {::});
            tokens.extend(segment.to_token_stream());
        }

        tokens
    }

    pub fn func_token_stream(&self) -> TokenStream2 {
        let mut tokens = TokenStream2::new();

        let mut seg_iter = self.path_segments.iter();
        tokens.extend(
            seg_iter
                .next()
                .expect("No path segments in WidgetFunc. Can't generate function tokens.")
                .to_token_stream(),
        );

        for segment in seg_iter {
            tokens.extend(quote! {::});
            tokens.extend(segment.to_token_stream());
        }

        if let Some(args) = &self.args {
            tokens.extend(quote! {(#args)});
            tokens
        } else {
            quote! {
                #tokens::default()
            }
        }
    }
}

impl PropertyName {
    fn assign_fn_stream(&self, p_generics: &Option<Generics>, w_name: &Ident) -> TokenStream2 {
        let mut tokens = match self {
            PropertyName::Ident(ident) => {
                quote! { #w_name.#ident }
            }
            PropertyName::Path(path) => quote! { #path },
        };

        if let Some(generics) = p_generics {
            tokens.extend(quote! { :: #generics });
        }

        tokens
    }

    fn assign_args_stream(&self, w_name: &Ident) -> Option<TokenStream2> {
        match self {
            PropertyName::Ident(_) => None,
            PropertyName::Path(_) => Some(quote! { &#w_name, }),
        }
    }

    fn self_assign_fn_stream(&self, p_generics: &Option<Generics>, w_name: &Ident) -> TokenStream2 {
        let mut tokens = match self {
            PropertyName::Ident(ident) => {
                quote! { self.#w_name.#ident }
            }
            PropertyName::Path(path) => quote! { #path },
        };

        if let Some(generics) = p_generics {
            tokens.extend(quote! { :: #generics });
        }

        tokens
    }

    fn self_assign_args_stream(&self, w_name: &Ident) -> Option<TokenStream2> {
        match self {
            PropertyName::Ident(_) => None,
            PropertyName::Path(_) => Some(quote! { &self.#w_name, }),
        }
    }
}

impl Widget {
    pub fn return_stream(&self) -> TokenStream2 {
        let w_span = self.func.span();
        let w_name = &self.name;
        quote_spanned! {
            w_span => #w_name,
        }
    }

    pub fn widget_assignment(&self) -> TokenStream2 {
        let w_span = self.func.span();
        let w_name = &self.name;
        let out_stream = if self.assign_as_ref {
            quote_spanned! { w_span => & self.#w_name}
        } else {
            quote! { self.#w_name}
        };
        if let Some(wrapper) = &self.wrapper {
            quote_spanned! {
                wrapper.span() => #wrapper(#out_stream)
            }
        } else {
            out_stream
        }
    }

    pub fn view_stream(&self, relm4_path: &Path, property_stream: &mut TokenStream2) {
        let w_name = &self.name;

        for prop in &self.properties.properties {
            let p_assign_opt = prop.ty.view_assign_tokens();
            if let Some(p_assign) = p_assign_opt {
                let assign_fn = prop.name.self_assign_fn_stream(&prop.generics, w_name);
                let self_assign_args = prop.name.self_assign_args_stream(w_name);

                property_assign_tokens(
                    property_stream,
                    prop,
                    assign_fn,
                    self_assign_args,
                    p_assign,
                    None,
                );
            }

            let fact_assign_opt = prop.ty.factory_expr();
            if let Some(f_expr) = fact_assign_opt {
                property_stream.extend(quote! {
                    #relm4_path::factory::Factory::generate(&#f_expr, &self.#w_name, sender.clone());
                });
            }
        }
    }

    pub fn property_assign_stream(&self, relm4_path: &Path, property_stream: &mut TokenStream2) {
        let w_name = &self.name;

        for prop in &self.properties.properties {
            let p_assign_opt = prop.ty.init_assign_tokens();
            if let Some(p_assign) = p_assign_opt {
                let args_stream = prop.args_stream();

                let assign_fn = prop.name.assign_fn_stream(&prop.generics, w_name);
                let self_assign_args = prop.name.assign_args_stream(w_name);

                property_assign_tokens(
                    property_stream,
                    prop,
                    assign_fn,
                    self_assign_args,
                    p_assign,
                    Some(args_stream),
                );
            }

            let fact_assign_opt = prop.ty.factory_expr();
            if let Some(f_expr) = fact_assign_opt {
                property_stream.extend(quote! {
                    #relm4_path::factory::Factory::generate(&#f_expr, &#w_name, sender.clone());
                });
            }
        }
    }

    pub fn connect_stream(&self) -> TokenStream2 {
        let w_name = &self.name;
        let mut stream = TokenStream2::new();

        for prop in &self.properties.properties {
            let p_assign_opt = prop.ty.connect_assign_tokens();
            if let Some(p_assign) = p_assign_opt {
                let p_name = &prop.name;
                let p_span = p_name.span().unwrap().into();

                let assign_fn = prop.name.assign_fn_stream(&prop.generics, w_name);
                let self_assign_args = prop.name.assign_args_stream(w_name);

                let mut clone_stream = TokenStream2::new();
                if let Some(args) = &prop.args {
                    for arg in &args.inner {
                        clone_stream.extend(quote_spanned! { arg.span() =>
                            #[allow(clippy::redundant_clone)]
                            let #arg = #arg.clone();
                        });
                    }
                }

                stream.extend(quote_spanned! {
                    p_span => {
                        #clone_stream
                        #assign_fn(#self_assign_args #p_assign);
                    }
                });
            }
        }

        stream
    }

    pub fn connect_component_stream(&self) -> TokenStream2 {
        let w_name = &self.name;
        let mut stream = TokenStream2::new();

        for prop in &self.properties.properties {
            let p_assign_opt = prop.ty.connect_component_tokens();
            if let Some(p_assign) = p_assign_opt {
                let p_name = &prop.name;
                let p_span = p_name.span().unwrap().into();

                let assign_fn = prop.name.self_assign_fn_stream(&prop.generics, w_name);
                let self_assign_args = prop.name.self_assign_args_stream(w_name);

                let mut arg_stream = TokenStream2::new();
                if let Some(args) = &prop.args {
                    for arg in &args.inner {
                        arg_stream.extend(quote_spanned! { arg.span() =>
                            let #arg;
                        });
                    }
                }

                stream.extend(quote_spanned! {
                    p_span => {
                        #arg_stream
                        #assign_fn(#self_assign_args #p_assign);
                    }
                });
            }
        }

        stream
    }

    pub fn track_stream(&self, ty: &Type) -> TokenStream2 {
        let w_name = &self.name;
        let mut stream = TokenStream2::new();

        for prop in &self.properties.properties {
            let p_assign_opt = prop.ty.track_tokens(ty);
            if let Some((bool_stream, update_stream)) = p_assign_opt {
                let p_name = &prop.name;
                let p_span = p_name.span().unwrap().into();

                let assign_fn = prop.name.self_assign_fn_stream(&prop.generics, w_name);
                let self_assign_args = prop.name.self_assign_args_stream(w_name);
                let args_stream = prop.args_stream();

                stream.extend(quote_spanned! {
                    p_span =>  if #bool_stream {
                        #assign_fn(#self_assign_args #update_stream #args_stream);
                }});
            }
        }
        stream
    }

    pub fn component_stream(&self) -> TokenStream2 {
        let w_name = &self.name;
        let mut stream = TokenStream2::new();

        for prop in &self.properties.properties {
            let p_assign_opt = prop.ty.component_tokens();
            if let Some(component_tokens) = p_assign_opt {
                let args_stream = prop.args_stream();
                let assign_fn = prop.name.self_assign_fn_stream(&prop.generics, w_name);
                let self_assign_args = prop.name.self_assign_args_stream(w_name);

                property_assign_tokens(
                    &mut stream,
                    prop,
                    assign_fn,
                    self_assign_args,
                    component_tokens,
                    Some(args_stream),
                );
            }
        }
        stream
    }
}

fn property_assign_tokens(
    stream: &mut TokenStream2,
    prop: &Property,
    assign_fn: TokenStream2,
    self_assign_args: Option<TokenStream2>,
    p_assign: TokenStream2,
    args_stream: Option<TokenStream2>,
) {
    let p_name = &prop.name;
    let p_span = p_name.span().unwrap().into();
    stream.extend(match (prop.optional_assign, prop.iterative) {
        (false, false) => {
            quote_spanned! {
                p_span => #assign_fn(#self_assign_args #p_assign #args_stream);
            }
        }
        (true, false) => {
            quote_spanned! {
                p_span => if let Some(__p_assign) = #p_assign {
                    #assign_fn(#self_assign_args __p_assign #args_stream);
                }
            }
        }
        (false, true) => {
            quote_spanned! {
                p_span => for __elem in #p_assign {
                    #assign_fn(#self_assign_args __elem #args_stream );
                }
            }
        }
        (true, true) => {
            quote_spanned! {
                p_span => for __elem in #p_assign {
                    if let Some(__p_assign) = __elem {
                        #assign_fn(#self_assign_args __p_assign #args_stream );
                    }
                }
            }
        }
    });
}
