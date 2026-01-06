use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ReviewExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.review().ping(payload)
}
