use std::env::consts;
use std::ffi::CString;
use std::path::{Path, PathBuf};

pub mod args;
pub mod error;
mod plugin_loader;

pub fn process_image(args: args::Args) -> Result<(), error::Error> {
    let mut img = decode_image(&args.input)?;
    let data: *mut u8 = img.as_mut_ptr();

    let params_cstring = get_params(&args.params)?;
    let params_ptr = params_cstring.as_ptr();

    let plugin = open_plugin(&args.plugin, &args.plugin_path)?;
    let plugin_interface = plugin.processor()?;
    (plugin_interface.process_image)(img.width(), img.height(), data, params_ptr);

    let output = Path::new(&args.output);
    if output.extension().and_then(|e| e.to_str()) != Some("png") {
        return Err(error::Error::Io("output path must have .png extension".to_string()));
    }
    img.save(output)
        .map_err(|e| error::Error::ImageError(format!("error saving image: {}", e)))?;

    Ok(())
}

fn decode_image(input: &str) -> Result<image::RgbaImage, error::Error> {
    let img = image::ImageReader::open(input)
        .map_err(|e| error::Error::Io(format!("file {} open error: {}", input, e)))?
        .decode()
        .map_err(|e| error::Error::ImageError(format!("decode error: {}", e)))?
        .to_rgba8();

    Ok(img)
}

fn open_plugin(plugin: &str, path: &str) -> Result<plugin_loader::Plugin, error::Error> {
    let plug_path = PathBuf::from(path).join(format!(
        "{}{}.{}",
        consts::DLL_PREFIX,
        plugin,
        consts::DLL_EXTENSION
    ));
    let plugin = plugin_loader::Plugin::new(plug_path.to_str().ok_or_else(|| {
        error::Error::Io(format!("invalid plugin path: {}", plug_path.display()))
    })?)?;
    Ok(plugin)
}

fn get_params(params: &str) -> Result<CString, error::Error> {
    let params = std::fs::read_to_string(params)
        .map_err(|e| error::Error::Io(format!("error reading file {}: {}", params, e)))?;
    let params_cstring = CString::new(params.as_str()).map_err(|e| {
        error::Error::Io(format!("error converting params to CString: {}", e))
    })?;
    Ok(params_cstring)
}
