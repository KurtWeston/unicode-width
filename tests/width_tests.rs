use unicode_width_cli::{calculate_width, width_breakdown};

#[test]
fn test_ascii_strings() {
    assert_eq!(calculate_width("hello"), 5);
    assert_eq!(calculate_width("world!"), 6);
    assert_eq!(calculate_width(""), 0);
}

#[test]
fn test_emoji_strings() {
    assert_eq!(calculate_width("👋"), 2);
    assert_eq!(calculate_width("🎉🎊"), 4);
    assert_eq!(calculate_width("Hello 👋 World"), 13);
}

#[test]
fn test_cjk_characters() {
    assert_eq!(calculate_width("你好"), 4);
    assert_eq!(calculate_width("こんにちは"), 10);
    assert_eq!(calculate_width("안녕하세요"), 10);
}

#[test]
fn test_combining_characters() {
    assert_eq!(calculate_width("e\u{0301}"), 1);
    assert_eq!(calculate_width("café"), 4);
}

#[test]
fn test_zero_width_characters() {
    assert_eq!(calculate_width("\u{200B}"), 0);
    assert_eq!(calculate_width("hello\u{200B}world"), 10);
}

#[test]
fn test_mixed_content() {
    assert_eq!(calculate_width("Hello 世界 👋"), 12);
    assert_eq!(calculate_width("ASCII + 中文 + 🎉"), 16);
}

#[test]
fn test_width_breakdown() {
    let breakdown = width_breakdown("a你👋");
    assert_eq!(breakdown.len(), 3);
    assert_eq!(breakdown[0], ('a', 1));
    assert_eq!(breakdown[1], ('你', 2));
    assert_eq!(breakdown[2], ('👋', 2));
}
