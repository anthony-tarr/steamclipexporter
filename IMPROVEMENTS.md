# Steam Clip Exporter Improvements

## Highest-Impact Issues

- `src/main.rs:101`: `concat_m4s_files(...)` returns `Result<()>`, but the result is ignored. Failed exports can look successful.
- `src/main.rs:209` and `src/main.rs:252`: initial video/audio `io::copy(...)` results are ignored. A failed copy could still proceed into `ffmpeg`.
- `src/utils.rs:25`: `parse_clip_string` uses `unwrap()` and unchecked indexes. One unexpected directory name can crash the whole batch.
- `src/main.rs:93`: if no `video/bg_*` directory exists, this silently becomes an empty path and later fails with a confusing "init files not found" error.
- `src/main.rs:292`: when `-o` is omitted, output goes to the current working directory, not the clips directory as the README implies.

## Recommended Improvements

- Replace noisy `println!` debugging with structured logging or gate it behind `--verbose`. `parse_clip_string` currently always prints `parts`.
- Cache Steam app names by app id. Multiple clips from the same game currently make repeated API calls.
- Use `serde_json::Value::as_str()` for the Steam app name instead of `name.to_string()`, otherwise JSON string quoting can leak into filename handling.
- Add a timeout to the Steam API client and reuse one `reqwest::blocking::Client`.
- Pipe or inherit `ffmpeg` stderr. `ffmpeg` writes most useful output to stderr, but the code only reads stdout.
- Handle existing output filenames explicitly. Consider unique suffixes or an `--overwrite` flag.
- Remove or expose `quick_join_video_audio`; clippy flags it as dead code.
- Add integration-style tests using a temp clip directory layout to cover directory discovery, malformed names, missing `bg_*`, chunk ordering, and destination path behavior.
- Extend CI to run `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D warnings`.

## Verification Notes

- `cargo test` passes with 2 tests.
- `cargo fmt --check` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` fails due to ignored `Result`s, dead code, and smaller style issues.
