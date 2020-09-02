use super::INVALID_SVG;
use crate::badge::BadgeStyle;

use cairo::{Context, ImageSurface, Rectangle};
use gio::MemoryInputStream;
use glib::Bytes;
use librsvg::IntrinsicDimensions;
use librsvg_imports::{CairoRenderer, Loader};

pub enum SvgToPngConversion {
    Success(Vec<u8>),
    Failure,
}

// (width, height)
fn get_dimensions(renderer: &CairoRenderer) -> (f64, f64) {
    let IntrinsicDimensions {
        width: width_dim,
        height: height_dim,
        ..
    } = renderer.intrinsic_dimensions();
    let width = width_dim.map_or(0f64, |w| w.length);
    let height = height_dim.map_or(0f64, |h| h.length);

    (width, height)
}

pub fn convert_svg_to_png(
    svg_bytes: Option<Vec<u8>>,
    _badge_style: BadgeStyle,
) -> SvgToPngConversion {
    let bytes_stream = match svg_bytes {
        Some(b) => Bytes::from_owned(b),
        None => Bytes::from_static(INVALID_SVG),
    };

    let stream = MemoryInputStream::new_from_bytes(&bytes_stream);
    let handle =
        match Loader::new().read_stream(&stream, None::<&gio::File>, None::<&gio::Cancellable>) {
            Ok(h) => h,
            Err(_) => return SvgToPngConversion::Failure,
        };

    let renderer = CairoRenderer::new(&handle);
    let (width, height) = get_dimensions(&renderer);
    let surface = match ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("surface creation error: {}", e);
            return SvgToPngConversion::Failure;
        }
    };

    let context = Context::new(&surface);
    let cr = Rectangle {
        x: 0.0,
        y: 0.0,
        width,
        height,
    };

    if let Err(e) = renderer.render_document(&context, &cr) {
        eprintln!("rendering error: {}", e);
        return SvgToPngConversion::Failure;
    }

    let mut png_stream = Vec::new();
    if let Err(e) = surface.write_to_png(&mut png_stream) {
        eprintln!("png conversion error: {}", e);
        return SvgToPngConversion::Failure;
    }

    SvgToPngConversion::Success(png_stream)
}

mod librsvg_imports {
    // Configure imports to support mocks during testing
    // given the struct parameters in many of the Cairo functions.
    cfg_if::cfg_if! {
        if #[cfg(test)] {
            use super::{Context, Rectangle};
            use mockall::*;
            use librsvg::{LoadingError, SvgHandle};
            mock! {
                pub Loader {
                    fn new() -> Self;
                    fn read_stream<S: glib::IsA<gio::InputStream>, F: glib::IsA<gio::File>, P: glib::IsA<gio::Cancellable>>(&self, stream: &S, base_file: Option<&'static F>, cancellable: Option<&'static P>) -> Result<SvgHandle, LoadingError>;
                }
            }
            pub(super) use MockLoader as Loader;
            mock! {
                pub CairoRenderer {
                    fn new(handle: &librsvg::SvgHandle) -> Self;
                    fn intrinsic_dimensions(&self) -> librsvg::IntrinsicDimensions;
                    fn render_document(&self, ctx: &Context, cr: &Rectangle) -> Result<(), librsvg::RenderingError>;
                    fn with_dpi(&self, x: f64, y: f64) -> Self;
                }
            }
            pub(super) use MockCairoRenderer as CairoRenderer;
        } else {
            pub(super) use librsvg::{CairoRenderer, Loader};
        }
    }
}

#[cfg(test)]
#[path = "raster_test.rs"]
mod tests;
