use std::{env, sync::Once};

use crate::BASE_URL_OVERRIDE;

static INIT: Once = Once::new();

pub fn prepare_tests() {
    INIT.call_once(|| {
        if let Ok(u) = env::var("ICEPORTAL_TESTS_URL") {
            let mut url_override = BASE_URL_OVERRIDE.write().unwrap();
            println!("Set iceportal tests url to {u}");
            *url_override = u;
        }
    });
}
