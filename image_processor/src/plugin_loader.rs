use libloading::{Library, Symbol};
use crate::error;

pub struct PluginInterface<'a> {
    pub process_image: Symbol<
        'a,
        extern "C" fn(width: u32, height: u32, rgba_data: *mut u8, params: *const libc::c_char),
    >,
}

pub struct Plugin {
    plugin: Library,
}

impl Plugin {
    pub fn new(filename: &str) -> Result<Self, error::Error> {
        Ok(Plugin {
            // SAFETY: доверяем что загружаемая библиотека безопасна
            // и не содержит кода вызывающего UB при инициализации
            plugin: unsafe { Library::new(filename).map_err(|e| {
                error::Error::LibError(format!("unable to load {}: {}", filename, e))
            })? },
        })
    }

    pub fn processor(&self) -> Result<PluginInterface<'_>, error::Error> {
        Ok(PluginInterface {
            // SAFETY: вызывающая сторона гарантирует сигнатуру функции
            // void process_image(
            //      uint32_t width,
            //      uint32_t height,
            //      uint8_t* rgba_data,
            //      const char* params
            // );
            process_image: unsafe { self.plugin.get("process_image").map_err(|e| {
                error::Error::LibError(format!("unable to find process image: {}", e))
            })? },
        })
    }
}
