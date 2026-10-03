use crate::{
    api::{
        errors::LibnotifyError,
        notification::{HintValue, Notification, Timeout, Urgency},
    },
    wrappers::context::LibnotifyContext,
};

/// Higher level abstraction of `Notification`. Allow for builder pattern notification construction.
/// Requires `LibnotifyContext`
pub struct NotificationBuilder<'a> {
    notification: Notification,
    _context: &'a LibnotifyContext,
}

impl<'a> NotificationBuilder<'a> {
    /// Create new NotificationBuilder, with given context.
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

    /// Sets app name for this specific notification.
    pub fn set_app_name(mut self, app_name: &str) -> Result<Self, LibnotifyError> {
        self.notification.set_app_name(app_name)?;
        Ok(self)
    }

    /// Sets app icon fot this specific notification.
    pub fn set_app_icon(mut self, app_icon: &str) -> Result<Self, LibnotifyError> {
        self.notification.set_app_icon(app_icon)?;
        Ok(self)
    }

    /// Sets category for this specific notification
    pub fn set_category(mut self, category: &str) -> Result<Self, LibnotifyError> {
        self.notification.set_category(category)?;
        Ok(self)
    }

    /// Set hint for this specific notification
    pub fn set_hint(
        mut self,
        key: &str,
        value: HintValue,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        self.notification.set_hint(key, value)?;
        Ok(self)
    }

    /// Set timeout until close
    pub fn set_timeout(mut self, timeout: Timeout) -> Self {
        self.notification.set_timeout(timeout);
        self
    }

    /// Set notification urgency level
    pub fn set_urgency(mut self, urgency: Urgency) -> Self {
        self.notification.set_urgency(urgency);
        self
    }

    /// Add callback
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

    /// Show notification
    ///
    /// Uses `&self` borrow, so you can use this object later, after showing.
    pub fn show(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.notification.show()?;
        Ok(())
    }

    /// Close notification synchronously.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.notification.close()?;
        Ok(())
    }
}
