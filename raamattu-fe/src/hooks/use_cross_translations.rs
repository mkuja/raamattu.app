use gloo_net::http::Request;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};
use yew_router::hooks::use_route;

use crate::{context::ApplicationOptions, Route};

#[derive(Deserialize, Clone, PartialEq)]
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

impl Default for AlternativeBookTranslations {
    fn default() -> Self {
        Self {
            book_id: 0,
            book_color: "#fff".to_string(),
            short_name: "unknown".to_string(),
            full_name: "unknown".to_string(),
            language: "en".to_string(),
            translation: "unknown".to_string(),
            translation_description: "none".to_string(),
            matching: None,
        }
    }
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
pub fn use_cross_translations(
    current_choice_of_book: &str,
) -> (
    UseStateHandle<Option<AlternativeBookTranslations>>,
    UseStateHandle<bool>,
    UseStateHandle<Option<&'static str>>,
) {
    // TODO: Initial book from Route, and any changes from parameter

    // The return value
    let alternative_translations: UseStateHandle<Option<AlternativeBookTranslations>> =
        use_state(|| None);
    let is_loading = use_state(|| true);
    let error = use_state(|| None);
    let route: Option<Route> = use_route();

    // Initial route parts come from the url.
    let route_clone = route.clone();
    let route_parts = use_state(|| match route_clone.unwrap() {
        Route::Chapters { translation, book } => Some((translation, book, None)),
        Route::Chapter {
            translation,
            book,
            chapter,
        } => Some((translation, book, Some(chapter))),
        _ => None,
    });

    // And any changes to path are reflected from parameters.
    let route_parts_clone = route_parts.clone();
    let current_choice_of_book_s = current_choice_of_book.to_string();
    let route_clone = route.as_ref().unwrap().clone();
    use_effect_with(current_choice_of_book_s, move |b| {
        route_parts_clone.set(match route_clone {
            Route::Chapters { translation, book } => {
                Some((translation.clone(), b.to_string(), None))
            }
            Route::Chapter {
                translation,
                book,
                chapter,
            } => Some((translation.clone(), b.to_string(), Some(chapter.clone()))),
            _ => None,
        });
    });

    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();

    let is_loading_copy = is_loading.clone();
    let error_copy = error.clone();
    let alt_trans = alternative_translations.clone();
    let route_parts_clone = route_parts.clone();
    use_effect_with((), move |_| {
        let is_loading_copy2 = is_loading_copy.clone();
        is_loading_copy.set(true);
        let ctx = ctx.clone();
        let route_parts_clone = route_parts_clone.clone();
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

    (alternative_translations, is_loading, error)
}
