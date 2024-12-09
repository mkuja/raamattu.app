use rust_i18n::t;
use yew::{hook, use_context, use_effect_with, use_state, UseStateHandle};

use crate::context::ApplicationOptions;

/// Contains translation on given translation token.
pub struct Translation {
    trans: String,
}

impl<'a> Translation {
    pub fn new(token: impl ToString, loc: String) -> Self {
        Self {
            trans: t!(token.to_string(), locale = loc).to_string(),
        }
    }

    pub fn get_translation(&self) -> String {
        return self.trans.clone();
    }
}

#[hook]
pub fn use_translation(token: &'static str) -> UseStateHandle<Translation> {
    let context = use_context::<UseStateHandle<ApplicationOptions>>();
    let ctx_copy = context.clone();
    let translation =
        use_state(|| Translation::new(token.to_string(), ctx_copy.unwrap().language.clone()));
    let tr_copy = translation.clone();

    use_effect_with(context, move |v| {
        translation.set(Translation::new(
            token.to_string(),
            v.as_ref().unwrap().language.clone(),
        ));
        ()
    });

    tr_copy
}
