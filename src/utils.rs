use std::path::{Path, PathBuf};

pub fn sort_chunks(chunk_files: &mut Vec<PathBuf>) {
    chunk_files.sort_by(|a, b| {
        // Extract the numeric part from the file names for comparison
        let a_num = a
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split('-').last())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        let b_num = b
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split('-').last())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        a_num.cmp(&b_num)
    });
}

pub fn parse_clip_string(clip_string: &str) -> Result<(u64, u64, u64), String> {
    let path = Path::new(clip_string);
    let last_part = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("Invalid clip path: {clip_string}"))?;
    let trimmed_part = last_part
        .strip_prefix("clip_")
        .ok_or_else(|| format!("Clip directory must start with 'clip_': {last_part}"))?;
    let mut parts = trimmed_part.split('_');

    let clip_number = parse_clip_part(parts.next(), "Steam app id", last_part)?;
    let date = parse_clip_part(parts.next(), "date", last_part)?;
    let time = parse_clip_part(parts.next(), "time", last_part)?;

    if parts.next().is_some() {
        return Err(format!(
            "Clip directory has unexpected extra parts: {last_part}"
        ));
    }

    Ok((clip_number, date, time))
}

fn parse_clip_part(part: Option<&str>, label: &str, clip_name: &str) -> Result<u64, String> {
    let part = part.ok_or_else(|| format!("Clip directory is missing {label}: {clip_name}"))?;
    part.parse::<u64>()
        .map_err(|_| format!("Clip directory has invalid {label}: {clip_name}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_sort_chunks() {
        let mut chunks = vec![
            PathBuf::from("chunk-stream0-2"),
            PathBuf::from("chunk-stream0-0"),
            PathBuf::from("chunk-stream0-1"),
        ];
        sort_chunks(&mut chunks);
        assert_eq!(
            chunks,
            vec![
                PathBuf::from("chunk-stream0-0"),
                PathBuf::from("chunk-stream0-1"),
                PathBuf::from("chunk-stream0-2"),
            ]
        );
    }

    #[test]
    fn test_parse_clip_string() {
        let (steam_id, date, time) =
            parse_clip_string("clip_12345_20250101_120000").expect("valid clip name");
        assert_eq!(steam_id, 12345);
        assert_eq!(date, 20250101);
        assert_eq!(time, 120000);
    }

    #[test]
    fn test_parse_clip_string_rejects_invalid_names() {
        assert!(parse_clip_string("not_a_clip").is_err());
        assert!(parse_clip_string("clip_12345_20250101").is_err());
        assert!(parse_clip_string("clip_invalid_20250101_120000").is_err());
        assert!(parse_clip_string("clip_12345_20250101_120000_extra").is_err());
    }
}
