
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(target_os = "android")]
use tauri::plugin::PluginHandle;

#[tauri::command]
async fn request_review<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        app.plugin_manager()
            .android()
            .run_then_drop(|plugin_handle| {
                plugin_handle
                    .invoke("requestReview", serde_json::Value::Null)
                    .map_err(|e| e.to_string())
            })
            .await?;
        Ok(())
    }
    
    #[cfg(not(target_os = "android"))]
    {
        Err("In-app review is only supported on Android".to_string())
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("review")
        .invoke_handler(tauri::generate_handler![request_review])
        .build()
}
