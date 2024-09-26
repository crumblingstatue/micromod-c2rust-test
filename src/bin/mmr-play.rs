use {
    micromod_c2rust_test::MmC2r,
    std::{
        io::{BufWriter, Write as _},
        process::Command,
    },
};

fn main() {
    let mod_data = std::fs::read(std::env::args_os().nth(1).expect("Need path to song")).unwrap();
    let output_file = std::fs::File::create("/tmp/micromod_out.pcm").unwrap();
    let mut writer = BufWriter::new(output_file);
    let mut mm = MmC2r::new(&mod_data, 48_000).unwrap();
    loop {
        let mut out = [0; 4096];
        if !mm.get_audio(&mut out, 2048) {
            break;
        }
        for sample in out {
            writer.write_all(sample.to_le_bytes().as_slice()).unwrap();
        }
    }
    Command::new("aplay")
        .args([
            "-f",
            "s16_le",
            "-r",
            "48000",
            "-c",
            "2",
            "/tmp/micromod_out.pcm",
        ])
        .status()
        .unwrap();
}
