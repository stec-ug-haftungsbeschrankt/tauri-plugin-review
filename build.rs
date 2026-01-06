fn main() {
    tauri_plugin::Builder::new(&["android"])
        .android_path("android")
        .build();
}
