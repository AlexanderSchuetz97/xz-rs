## xzcheck
This program checks if .xz files are valid and can be decoded by the rust xz-rs library without errors.

You can install it via `cargo install xzcheck`
The program fully decodes the entire .xz file and checks the content hash/checksum.
If it succeeds in decoding the file it exits with code 0 otherwise it exits with code 255 and
prints an error to stderr.