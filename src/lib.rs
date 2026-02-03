use unicode_width::UnicodeWidthChar;

pub fn calculate_width(s: &str) -> usize {
    s.chars().map(|c| UnicodeWidthChar::width(c).unwrap_or(0)).sum()
}

pub fn width_breakdown(s: &str) -> Vec<(char, usize)> {
    s.chars()
        .map(|c| (c, UnicodeWidthChar::width(c).unwrap_or(0)))
        .collect()
}

pub fn grapheme_safe_truncate(s: &str, max_width: usize) -> String {
    let mut current_width = 0;
    let mut result = String::new();

    for c in s.chars() {
        let char_width = UnicodeWidthChar::width(c).unwrap_or(0);
        if current_width + char_width > max_width {
            break;
        }
        result.push(c);
        current_width += char_width;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_width() {
        assert_eq!(calculate_width("hello"), 5);
        assert_eq!(calculate_width("a"), 1);
        assert_eq!(calculate_width(""), 0);
    }

    #[test]
    fn test_emoji_width() {
        assert_eq!(calculate_width("👋"), 2);
        assert_eq!(calculate_width("🎉"), 2);
        assert_eq!(calculate_width("👨‍👩‍👧‍👦"), 2);
    }

    #[test]
    fn test_cjk_width() {
        assert_eq!(calculate_width("你好"), 4);
        assert_eq!(calculate_width("日本語"), 6);
        assert_eq!(calculate_width("한글"), 4);
    }

    #[test]
    fn test_combining_chars() {
        assert_eq!(calculate_width("e\u{0301}"), 1);
        assert_eq!(calculate_width("a\u{0300}\u{0301}"), 1);
    }

    #[test]
    fn test_zero_width_chars() {
        assert_eq!(calculate_width("\u{200B}"), 0);
        assert_eq!(calculate_width("hello\u{200B}world"), 10);
    }

    #[test]
    fn test_mixed_content() {
        assert_eq!(calculate_width("Hello 世界 👋"), 13);
        assert_eq!(calculate_width("abc你好123"), 11);
    }

    #[test]
    fn test_width_breakdown() {
        let result = width_breakdown("a你👋");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ('a', 1));
        assert_eq!(result[1], ('你', 2));
        assert_eq!(result[2], ('👋', 2));
    }

    #[test]
    fn test_width_breakdown_empty() {
        let result = width_breakdown("");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_grapheme_safe_truncate_ascii() {
        assert_eq!(grapheme_safe_truncate("hello", 3), "hel");
        assert_eq!(grapheme_safe_truncate("hello", 10), "hello");
        assert_eq!(grapheme_safe_truncate("hello", 0), "");
    }

    #[test]
    fn test_grapheme_safe_truncate_cjk() {
        assert_eq!(grapheme_safe_truncate("你好世界", 4), "你好");
        assert_eq!(grapheme_safe_truncate("你好世界", 5), "你好");
        assert_eq!(grapheme_safe_truncate("你好世界", 6), "你好世");
    }

    #[test]
    fn test_grapheme_safe_truncate_emoji() {
        assert_eq!(grapheme_safe_truncate("👋👋👋", 4), "👋👋");
        assert_eq!(grapheme_safe_truncate("abc👋", 4), "abc");
    }

    #[test]
    fn test_grapheme_safe_truncate_mixed() {
        assert_eq!(grapheme_safe_truncate("Hello世界", 7), "Hello世");
        assert_eq!(grapheme_safe_truncate("a你b", 3), "a你");
    }
}