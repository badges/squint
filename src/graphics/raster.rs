use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;

use super::{SvgProcessor, INVALID_SVG};
use crate::badge::BadgeStyle;

// Module below used for importing these to support mocking
// for tests
use self::librsvg_imports::{CairoRenderer, Loader};
use cairo::{Context, ImageSurface, Rectangle};
use gio::{MemoryInputStream, NONE_CANCELLABLE, NONE_FILE};
use glib::Bytes;
use librsvg::IntrinsicDimensions;

#[derive(Debug, Eq, PartialEq)]
pub enum SvgToPngConversionError {
    ImageContextCreationFailure,
    ImageSurfaceCreationFailure,
    SvgBytesProcessingFailure,
    SvgHandleCreationFailure,
    SvgRenderingError,
    PngCreationError,
}

// Returns tuple with (width, height)
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

fn get_bytes_stream<S: SvgProcessor>(
    svg_bytes: Option<Vec<u8>>,
    badge_style: &BadgeStyle,
    svg_processor: &S,
) -> Result<Bytes, SvgToPngConversionError> {
    match svg_bytes {
        Some(b) => Ok(Bytes::from_owned(
            svg_processor
                .prepare_svg_for_png_conversion(b, badge_style)
                .map_err(|_| SvgToPngConversionError::SvgBytesProcessingFailure)?,
        )),
        None => Ok(Bytes::from_static(INVALID_SVG)),
    }
}

pub(super) fn convert_svg_to_png<S: SvgProcessor>(
    svg_bytes: Option<Vec<u8>>,
    badge_style: BadgeStyle,
    svg_processor: S,
) -> Result<Vec<u8>, SvgToPngConversionError> {
    use SvgToPngConversionError::*;
    let stream =
        MemoryInputStream::from_bytes(&get_bytes_stream(svg_bytes, &badge_style, &svg_processor)?);
    let handle = Loader::new()
        .read_stream(&stream, NONE_FILE, NONE_CANCELLABLE)
        .map_err(|_| SvgHandleCreationFailure)?;

    let renderer = CairoRenderer::new(&handle);
    let (width, height) = get_dimensions(&renderer);
    let surface = ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32)
        .map_err(|_| ImageSurfaceCreationFailure)?;

    let context = Context::new(&surface).map_err(|_| ImageContextCreationFailure)?;
    let cr = Rectangle {
        x: 0.0,
        y: 0.0,
        width,
        height,
    };
    renderer
        .render_document(&context, &cr)
        .map_err(|_| SvgRenderingError)?;

    let mut png_stream = Vec::new();
    surface
        .write_to_png(&mut png_stream)
        .map_err(|_| PngCreationError)?;

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
                    pub fn new() -> Self;
                    pub fn read_stream<S, F, P>(
                        &self,
                        stream: &S,
                        base_file: Option<&'static F>,
                        cancellable: Option<&'static P>
                    ) -> Result<SvgHandle, LoadingError>
                    where
                        S: glib::IsA<gio::InputStream>,
                        F: glib::IsA<gio::File>,
                        P: glib::IsA<gio::Cancellable>;
                }
            }
            pub(super) use MockLoader as Loader;
            mock! {
                pub CairoRenderer {
                    pub fn new(handle: &librsvg::SvgHandle) -> Self;
                    pub fn intrinsic_dimensions(&self) -> librsvg::IntrinsicDimensions;
                    pub fn render_document(
                        &self,
                        ctx: &Context,
                        cr: &Rectangle
                    ) -> Result<(), librsvg::RenderingError>;
                    pub fn with_dpi(&self, x: f64, y: f64) -> Self;
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
