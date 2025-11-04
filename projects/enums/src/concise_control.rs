const CONFIG_MAX: Option<u8> = Some(3);

pub fn check_config() {
    if let Some(max) = CONFIG_MAX {
        println!("The maximum is configured to be {max}");
    } else {
    }
}
