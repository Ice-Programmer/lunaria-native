use std::time::{SystemTime, UNIX_EPOCH};

use crate::DatabaseError;

pub(crate) fn unix_timestamp_secs() -> Result<i64, DatabaseError> {
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    i64::try_from(seconds).map_err(|_| DatabaseError::TimestampOutOfRange)
}
