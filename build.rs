fn main() {
    tauri_plugin::Builder::new(&["request_review"])
        .android_path("android")
        .build();
}
