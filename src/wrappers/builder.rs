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
    ///
    /// # Errors
    ///
    /// This function can return LibnotifyError::{NewNotificationError, NulError}
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
    ///
    /// # Errors
    /// This function can return LibnotifyError::{NulError}
    pub fn set_app_name<T: Into<Option<&'a str>>>(
        mut self,
        app_name: T,
    ) -> Result<Self, LibnotifyError> {
        if let Some(app_name) = app_name.into() {
            self.notification.set_app_name(app_name)?;
        }

        Ok(self)
    }

    /// Sets app icon fot this specific notification.
    ///
    /// # Errors
    /// This function can return LibnotifyError::{NulError}
    pub fn set_app_icon<T: Into<Option<&'a str>>>(
        mut self,
        app_icon: T,
    ) -> Result<Self, LibnotifyError> {
        if let Some(app_icon) = app_icon.into() {
            self.notification.set_app_icon(app_icon)?;
        }

        Ok(self)
    }

    /// Sets category for this specific notification
    ///
    /// # Errors
    /// This function can return LibnotifyError::{NulError}
    pub fn set_category<T: Into<Option<&'a str>>>(
        mut self,
        category: T,
    ) -> Result<Self, LibnotifyError> {
        if let Some(category) = category.into() {
            self.notification.set_category(category)?;
        }

        Ok(self)
    }

    /// Set hint for this specific notification
    /// # Errors
    /// This function can return LibnotifyError::{NulError, AllocationError}
    pub fn set_hint(mut self, key: &str, value: HintValue) -> Result<Self, LibnotifyError> {
        self.notification.set_hint(key, value)?;
        Ok(self)
    }

    /// Set timeout until close
    pub fn set_timeout<T: Into<Option<Timeout>>>(mut self, timeout: T) -> Self {
        if let Some(timeout) = timeout.into() {
            self.notification.set_timeout(timeout);
        }

        self
    }

    /// Set notification urgency level
    pub fn set_urgency<T: Into<Option<Urgency>>>(mut self, urgency: T) -> Self {
        if let Some(urgency) = urgency.into() {
            self.notification.set_urgency(urgency);
        }

        self
    }

    /// Add callback
    ///
    /// # Errors
    /// This function can return LibnotifyError::NulError
    pub fn add_action<F>(
        mut self,
        action: &str,
        label: &str,
        callback: F,
    ) -> Result<Self, LibnotifyError>
    where
        F: Fn(&str) + 'static,
    {
        self.notification.add_action(action, label, callback)?;
        Ok(self)
    }

    /// Show notification
    ///
    /// Uses `&self` borrow, so you can use this object later, after showing.
    ///
    /// # Errors
    /// This function can return LibnotifyError::{GerrorError, AllocationError}
    pub fn show(&self) -> Result<(), LibnotifyError> {
        self.notification.show()?;
        Ok(())
    }

    /// Close notification synchronously.
    ///
    /// Uses `&self` borrow, so you can use this object later, after closing.
    ///
    /// # Errors
    /// This function can return LibnotifyError::{GerrorError, AllocationError}
    pub fn close(&self) -> Result<(), LibnotifyError> {
        self.notification.close()?;
        Ok(())
    }
}
