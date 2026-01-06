use tauri::{AppHandle, command, Runtime};

use crate::Result;
use crate::ReviewExt;

#[command]
pub(crate) async fn request_review<R: Runtime>(
    app: AppHandle<R>,
) -> Result<()> {
    app.review().request_review()
}
