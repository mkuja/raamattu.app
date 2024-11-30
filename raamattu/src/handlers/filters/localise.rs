use rust_i18n::*;

pub fn localise(s: &str, lang:&str) -> ::askama::Result<String> {
    let translation=t!(s,locale=lang);
    Ok(translation.to_string())
}
