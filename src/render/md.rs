pub fn render_md(text: &str) -> String {
    let mut out = String::new();
    out.push_str("```text\n");
    out.push_str(text);
    out.push('\n');
    out.push_str("```");
    out
}
