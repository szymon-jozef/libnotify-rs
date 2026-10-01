use crate::{
    api::notification::{HintValue, Notification, Timeout, Urgency},
    wrappers::context::LibnotifyContext,
};

pub struct NotificationBuilder<'a> {
    notification: Notification,
    _context: &'a LibnotifyContext,
}

impl<'a> NotificationBuilder<'a> {
    pub fn new<T, Y>(
        summary: &str,
        body: T,
        icon: Y,
        context: &'a LibnotifyContext,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        T: Into<Option<&'a str>>,
        Y: Into<Option<&'a str>>,
    {
        let notification = Notification::new(summary, body, icon)?;

        Ok(Self {
            notification,
            _context: context,
        })
    }

    pub fn set_app_name(mut self, app_name: &str) -> Result<Self, std::ffi::NulError> {
        self.notification.set_app_name(app_name)?;
        Ok(self)
    }

    pub fn set_app_icon(mut self, app_icon: &str) -> Result<Self, std::ffi::NulError> {
        self.notification.set_app_icon(app_icon)?;
        Ok(self)
    }
    pub fn set_category(mut self, category: &str) -> Result<Self, std::ffi::NulError> {
        self.notification.set_category(category)?;
        Ok(self)
    }

    pub fn set_hint(
        mut self,
        key: &str,
        value: HintValue,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        self.notification.set_hint(key, value)?;
        Ok(self)
    }

    pub fn set_timeout(mut self, timeout: Timeout) -> Self {
        self.notification.set_timeout(timeout);
        self
    }

    pub fn set_urgency(mut self, urgency: Urgency) -> Self {
        self.notification.set_urgency(urgency);
        self
    }

    pub fn add_action<F>(
        mut self,
        action: &str,
        label: &str,
        callback: F,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        F: Fn(&str) + 'static,
    {
        self.notification.add_action(action, label, callback)?;
        Ok(self)
    }

    pub fn show(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.notification.show()?;
        Ok(())
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.notification.close()?;
        Ok(())
    }
}
