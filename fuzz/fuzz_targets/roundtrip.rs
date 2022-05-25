#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (orz::LZCfg, &[u8])| {
    let (cfg, mut source) = data;
    let mut target = Vec::new();
    if let Ok(stats) = orz::encode(&mut source, &mut target, &cfg) {
        assert_eq!(stats.source_size, data.1.len() as u64);
        assert_eq!(stats.target_size, target.len() as u64);

        #[cfg(feature = "dump-target")]
        std::fs::write("blob", &target).unwrap();

        let mut decoded = Vec::new();
        if let Ok(stats) = orz::decode(&mut target.as_slice(), &mut decoded) {
            assert_eq!(stats.source_size, data.1.len() as u64);
            assert_eq!(stats.target_size, target.len() as u64);
            assert_eq!(decoded, data.1);
        }
    }
});
