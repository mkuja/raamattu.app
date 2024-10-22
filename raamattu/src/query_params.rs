use serde::Deserialize;

#[derive(Deserialize)]
pub struct FrontPageQueryParams {
    /// Translation used. It is the **short_name** field. `None` will default to *kr38*.
    pub tr: Option<String>,
}