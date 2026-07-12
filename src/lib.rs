mod api;
mod error;
pub use error::TIError;

pub struct TrayItem(api::TrayItemImpl);

#[derive(Clone)]
pub enum IconSource {
    Resource(&'static str),
    #[cfg(target_os = "windows")]
    RawIcon(windows_sys::Win32::UI::WindowsAndMessaging::HICON),
    #[cfg(any(target_os = "macos", all(target_os = "linux", feature = "ksni")))]
    Data {
        height: i32,
        width: i32,
        data: Vec<u8>,
    },
}

impl IconSource {
    pub fn as_str(&self) -> &str {
        match self {
            IconSource::Resource(res) => res,
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
    }
}

impl TrayItem {
    pub fn new(title: &str, icon: IconSource) -> Result<Self, TIError> {
        Ok(Self(api::TrayItemImpl::new(title, icon)?))
    }

    pub fn set_icon(&mut self, icon: IconSource) -> Result<(), TIError> {
        self.0.set_icon(icon)
    }

    pub fn add_label(&mut self, label: &str) -> Result<(), TIError> {
        self.0.add_label(label)
    }

    pub fn add_menu_item<F>(&mut self, label: &str, cb: F) -> Result<(), TIError>
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.0.add_menu_item(label, cb)
    }

    pub fn inner_mut(&mut self) -> &mut api::TrayItemImpl {
        &mut self.0
    }

    // TODO
    /*pub fn set_leftclick_callback<F>(&mut self, cb: F)
    where
        F: Fn() + Send + 'static,
    {
        self.0.set_leftclick_callback(cb);
    }
    pub fn set_rightclick_callback<F>(&mut self, cb: F)
    where
        F: Fn() + Send + 'static,
    {
        self.0.set_rightclick_callback(cb);
    }
    pub fn set_doubleclick_callback<F>(&mut self, cb: F)
    where
        F: Fn() + Send + 'static,
    {
        self.0.set_doubleclick_callback(cb);
    }
    pub fn set_middleclick_callback<F>(&mut self, cb: F)
    where
        F: Fn() + Send + 'static,
    {
        self.0.set_middleclick_callback(cb);
    }*/
}
