#![cfg(target_os = "android")]

mod logic;

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    slint::android::init(app);
    logic::run_app()    
}
