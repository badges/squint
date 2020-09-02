use super::{convert_svg_to_png, SvgToPngConversion};
use crate::badge::BadgeStyle;

pub static INVALID_SVG: &[u8] = br##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="160" height="20">
    <linearGradient id="b" x2="0" y2="100%">
        <stop offset="0" stop-color="#bbbbbb" stop-opacity=".1"/>
        <stop offset="1" stop-opacity=".1"/>
    </linearGradient>
    <clipPath id="a">
        <rect width="160" height="20" rx="3" fill="#ffffff"/>
    </clipPath>
    <g clip-path="url(#a)">
        <path fill="#555555" d="M0 0h37v20H0z"/>
        <path fill="#e05d44" d="M37 0h123v20H37z"/>
        <path fill="url(#b)" d="M0 0h160v20H0z"/>
    </g>
    <g fill="#ffffff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="110">
        <text x="195" y="150" fill="010101" fill-opacity=".3" transform="scale(.1)" textLength="270">error</text>
        <text x="195" y="140" transform="scale(.1)" textLength="270">error</text>
        <text x="975" y="150" fill="010101" fill-opacity=".3" transform="scale(.1)" textLength="1130">invalid svg response</text>
        <text x="975" y="140" transform="scale(.1)" textLength="1130">invalid svg response</text>
    </g>
</svg>"##;

lazy_static! {
    pub static ref INVALID_SVG_BADGE: Vec<u8> = {
        match convert_svg_to_png(None, BadgeStyle::Unspecified) {
            SvgToPngConversion::Failure => {
                // This happens during server initialization, so if it fails
                // then we may as well crash, and crash loudly, as the server
                // won't be able to rasterize anything.
                panic!("Fatal error creating error response badge");
            }
            SvgToPngConversion::Success(png) => png,
        }
    };
}
