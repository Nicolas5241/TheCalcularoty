#![cfg(target_os = "android")]

mod android;

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    slint::android::init(app.clone())?;

    android::run_app(app.asset_manager())
}
