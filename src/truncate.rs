use crate::calculate_width;

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

pub fn truncate(s: &str, max_width: usize) -> String {
    let mut current_width = 0;
    let mut result = String::new();

    for c in s.chars() {
        let char_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
        if current_width + char_width > max_width {
            break;
        }
        result.push(c);
        current_width += char_width;
    }

    result
}

pub fn pad(s: &str, target_width: usize, align: Alignment) -> String {
    let current_width = calculate_width(s);
    
    if current_width >= target_width {
        return s.to_string();
    }

    let padding_needed = target_width - current_width;
    let padding = " ".repeat(padding_needed);

    match align {
        Alignment::Left => format!("{}{}", s, padding),
        Alignment::Right => format!("{}{}", padding, s),
        Alignment::Center => {
            let left_pad = padding_needed / 2;
            let right_pad = padding_needed - left_pad;
            format!("{}{}{}", " ".repeat(left_pad), s, " ".repeat(right_pad))
        }
    }
}

pub fn truncate_and_pad(s: &str, width: usize, align: Alignment) -> String {
    let truncated = truncate(s, width);
    pad(&truncated, width, align)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_ascii() {
        assert_eq!(truncate("hello world", 5), "hello");
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello", 0), "");
    }

    #[test]
    fn test_truncate_cjk() {
        assert_eq!(truncate("你好世界", 4), "你好");
        assert_eq!(truncate("你好世界", 5), "你好");
    }

    #[test]
    fn test_pad_left() {
        assert_eq!(pad("hi", 5, Alignment::Left), "hi   ");
        assert_eq!(pad("你", 5, Alignment::Left), "你   ");
        assert_eq!(pad("hello", 3, Alignment::Left), "hello");
    }

    #[test]
    fn test_pad_right() {
        assert_eq!(pad("hi", 5, Alignment::Right), "   hi");
        assert_eq!(pad("你", 5, Alignment::Right), "   你");
    }

    #[test]
    fn test_pad_center() {
        assert_eq!(pad("hi", 6, Alignment::Center), "  hi  ");
        assert_eq!(pad("hi", 5, Alignment::Center), "  hi ");
        assert_eq!(pad("你", 6, Alignment::Center), "  你  ");
    }

    #[test]
    fn test_truncate_and_pad_left() {
        assert_eq!(truncate_and_pad("hello world", 8, Alignment::Left), "hello wo");
        assert_eq!(truncate_and_pad("hi", 8, Alignment::Left), "hi      ");
    }

    #[test]
    fn test_truncate_and_pad_right() {
        assert_eq!(truncate_and_pad("hi", 8, Alignment::Right), "      hi");
    }

    #[test]
    fn test_truncate_and_pad_center() {
        assert_eq!(truncate_and_pad("hi", 8, Alignment::Center), "   hi   ");
    }

    #[test]
    fn test_truncate_and_pad_mixed() {
        assert_eq!(truncate_and_pad("你好世界", 6, Alignment::Left), "你好世");
        assert_eq!(truncate_and_pad("a你b", 6, Alignment::Center), " a你b ");
    }
}