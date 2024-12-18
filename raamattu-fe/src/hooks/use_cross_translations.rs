use gloo_net::http::Request;
use log::info;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};
use yew_router::{hooks::use_route, Routable};

use crate::{context::ApplicationOptions, Route};

#[derive(Deserialize, PartialEq)]
pub struct AlternativeBookTranslations {
    pub book_id: i32,
    pub book_color: String,
    pub short_name: String,
    pub full_name: String,
    pub language: String,
    pub translation: String,
    pub translation_description: String,
    pub matching: Option<Vec<AlternativeBookTranslations>>,
}

impl AlternativeBookTranslations {
    pub fn to_route(&self, chapter: Option<i32>) -> Route {
        if chapter.is_some() {
            Route::Chapter {
                translation: self.translation.clone(),
                book: self.short_name.clone(),
                chapter: chapter.unwrap().to_string(),
            }
        } else {
            Route::Chapters {
                translation: self.translation.clone(),
                book: self.short_name.clone(),
            }
        }
    }
}

#[hook]
pub fn use_cross_translations() -> UseStateHandle<Option<AlternativeBookTranslations>> {
    // The return value
    let alternative_translations: UseStateHandle<Option<AlternativeBookTranslations>> =
        use_state(|| None);
    let is_loading = use_state(|| false);
    let error = use_state(|| None);
    let route: Option<Route> = use_route();

    // current route parts. Used for request to be for corresponding translation books.
    let route_parts = match route.unwrap() {
        Route::Chapters { translation, book } => Some((translation, book, None)),
        Route::Chapter {
            translation,
            book,
            chapter,
        } => Some((translation, book, Some(chapter))),
        _ => None,
    };
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();

    let is_loading_copy = is_loading.clone();
    let error_copy = error.clone();
    let alt_trans = alternative_translations.clone();
    use_effect_with((), move |_| {
        let is_loading_copy2 = is_loading_copy.clone();
        is_loading_copy.set(true);
        let ctx = ctx.clone();
        let route_parts = route_parts.clone();
        if route_parts.is_some() {
            spawn_local(async move {
                let url = if route_parts.as_ref().unwrap().2.is_none() {
                    format!(
                        "{}/other-translations/{}/{}",
                        ctx.backend_base_url,
                        route_parts.as_ref().unwrap().0,
                        route_parts.as_ref().unwrap().1,
                    )
                } else {
                    // chapter is some
                    format!(
                        "{}/other-translations/{}/{}/{}",
                        ctx.backend_base_url,
                        route_parts.as_ref().unwrap().0,
                        route_parts.as_ref().unwrap().1,
                        route_parts.as_ref().unwrap().2.as_ref().unwrap(),
                    )
                };
                let response = Request::get(url.as_str()).send().await;
                if let Ok(ok) = response {
                    let lala = ok.json::<AlternativeBookTranslations>().await;
                    if let Ok(c) = lala {
                        alt_trans.set(Some(c));
                    } else {
                        alt_trans.set(None);
                    }
                } else {
                    error_copy.set(Some("net_error"));
                };
                is_loading_copy2.set(false);
            });
        }
    });

    alternative_translations
}
