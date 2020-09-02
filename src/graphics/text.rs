use crate::badge::BadgeStyle;

pub(super) trait LetterSpacing {
    fn get_letter_spacing(element_text: &str, badge_style: BadgeStyle);
}
