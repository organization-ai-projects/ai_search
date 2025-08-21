//! Module image modulaire : décodage réel avec la crate image
use crate::orchestrator::{DataPacket, Module};
use image::GenericImageView;

pub struct ImageModule;

impl Module for ImageModule {
    fn name(&self) -> &str {
        "ImageModule"
    }
    fn modality(&self) -> &str {
        "image"
    }
    fn process(&self, input: &DataPacket) -> DataPacket {
        if let Some(img) = (input.payload.as_ref() as &dyn std::any::Any).downcast_ref::<Vec<u8>>()
        {
            let mut meta = input.meta.clone();
            // Détection du format et décodage
            let format = image::guess_format(img)
                .map(|f| format!("{:?}", f))
                .unwrap_or("Unknown".to_string());
            let decoded = image::load_from_memory(img);
            match decoded {
                Ok(imgbuf) => {
                    let (w, h) = imgbuf.dimensions();
                    let color = format!("{:?}", imgbuf.color());
                    meta.insert("image_format".into(), format);
                    meta.insert("image_size".into(), img.len().to_string());
                    meta.insert("width".into(), w.to_string());
                    meta.insert("height".into(), h.to_string());
                    meta.insert("color_type".into(), color);
                    // On expose les pixels sous forme de Vec<u8> (flatten)
                    let pixels: Vec<u8> = imgbuf.as_bytes().to_vec();
                    DataPacket {
                        modality: "image".into(),
                        payload: Box::new(pixels),
                        meta,
                    }
                }
                Err(e) => {
                    meta.insert("image_error".into(), format!("Erreur décodage: {}", e));
                    DataPacket {
                        modality: "image".into(),
                        payload: Box::new(img.clone()),
                        meta,
                    }
                }
            }
        } else {
            input.clone()
        }
    }
}
