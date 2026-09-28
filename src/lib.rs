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
    ApiReqest,
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
