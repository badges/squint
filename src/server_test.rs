#[cfg(test)]
mod extract_pixel_ratio_tests {
    use crate::server::extract_pixel_ratio;

    #[test]
    fn none() {
        assert_eq!(extract_pixel_ratio(None), (1, None));
    }

    #[test]
    fn empty() {
        assert_eq!(extract_pixel_ratio(Some("")), (1, None));
    }

    #[test]
    fn different_param() {
        assert_eq!(
            extract_pixel_ratio(Some("foo=bar")),
            (1, Some("foo=bar".to_string()))
        );
    }

    #[test]
    fn empty_param() {
        assert_eq!(extract_pixel_ratio(Some("pixel_ratio=")), (1, None));
    }

    #[test]
    fn not_an_int() {
        assert_eq!(extract_pixel_ratio(Some("pixel_ratio=foo")), (1, None));
    }

    #[test]
    fn not_a_whole_number() {
        // A decimal parses neither as u8 nor to a valid ratio, so it falls back
        // to 1 (and is still stripped from the forwarded query).
        assert_eq!(extract_pixel_ratio(Some("pixel_ratio=2.5")), (1, None));
    }

    #[test]
    fn normal_case() {
        assert_eq!(extract_pixel_ratio(Some("pixel_ratio=2")), (2, None));
    }

    #[test]
    fn over_max() {
        // Out-of-range ratios fall back to 1 rather than clamping to MAX_PIXEL_RATIO.
        assert_eq!(extract_pixel_ratio(Some("pixel_ratio=5")), (1, None));
    }

    #[test]
    fn under_min() {
        // Out-of-range ratios fall back to 1 rather than clamping to MIN_PIXEL_RATIO.
        assert_eq!(extract_pixel_ratio(Some("pixel_ratio=0")), (1, None));
    }

    #[test]
    fn strips_pixel_ratio_keeps_others() {
        assert_eq!(
            extract_pixel_ratio(Some("style=flat&pixel_ratio=3&label=hi")),
            (3, Some("style=flat&label=hi".to_string()))
        );
    }

    #[test]
    fn invalid_pixel_ratio_is_still_stripped() {
        assert_eq!(
            extract_pixel_ratio(Some("pixel_ratio=99&style=flat")),
            (1, Some("style=flat".to_string()))
        );
    }
}
