use super::convert_svg_to_png;
use crate::badge::BadgeStyle;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

#[cfg(test)]
use mockall::automock;

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
            Ok(png) => png,
            Err(_) => {
                // This happens during server initialization, so if it fails
                // then we may as well crash, and crash loudly, as the server
                // won't be able to rasterize anything.
                panic!("Fatal error creating error response badge");
            }
        }
    };
}
/// Various utility methods for processing SVGs.
#[cfg_attr(test, automock)]
pub(super) trait SvgProcessor {
    /// Performs preprocessing of an SVG in preparation
    /// for converting the SVG to a PNG.
    fn prepare_svg_for_png_conversion(
        &self,
        orig_bytes: Vec<u8>,
        badge_style: &BadgeStyle,
    ) -> Result<Vec<u8>, ()>;
}

// Default SVG processor that will swap letter-spacing in for textLength
// as necessary depending on the badge style.
pub(super) struct LetterSpacingSvgProcessor {}

impl LetterSpacingSvgProcessor {
    pub(super) fn new() -> Self {
        Self {}
    }

    fn transform_for_the_badge(&self, orig_bytes: Vec<u8>) -> Result<Vec<u8>, ()> {
        let mut reader = Reader::from_reader(orig_bytes.as_slice());
        reader.trim_text(true);
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut buf = Vec::new();
        // TODO - insert link to Shields.io source where this is set
        let letter_spacing = "12.5";

        loop {
            let event = reader.read_event(&mut buf).map_err(|_| ())?;
            match event {
                Event::Start(ref e) if e.name() == b"text" => {
                    let mut elem = BytesStart::borrowed(b"text", "text".len());
                    let attrs = e
                        .attributes()
                        .filter_map(|a| a.ok().filter(|a| a.key != b"textLength"))
                        .collect::<Vec<_>>();
                    elem.extend_attributes(attrs);
                    elem.push_attribute(("letter-spacing", letter_spacing));
                    writer.write_event(Event::Start(elem)).map_err(|_| ())?;
                }
                Event::Eof => break,
                _ => {
                    writer.write_event(&event).map_err(|_| ())?;
                }
            }
            buf.clear();
        }

        Ok(writer.into_inner().into_inner())
    }
}

impl SvgProcessor for LetterSpacingSvgProcessor {
    fn prepare_svg_for_png_conversion(
        &self,
        orig_bytes: Vec<u8>,
        badge_style: &BadgeStyle,
    ) -> Result<Vec<u8>, ()> {
        match badge_style {
            BadgeStyle::Flat
            | BadgeStyle::FlatSquare
            | BadgeStyle::Social
            | BadgeStyle::Plastic
            | BadgeStyle::Unspecified => Ok(orig_bytes),
            BadgeStyle::ForTheBadge => self.transform_for_the_badge(orig_bytes),
        }
    }
}

#[cfg(test)]
#[path = "svg_test.rs"]
mod tests;
