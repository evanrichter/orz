#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut reader = data;
    let mut sink = std::io::sink();
    if let Ok(stats) = orz::decode(&mut reader, &mut sink) {
        assert_eq!(
            stats.target_size,
            data.len() as u64,
            "decode was Ok() but did not consume all input data"
        )
    }
});
