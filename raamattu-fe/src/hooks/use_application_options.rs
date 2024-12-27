use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use yew::{hook, use_context, use_effect_with, use_state, UseStateHandle};

use crate::context::ApplicationOptions;

#[hook]
pub fn use_application_options(new_value: Option<ApplicationOptions>) -> ApplicationOptions {
    // Application babysitted here, and later returned.
    let app_opts = use_context::<ApplicationOptions>();
    let app_opts_state = use_state(|| app_opts.unwrap());

    // Initial load from the disk.
    let app_opts_state_ = app_opts_state.clone();
    use_effect_with((), move |_| {
        let options: Result<ApplicationOptions, StorageError> = LocalStorage::get("app_opts");

        match options {
            Ok(opts) => {
                app_opts_state_.set(opts);
            }
            Err(_err) => {}
        }
    });

    // In case new_value is Some()
    let app_opts_state_ = app_opts_state.clone();
    use_effect_with((new_value, app_opts_state_), move |(new_value, opts)| {
        if let Some(new_value) = new_value {
            let _result = LocalStorage::set("app_opts", &new_value);
            opts.set((*new_value).clone());
        }
    });

    (*app_opts_state).clone()
}
