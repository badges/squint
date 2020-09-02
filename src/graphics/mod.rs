mod raster;
mod svg;

pub(crate) use raster::{convert_svg_to_png, SvgToPngConversion};
use svg::INVALID_SVG;
pub(crate) use svg::INVALID_SVG_BADGE;
