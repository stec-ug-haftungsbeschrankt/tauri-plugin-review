use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Review;
#[cfg(mobile)]
use mobile::Review;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the review APIs.
pub trait ReviewExt<R: Runtime> {
  fn review(&self) -> &Review<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ReviewExt<R> for T {
  fn review(&self) -> &Review<R> {
    self.state::<Review<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("review")
    .invoke_handler(tauri::generate_handler![commands::request_review])
    .setup(|app, api| {
      #[cfg(mobile)]
      let review = mobile::init(app, api)?;
      #[cfg(desktop)]
      let review = desktop::init(app, api)?;
      app.manage(review);
      Ok(())
    })
    .build()
}
