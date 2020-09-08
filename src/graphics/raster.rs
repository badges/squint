use super::{LetterSpacer, INVALID_SVG};
use crate::badge::BadgeStyle;

// Module below used for importing these to support mocking
// for tests
use self::librsvg_imports::{CairoRenderer, Loader};
use cairo::{Context, ImageSurface, Rectangle};
use gio::MemoryInputStream;
use glib::Bytes;
use librsvg::IntrinsicDimensions;

pub enum SvgToPngConversionError {
    ImageSurfaceCreationFailure,
    SvgHandleCreationFailure,
    SvgRenderingError,
    PngCreationError,
}

impl From<librsvg::LoadingError> for SvgToPngConversionError {
    fn from(_e: librsvg::LoadingError) -> SvgToPngConversionError {
        SvgToPngConversionError::SvgHandleCreationFailure
    }
}

impl From<cairo::Status> for SvgToPngConversionError {
    fn from(_e: cairo::Status) -> SvgToPngConversionError {
        SvgToPngConversionError::ImageSurfaceCreationFailure
    }
}

impl From<librsvg::RenderingError> for SvgToPngConversionError {
    fn from(_e: librsvg::RenderingError) -> SvgToPngConversionError {
        SvgToPngConversionError::SvgRenderingError
    }
}

impl From<cairo::IoError> for SvgToPngConversionError {
    fn from(_e: cairo::IoError) -> SvgToPngConversionError {
        SvgToPngConversionError::PngCreationError
    }
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

pub(super) fn convert_svg_to_png<L: LetterSpacer>(
    svg_bytes: Option<Vec<u8>>,
    _badge_style: BadgeStyle,
    _letter_spacer: L,
) -> Result<Vec<u8>, SvgToPngConversionError> {
    let bytes_stream = match svg_bytes {
        Some(b) => Bytes::from_owned(b),
        None => Bytes::from_static(INVALID_SVG),
    };
    let stream = MemoryInputStream::new_from_bytes(&bytes_stream);
    let handle =
        Loader::new().read_stream(&stream, None::<&gio::File>, None::<&gio::Cancellable>)?;

    let renderer = CairoRenderer::new(&handle);
    let (width, height) = get_dimensions(&renderer);
    let surface = ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32)?;

    let context = Context::new(&surface);
    let cr = Rectangle {
        x: 0.0,
        y: 0.0,
        width,
        height,
    };
    renderer.render_document(&context, &cr)?;

    let mut png_stream = Vec::new();
    surface.write_to_png(&mut png_stream)?;

    Ok(png_stream)
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
