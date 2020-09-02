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
