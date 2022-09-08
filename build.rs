#[cfg(feature = "shadow")]
fn main() -> shadow_rs::SdResult<()> {
    shadow_rs::new()
}

#[cfg(not(feature = "shadow"))]
fn main() {}
