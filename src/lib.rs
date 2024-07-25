#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use ext_php_rs::binary::*;
use lepton_jpeg::*;
use lepton_jpeg::enabled_features::EnabledFeatures;

#[php_function]
pub fn convert_lepton_to_jpeg(lepton: Binary<u8>, max_threads: Option<u32>) -> Option<Binary<u8>> {
    let input_lepton: Vec<u8> = Vec::from(lepton);
    let input_size: usize = input_lepton.len();

    let mut reader = std::io::Cursor::new(input_lepton);

    let mut output_buffer: Vec<u8> = Vec::with_capacity(input_size * 3);

    match decode_lepton(
        &mut reader,
        &mut output_buffer,
        max_threads.unwrap_or(4) as usize,
    ) {
        Ok(_) => {
            Some(Binary::from(output_buffer))
        },
        Err(_) => None,
    }
}

#[php_function]
pub fn convert_jpeg_to_lepton(lepton: Binary<u8>, max_threads: Option<u32>) -> Option<Binary<u8>> {
    let input_jpeg: Vec<u8> = Vec::from(lepton);
    let input_size: usize = input_jpeg.len();
    let mut reader = std::io::Cursor::new(input_jpeg);

    let mut output_buffer: Vec<u8> = Vec::with_capacity(input_size * 2);
    let mut writer = std::io::Cursor::new(&mut output_buffer);

    let enabled_features = EnabledFeatures {
        progressive: true,
        max_jpeg_width: 16386,
        max_jpeg_height: 16386,
    };

    match encode_lepton(
        &mut reader,
        &mut writer,
        max_threads.unwrap_or(4) as usize,
        &enabled_features,
    ) {
        Ok(_) => {
            Some(Binary::from(output_buffer))
        },
        Err(_) => None,
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
