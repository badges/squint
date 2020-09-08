use super::{BadgeStyle, LetterSpacer, SimpleCairoDockerLetterSpacer};

#[cfg(test)]
mod simple_cairo_docker_letter_spacer {
    use super::*;

    mod get_letter_spacing_adjustment {
        use super::{BadgeStyle, SimpleCairoDockerLetterSpacer};

        fn assert_style_adjustment_value(style: &BadgeStyle) {
            assert_eq!(
                0.0,
                SimpleCairoDockerLetterSpacer::new()
                    .get_letter_spacing_adjustment(2.0, false, style)
            );
        }

        #[test]
        fn returns_zero_for_flat_square_style() {
            assert_style_adjustment_value(&BadgeStyle::FlatSquare);
        }

        #[test]
        fn returns_zero_for_flat_style() {
            assert_style_adjustment_value(&BadgeStyle::Flat);
        }

        #[test]
        fn returns_zero_for_plastic_style() {
            assert_style_adjustment_value(&BadgeStyle::Plastic);
        }

        #[test]
        fn returns_zero_for_social_style() {
            assert_style_adjustment_value(&BadgeStyle::Social);
        }
    }

    mod get_letter_spacing {
        use super::*;

        fn assert_letter_spacing_is_none(style: &BadgeStyle) {
            assert!(SimpleCairoDockerLetterSpacer::new()
                .get_letter_spacing("", false, |_: &str| 32.0, 350.0, 0.1, style,)
                .is_none(),);
        }

        #[test]
        fn returns_none_for_flat_style() {
            assert_letter_spacing_is_none(&BadgeStyle::Flat);
        }

        #[test]
        fn returns_none_for_flat_square_style() {
            assert_letter_spacing_is_none(&BadgeStyle::FlatSquare);
        }

        #[test]
        fn returns_none_for_plastic_style() {
            assert_letter_spacing_is_none(&BadgeStyle::Plastic);
        }

        #[test]
        fn returns_none_for_social_style() {
            assert_letter_spacing_is_none(&BadgeStyle::Social);
        }

        mod ftb_style_bold_text {
            use super::{BadgeStyle, LetterSpacer, SimpleCairoDockerLetterSpacer};

            #[test]
            fn derives_correct_value_for_extra_short_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 21.000000000000004;
                let is_bold = true;
                let text_length = 250.0;
                let scale_ratio = 0.1;
                let text = "BAR";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(11.027777777777775));
            }

            #[test]
            fn derives_correct_value_for_short_bold_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 42.001;
                let is_bold = true;
                let text_length = 610.0;
                let scale_ratio = 0.1;
                let text = "PASSING";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(24.01877551020408));
            }

            #[test]
            fn derives_correct_value_for_medium_bold_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 78.0;
                let is_bold = true;
                let text_length = 1090.0;
                let scale_ratio = 0.1;
                let text = "SOMETHINGBIG";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(20.486111111111114));
            }

            #[test]
            fn derives_correct_value_for_large_bold_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 154.0;
                let is_bold = true;
                let text_length = 2090.0;
                let scale_ratio = 0.1;
                let text = "BARFFFFFFFFFFF232FACDAFD";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(15.62152777777778));
            }

            #[test]
            fn derives_correct_value_for_extra_large_bold_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 262.000006;
                let is_bold = true;
                let text_length = 3610.0;
                let scale_ratio = 0.1;
                let text = "barffffffffffffffffffffff8766ghhhhjfffffff";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(15.632651598639459));
            }
        }

        mod ftb_style_non_bold_text {
            use super::{BadgeStyle, LetterSpacer, SimpleCairoDockerLetterSpacer};

            #[test]
            fn derives_correct_value_for_extra_short_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 21.000000000000004;
                let is_bold = false;
                let text_length = 235.0;
                let scale_ratio = 0.1;
                let text = "BAR";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(14.111111111111095));
            }

            #[test]
            fn derives_correct_value_for_short_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 42.00000000000001;
                let is_bold = false;
                let text_length = 575.0;
                let scale_ratio = 0.1;
                let text = "PASSING";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(27.80612244897957));
            }

            #[test]
            fn derives_correct_value_for_medium_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 78.0;
                let is_bold = false;
                let text_length = 1030.0;
                let scale_ratio = 0.1;
                let text = "SOMETHINGBIG";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(22.96944444444445));
            }

            #[test]
            fn derives_correct_value_for_large_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 154.0;
                let is_bold = false;
                let text_length = 1970.0;
                let scale_ratio = 0.1;
                let text = "BARFFFFFFFFFFF232FACDAFD";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(19.063194444444445));
            }

            #[test]
            fn derives_correct_value_for_extra_large_text() {
                let spacer = SimpleCairoDockerLetterSpacer::new();
                let get_text_width = |_: &str| 262.000006;
                let is_bold = false;
                let text_length = 3400.0;
                let scale_ratio = 0.1;
                let text = "barffffffffffffffffffffff8766ghhhhjfffffff";
                let letter_spacing = spacer.get_letter_spacing(
                    text,
                    is_bold,
                    get_text_width,
                    text_length,
                    scale_ratio,
                    &BadgeStyle::ForTheBadge,
                );
                assert_eq!(letter_spacing, Some(19.413603979591837));
            }
        }
    }
}
