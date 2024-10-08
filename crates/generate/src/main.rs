fn main() {
    let exe_binary = include_bytes!(env!("CARGO_BIN_FILE_SHIM_shim"));
    let mut exe_binary = exe_binary.to_vec();
    let magic =
        b"<MAGIC_STRING_START______________________________________________MAGIC_STRING_END>\0";
    let command = std::env::args().skip(1).next().unwrap();
    let output = std::env::args()
        .skip(2)
        .next()
        .unwrap_or_else(|| format!("{command}.exe"));
    let position = find_subsequence(&exe_binary, magic).unwrap();
    let replacement = command
        .as_bytes()
        .iter()
        .copied()
        .chain(std::iter::repeat(0))
        .take(magic.len())
        .collect::<Vec<_>>();
    exe_binary[position..position + magic.len()].copy_from_slice(&replacement);
    std::fs::write(output, exe_binary).unwrap();
}

pub fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
