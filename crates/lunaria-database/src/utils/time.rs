use anyhow::{Context, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn unix_timestamp_secs() -> Result<i64> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is earlier than the Unix epoch")?
        .as_secs();

    i64::try_from(seconds).context("Unix timestamp is outside the supported range")
}
