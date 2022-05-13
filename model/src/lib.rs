mod appoint;
pub mod constants;
mod entity;

use anyhow::Result;
use appoint::reserve;
use entity::get_entity;
use std::sync::Arc;

pub async fn appoint(
    schedule_list: &str,
    appoint_url: &'static str,
    cookie: String,
    asc: bool,
) -> Result<bool> {
    let mut entity = get_entity(schedule_list, &cookie).await?;
    if !asc {
        entity.reverse();
    }
    let cookie = Arc::new(cookie);

    let mut handles = vec![];

    for e in entity.iter() {
        let c = cookie.clone();
        let id = e.id.clone();
        let date = format!("{}-{}", e.start_time, e.end_time);
        let handle = tokio::spawn(async move { reserve(appoint_url, id, c, date).await });
        handles.push(handle);
    }

    for handle in handles {
        match handle.await? {
            Ok(_) => {
                println!("预约成功");
                return Ok(true);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    Ok(false)
}
