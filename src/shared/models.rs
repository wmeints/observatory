use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    title: String,
    body: String,
    category: String,
    date_created: NaiveDateTime,
    date_modified: Option<NaiveDateTime>,
    date_published: Option<NaiveDateTime>,
}
