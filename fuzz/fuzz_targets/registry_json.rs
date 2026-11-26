#![no_main]

use ferrum_ui::registry::Registry;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(json) = std::str::from_utf8(data) {
        let _ = Registry::from_json(json);
    }
});