use crate::badge::BadgeStyle;

pub(super) trait LetterSpacer {
    fn get_letter_spacing<W: FnOnce(&str) -> f64>(
        &self,
        text: &str,
        is_bold: bool,
        get_text_width: W,
        text_length: f64,
        scale_ratio: f64,
        badge_style: &BadgeStyle,
    ) -> Option<f64>;
}

// Naive and handcrafted letter spacing calculator, fine tuned
// for the Debian-based Docker image using Cairo.
pub(super) struct SimpleCairoDockerLetterSpacer {}

impl SimpleCairoDockerLetterSpacer {
    pub(super) fn new() -> Self {
        Self {}
    }

    fn get_letter_spacing_adjustment(
        &self,
        num_chars: f64,
        is_bold: bool,
        badge_style: &BadgeStyle,
    ) -> f64 {
        // These are indeed magic numbers, based off some manual tuning with
        // various badges.
        match badge_style {
            BadgeStyle::ForTheBadge if !is_bold && num_chars < 5.0 => 0.3,
            BadgeStyle::ForTheBadge if !is_bold && num_chars < 10.0 => 0.25,
            BadgeStyle::ForTheBadge if !is_bold => 0.04,
            BadgeStyle::ForTheBadge if num_chars < 5.0 => -0.675,
            BadgeStyle::ForTheBadge if num_chars < 10.0 => -0.7,
            BadgeStyle::ForTheBadge if num_chars < 20.0 => -0.75,
            BadgeStyle::ForTheBadge if num_chars < 25.0 => -0.825,
            BadgeStyle::ForTheBadge => -0.85,
            _ => 0.0,
        }
    }
}

impl LetterSpacer for SimpleCairoDockerLetterSpacer {
    fn get_letter_spacing<W: FnOnce(&str) -> f64>(
        &self,
        text: &str,
        is_bold: bool,
        get_text_width: W,
        text_length: f64,
        scale_ratio: f64,
        badge_style: &BadgeStyle,
    ) -> Option<f64> {
        if BadgeStyle::ForTheBadge != *badge_style {
            return None;
        }

        let num_chars = text.chars().count() as f64;
        let letter_spacing_adjustment =
            self.get_letter_spacing_adjustment(num_chars, is_bold, &badge_style) / scale_ratio;
        let text_width = get_text_width(text);
        let scaled_text_length = text_length * scale_ratio;

        let spacing = scaled_text_length / num_chars;
        let char_width = text_width / num_chars;
        Some(
            letter_spacing_adjustment
                + ((spacing - char_width) + (spacing - char_width) / num_chars) / scale_ratio,
        )
    }
}

#[cfg(test)]
#[path = "text_test.rs"]
mod tests;
