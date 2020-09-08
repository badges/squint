mod raster;
mod svg;
mod text;

use crate::badge::BadgeStyle;
pub(crate) use raster::SvgToPngConversionError;
use svg::INVALID_SVG;
pub(crate) use svg::INVALID_SVG_BADGE;
use text::{LetterSpacer, SimpleCairoDockerLetterSpacer};

pub fn convert_svg_to_png(
    svg_bytes: Option<Vec<u8>>,
    _badge_style: BadgeStyle,
) -> Result<Vec<u8>, SvgToPngConversionError> {
    raster::convert_svg_to_png(svg_bytes, _badge_style, SimpleCairoDockerLetterSpacer {})
}
