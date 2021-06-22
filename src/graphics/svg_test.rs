use super::super::test_data::{LETTER_SPACING_TEST_BYTES, TEST_BYTES};
use super::{BadgeStyle, LetterSpacingSvgProcessor, SvgProcessor};

#[cfg(test)]
mod letter_spacing_svg_processor_tests {
    use super::*;

    mod prepare_svg_for_png_conversion_tests {
        use super::*;

        fn assert_unchanged_test_bytes(style: &BadgeStyle) {
            assert_eq!(
                TEST_BYTES,
                LetterSpacingSvgProcessor::new()
                    .prepare_svg_for_png_conversion(TEST_BYTES.to_owned(), style)
                    .unwrap()
                    .as_slice(),
            )
        }

        #[test]
        fn correct_bytes_for_flat_style() {
            assert_unchanged_test_bytes(&BadgeStyle::Flat);
        }

        #[test]
        fn correct_bytes_for_flat_square_style() {
            assert_unchanged_test_bytes(&BadgeStyle::FlatSquare);
        }

        #[test]
        fn correct_bytes_for_social_style() {
            assert_unchanged_test_bytes(&BadgeStyle::Social);
        }

        #[test]
        fn correct_bytes_for_plastic_style() {
            assert_unchanged_test_bytes(&BadgeStyle::Plastic);
        }

        #[test]
        fn correct_bytes_for_unspecified_style() {
            assert_unchanged_test_bytes(&BadgeStyle::Unspecified);
        }

        mod for_the_badge_style {
            use super::*;

            #[test]
            fn text_length_replaced_with_letter_spacing() {
                // Viewing discrepancies between the expected and actual as the collection of bytes
                // isn't particularly helpful for human observers.
                // Convert to strings and trim whitespace indentation blocks and newline chars
                // so that the byte vectors will still match.
                assert_eq!(
                    String::from_utf8(LETTER_SPACING_TEST_BYTES.to_vec())
                        .unwrap()
                        .replace('\n', "")
                        .replace("    ", ""),
                    String::from_utf8(
                        LetterSpacingSvgProcessor::new()
                            .prepare_svg_for_png_conversion(
                                TEST_BYTES.to_owned(),
                                &BadgeStyle::ForTheBadge,
                            )
                            .unwrap()
                    )
                    .unwrap(),
                )
            }

            #[test]
            fn does_not_crash_when_text_length_missing() {
                const MISSING_TEXT_LENGTH_TEST_BYTES: &[u8] = br##"<?xml version="1.0" encoding="UTF-8"?>
                    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="80" height="20" role="img" aria-label="npm: v3.3.0">
                        <title>npm: v3.3.0</title>
                        <linearGradient id="s" x2="0" y2="100%">
                            <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
                            <stop offset="1" stop-opacity=".1"/>
                        </linearGradient>
                        <clipPath id="r">
                            <rect width="80" height="20" rx="3" fill="#fff"/>
                        </clipPath>
                        <g clip-path="url(#r)">
                            <rect width="35" height="20" fill="#555"/>
                            <rect x="35" width="45" height="20" fill="#007ec6"/>
                            <rect width="80" height="20" fill="url(#s)"/>
                        </g>
                        <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">
                            <text aria-hidden="true" x="185" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)">npm</text>
                            <text x="185" y="140" transform="scale(.1)" fill="#fff">npm</text>
                            <text aria-hidden="true" x="565" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)">v3.3.0</text>
                            <text x="565" y="140" transform="scale(.1)" fill="#fff">v3.3.0</text>
                        </g>
                    </svg>"##;
                assert_eq!(
                    String::from_utf8(LETTER_SPACING_TEST_BYTES.to_vec())
                        .unwrap()
                        .replace('\n', "")
                        .replace("    ", ""),
                    String::from_utf8(
                        LetterSpacingSvgProcessor::new()
                            .prepare_svg_for_png_conversion(
                                MISSING_TEXT_LENGTH_TEST_BYTES.to_owned(),
                                &BadgeStyle::ForTheBadge,
                            )
                            .unwrap()
                    )
                    .unwrap(),
                )
            }

            #[test]
            fn handles_invalid_xml_input() {
                assert!(LetterSpacingSvgProcessor::new()
                    .prepare_svg_for_png_conversion(
                        b"<?xml version=<".to_vec(),
                        &BadgeStyle::ForTheBadge,
                    )
                    .is_err(),);
            }
        }
    }
}
