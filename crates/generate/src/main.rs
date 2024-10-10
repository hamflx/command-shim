fn main() {
    let command = std::env::args().skip(1).next().unwrap();

    let exe_binary = include_bytes!(env!("CARGO_BIN_FILE_SHIM_shim"));
    let exe_binary = exe_binary.to_vec();
    let mut output = Vec::new();
    #[cfg(unix)]
    libsui::Elf::new(&exe_binary)
        .append("command", command.as_bytes(), &mut output)
        .unwrap();
    #[cfg(windows)]
    libsui::PortableExecutable::from(&exe_binary)
        .unwrap()
        .write_resource("command", command.as_bytes().into())
        .unwrap()
        .build(&mut output)
        .unwrap();

    let output_filename = std::env::args()
        .skip(2)
        .next()
        .unwrap_or_else(|| format!("{command}.exe"));
    std::fs::write(output_filename, output).unwrap();
}
