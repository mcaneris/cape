#[cfg(feature = "chrono")]
pub type Timestamp = chrono::NaiveDateTime;

#[cfg(not(feature = "chrono"))]
pub type Timestamp = ();
