/// Enum mapping every possible error that can happen in this lib, with a nice enum.
#[derive(Debug)]
pub enum LibnotifyError {
    /// [Libnotify init](https://gnome.pages.gitlab.gnome.org/libnotify/func.init.html) failed
    InitError,
    /// `&str` contains a null terminator. Read more at [std documentation](https://doc.rust-lang.org/std/ffi/struct.NulError.html)
    NulError(std::ffi::NulError),
    /// [Libnotify notification constructor](https://gnome.pages.gitlab.gnome.org/libnotify/ctor.Notification.new.html) failed allocating
    NewNotificationError,
    /// Custom libnotify error with explanation.
    GerrorError(String),
    /// Data failed allocating. Could indicate that system doesn't have enough memory
    AllocationError,
}

impl std::fmt::Display for LibnotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibnotifyError::InitError => {
                write!(f, "Libnotify failed to init")
            }

            LibnotifyError::NulError(_) => {
                write!(f, "Argument contains a null terminator")
            }

            LibnotifyError::NewNotificationError => {
                write!(f, "Notification constructor failed")
            }

            LibnotifyError::GerrorError(e) => {
                write!(f, "Libnotify returned an error: {}", e)
            }

            LibnotifyError::AllocationError => {
                write!(f, "Libnotify failed allocating memory")
            }
        }
    }
}

impl std::error::Error for LibnotifyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LibnotifyError::InitError => None,
            LibnotifyError::NulError(nul_error) => Some(nul_error),
            LibnotifyError::NewNotificationError => None,
            LibnotifyError::GerrorError(_) => None,
            LibnotifyError::AllocationError => None,
        }
    }
}

impl From<std::ffi::NulError> for LibnotifyError {
    fn from(value: std::ffi::NulError) -> Self {
        Self::NulError(value)
    }
}
