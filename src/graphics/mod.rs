mod raster;
mod svg;

use crate::badge::BadgeStyle;
pub(crate) use raster::SvgToPngConversionError;
pub(crate) use svg::INVALID_SVG_BADGE;
use svg::{LetterSpacingSvgProcessor, SvgProcessor, INVALID_SVG};

#[cfg(test)]
mod test_data;

pub fn convert_svg_to_png(
    svg_bytes: Option<Vec<u8>>,
    badge_style: BadgeStyle,
) -> Result<Vec<u8>, SvgToPngConversionError> {
    raster::convert_svg_to_png(svg_bytes, badge_style, LetterSpacingSvgProcessor::new())
}
