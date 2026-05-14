use serde::Serialize;

/// File kind inferred from magic bytes/prefix.
///
/// This is not a replacement for a full parser. It is a safe first-pass
/// classification step for intake and routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileKind {
    Pdf,
    Zip,
    Png,
    Jpeg,
    Gif,
    Text,
    ElfExecutable,
    WindowsExecutable,
    Unknown,
}

pub fn classify_prefix(prefix: &[u8]) -> FileKind {
    if prefix.starts_with(b"%PDF") {
        FileKind::Pdf
    } else if prefix.starts_with(b"PK\x03\x04") {
        FileKind::Zip
    } else if prefix.starts_with(b"\x89PNG\r\n\x1a\n") {
        FileKind::Png
    } else if prefix.starts_with(&[0xff, 0xd8, 0xff]) {
        FileKind::Jpeg
    } else if prefix.starts_with(b"GIF87a") || prefix.starts_with(b"GIF89a") {
        FileKind::Gif
    } else if prefix.starts_with(&[0x7f, b'E', b'L', b'F']) {
        FileKind::ElfExecutable
    } else if prefix.starts_with(b"MZ") {
        FileKind::WindowsExecutable
    } else if looks_like_text(prefix) {
        FileKind::Text
    } else {
        FileKind::Unknown
    }
}

fn looks_like_text(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }

    let printable_or_whitespace = bytes.iter().filter(|byte| {
        let byte = **byte;
        matches!(byte, b'\n' | b'\r' | b'\t') || byte.is_ascii_graphic() || byte == b' '
    });

    printable_or_whitespace.count() * 100 / bytes.len() >= 90
}

pub fn expected_extensions(kind: FileKind) -> &'static [&'static str] {
    match kind {
        FileKind::Pdf => &["pdf"],
        FileKind::Zip => &["zip", "jar", "docx", "xlsx", "pptx", "odt", "ods", "odp"],
        FileKind::Png => &["png"],
        FileKind::Jpeg => &["jpg", "jpeg"],
        FileKind::Gif => &["gif"],
        FileKind::Text => &["txt", "log", "csv", "json", "toml", "yaml", "yml", "md"],
        FileKind::ElfExecutable => &["elf", "bin", "so"],
        FileKind::WindowsExecutable => &["exe", "dll"],
        FileKind::Unknown => &[],
    }
}
