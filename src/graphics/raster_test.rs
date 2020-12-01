use super::super::svg::MockSvgProcessor;
use super::super::test_data::{LETTER_SPACING_TEST_BYTES, TEST_BYTES};
use super::*;

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod get_dimensions_tests {
    use super::{get_dimensions, CairoRenderer as MockCairoRenderer};
    use librsvg::{IntrinsicDimensions, Length, LengthUnit};

    #[test]
    fn handles_zero_dimensions() {
        let mut mock_renderer = MockCairoRenderer::default();
        mock_renderer
            .expect_intrinsic_dimensions()
            .return_const(IntrinsicDimensions {
                width: None,
                height: None,
                vbox: None,
            });
        let (width, height) = get_dimensions(&mock_renderer);
        assert_eq!(width, 0f64);
        assert_eq!(height, 0f64);
    }

    #[test]
    fn provides_correct_width() {
        let exp_width = 70.33f64;
        let mut mock_renderer = MockCairoRenderer::default();
        mock_renderer
            .expect_intrinsic_dimensions()
            .return_const(IntrinsicDimensions {
                width: Some(Length {
                    length: exp_width,
                    unit: LengthUnit::Px,
                }),
                height: None,
                vbox: None,
            });
        let (width, height) = get_dimensions(&mock_renderer);
        assert_eq!(width, exp_width);
        assert_eq!(height, 0f64);
    }

    #[test]
    fn provides_correct_height() {
        let exp_height = 35.001f64;
        let mut mock_renderer = MockCairoRenderer::default();
        mock_renderer
            .expect_intrinsic_dimensions()
            .return_const(IntrinsicDimensions {
                width: None,
                height: Some(Length {
                    length: exp_height,
                    unit: LengthUnit::Px,
                }),
                vbox: None,
            });
        let (width, height) = get_dimensions(&mock_renderer);
        assert_eq!(width, 0f64);
        assert_eq!(height, exp_height);
    }
}

#[cfg(test)]
mod get_bytes_stream_tests {
    use super::{
        get_bytes_stream, BadgeStyle, Bytes, MockSvgProcessor, SvgToPngConversionError,
        INVALID_SVG, LETTER_SPACING_TEST_BYTES, TEST_BYTES,
    };
    use mockall::predicate::*;

    #[test]
    fn returns_static_invalid_svg_on_none() {
        let mut mock_svg_processor = MockSvgProcessor::new();
        mock_svg_processor
            .expect_prepare_svg_for_png_conversion()
            .with(eq(INVALID_SVG.to_owned()), eq(&BadgeStyle::Unspecified))
            .times(0)
            .returning(|_, _| Err(()));
        assert_eq!(
            Bytes::from_static(INVALID_SVG),
            get_bytes_stream(None, &BadgeStyle::Unspecified, &mock_svg_processor).unwrap(),
        );
    }

    #[test]
    fn maps_processing_error_correctly() {
        let mut mock_svg_processor = MockSvgProcessor::new();
        mock_svg_processor
            .expect_prepare_svg_for_png_conversion()
            .with(eq(TEST_BYTES.to_owned()), eq(&BadgeStyle::Unspecified))
            .times(1)
            .returning(|_, _| Err(()));
        assert_eq!(
            SvgToPngConversionError::SvgBytesProcessingFailure,
            get_bytes_stream(
                Some(TEST_BYTES.to_owned()),
                &BadgeStyle::Unspecified,
                &mock_svg_processor
            )
            .unwrap_err(),
        );
    }

    #[test]
    fn returns_owned_bytes_on_successful_conversion() {
        let mut mock_svg_processor = MockSvgProcessor::new();
        mock_svg_processor
            .expect_prepare_svg_for_png_conversion()
            .with(eq(TEST_BYTES.to_owned()), eq(&BadgeStyle::Unspecified))
            .times(1)
            .returning(|_, _| Ok(LETTER_SPACING_TEST_BYTES.to_owned()));
        assert_eq!(
            Bytes::from_owned(LETTER_SPACING_TEST_BYTES),
            get_bytes_stream(
                Some(TEST_BYTES.to_owned()),
                &BadgeStyle::Unspecified,
                &mock_svg_processor
            )
            .unwrap(),
        );
    }
}
