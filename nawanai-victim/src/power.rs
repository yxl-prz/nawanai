pub fn power_action(s_type: u8) -> Result<std::process::Output, std::io::Error> {
    std::process::Command::new("shutdown")
        .args(&[
            match s_type {
                crate::FLAG_POWER_SHUTDOWN => "/s",
                crate::FLAG_POWER_RESTART => "/r",
                _ => "/s",
            },
            "/t",
            "0",
        ])
        .output()
}
