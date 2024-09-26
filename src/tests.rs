use crate::Engine;

#[test]
fn orig_cmp_tests() {
    compare_rendering(
        include_bytes!("../testdata/rainysum.mod"),
        include_bytes!("../testdata/rainysum_orig.pcm"),
    );
    compare_rendering(
        include_bytes!("../testdata/sanxion.mod"),
        include_bytes!("../testdata/sanxion_orig.pcm"),
    );
}

fn compare_rendering(mod_data: &[u8], orig: &[u8]) {
    let mut test_bytes: Vec<u8> = Vec::new();
    let mut mm = Engine::new(mod_data, 48_000).unwrap();
    for _ in 0..1000 {
        let mut out = [0; 4096];
        mm.get_audio(&mut out, 2048);
        test_bytes.extend_from_slice(bytemuck::cast_slice(&out));
    }
    assert!(test_bytes == orig);
}
