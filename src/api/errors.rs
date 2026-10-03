/// Enum mapping every possible error that can happen in this lib, with a nice enum.
#[derive(Debug)]
pub enum LibnotifyError {
    /// [libnotify init](https://gnome.pages.gitlab.gnome.org/libnotify/func.init.html) failed
    InitFail,
    /// `&str` contains a null terminator. Read more at [std documentation](https://doc.rust-lang.org/std/ffi/struct.NulError.html)
    NulError(std::ffi::NulError),
}

impl std::fmt::Display for LibnotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitFail => {
                write!(f, "Libnotify failed to init")
            }

            Self::NulError(_) => {
                write!(f, "Argument contains a null terminator")
            }
        }
    }
}

impl std::error::Error for LibnotifyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InitFail => None,
            Self::NulError(e) => Some(e),
        }
    }
}

impl From<std::ffi::NulError> for LibnotifyError {
    fn from(value: std::ffi::NulError) -> Self {
        Self::NulError(value)
    }
}
