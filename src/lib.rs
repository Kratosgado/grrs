
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write){
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}


#[test]
fn test_answer() {
    let mut result = Vec::new();
    find_matches("foo\nbar\nbaz", "bar", &mut result);
    assert_eq!(result, b"bar\n");
}