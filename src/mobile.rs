use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_review);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Review<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("de.stecug.tauri.plugin.review", "ReviewPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_review)?;
  Ok(Review(handle))
}

/// Access to the review APIs.
pub struct Review<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Review<R> {
  pub fn request_review(&self) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("requestReview", ())
      .map_err(Into::into)
  }
}
