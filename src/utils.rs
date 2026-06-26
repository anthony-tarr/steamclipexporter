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

pub fn parse_clip_string(clip_string: &str) -> (u64, u64, u64) {
    let path = Path::new(clip_string);
    let last_part = path.file_name().unwrap().to_str().unwrap();
    let trimmed_part = last_part.trim_start_matches("clip_");
    let parts: Vec<&str> = trimmed_part.split('_').collect();
    println!("parts: {:?}", parts);
    let clip_number = parts[0].parse().unwrap();
    let date = parts[1].parse().unwrap();
    let time = parts[2].parse().unwrap();

    (clip_number, date, time)
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
        let (steam_id, date, time) = parse_clip_string("clip_12345_20250101_120000");
        assert_eq!(steam_id, 12345);
        assert_eq!(date, 20250101);
        assert_eq!(time, 120000);
    }
}
