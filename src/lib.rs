#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub enum Urgency {
    Low,
    Normal,
    Critical,
}

pub enum ClosedReason {
    Unset,
    Expired,
    ApiRequest,
    Undefined,
}

pub struct Notification {
    inner: *mut NotifyNotification,
}

impl<'a> Notification {
    pub fn new<T, Y>(summary: &str, body: T, icon: Y) -> Result<Self, Box<dyn std::error::Error>>
    where
        T: Into<Option<&'a str>>,
        Y: Into<Option<&'a str>>,
    {
        let summary = std::ffi::CString::new(summary)?;

        let body = body
            .into()
            .map(|body| std::ffi::CString::new(body))
            .transpose()?;

        let icon = icon
            .into()
            .map(|icon| std::ffi::CString::new(icon))
            .transpose()?;

        let inner = unsafe {
            notify_notification_new(
                summary.as_ptr(),
                body.as_ref().map_or(std::ptr::null(), |body| body.as_ptr()),
                icon.as_ref().map_or(std::ptr::null(), |icon| icon.as_ptr()),
            )
        };

        if inner.is_null() {
            return Err("notify_notification_new returned null".into());
        }

        Ok(Self { inner })
    }

    pub fn add_action<F>(
        &mut self,
        action: &str,
        label: &str,
        callback: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(&str) + 'static,
    {
        let action = std::ffi::CString::new(action)?;
        let label = std::ffi::CString::new(label)?;

        let callback: Box<F> = Box::new(callback);
        let user_data = Box::into_raw(callback) as *mut std::ffi::c_void;

        let free_func = drop_box::<F>;

        unsafe {
            notify_notification_add_action(
                self.inner,
                action.as_ptr(),
                label.as_ptr(),
                Some(action_trampoline::<F>),
                user_data,
                Some(free_func),
            );
        }

        Ok(())
    }
}

unsafe extern "C" fn action_trampoline<F>(
    _notification: *mut NotifyNotification,
    action: *mut ::std::os::raw::c_char,
    user_data: gpointer,
) where
    F: Fn(&str) + 'static,
{
    let callback = unsafe { &*(user_data as *const F) };

    if action.is_null() {
        return;
    }

    let action = unsafe { std::ffi::CStr::from_ptr(action).to_string_lossy() };
    callback(&action);
}

unsafe extern "C" fn drop_box<F>(data: gpointer) {
    unsafe { drop(Box::from_raw(data as *mut F)) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        assert!(
            !Notification::new("Test", "Test", "Test")
                .unwrap()
                .inner
                .is_null()
        );
        assert!(
            !Notification::new("Test", None, None)
                .unwrap()
                .inner
                .is_null()
        );
        assert!(
            !Notification::new("Test", None, "Test")
                .unwrap()
                .inner
                .is_null()
        );
        assert!(
            !Notification::new("Test", "Test", None)
                .unwrap()
                .inner
                .is_null()
        );
    }
}
