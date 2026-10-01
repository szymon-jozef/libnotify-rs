use crate::api::functions::{init, is_initted, set_app_icon, set_app_name, uninit};

pub struct LibnotifyContext {}

static IS_CONTEXT_CREATED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

impl LibnotifyContext {
    pub fn new(app_name: &str) -> Result<LibnotifyContext, Box<dyn std::error::Error>> {
        if IS_CONTEXT_CREATED.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return Err("You can create only one context at a time".into());
        }

        if !is_initted() {
            init(app_name)?;
        }

        Ok(Self {})
    }

    pub fn set_new_name(&self, app_name: &str) -> Result<(), std::ffi::NulError> {
        set_app_name(app_name)?;
        Ok(())
    }

    pub fn set_app_icon(&self, app_icon: &str) -> Result<(), std::ffi::NulError> {
        set_app_icon(app_icon)?;
        Ok(())
    }
}

impl Drop for LibnotifyContext {
    fn drop(&mut self) {
        if is_initted() {
            uninit();

            IS_CONTEXT_CREATED.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
