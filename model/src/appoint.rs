use std::sync::Arc;

use crate::constants::get_client;
use anyhow::{anyhow, Ok, Result};
use reqwest::{header::COOKIE, multipart::Form};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReserveResponse {
    #[serde(rename = "errno", default)]
    pub errno: i64,

    #[serde(rename = "error", default)]
    pub error: String,

    #[serde(rename = "total", default)]
    pub total: i64,
}

pub async fn reserve(url: &str, id: String, cookie: Arc<String>, time: String) -> Result<()> {
    let client = get_client();
    let form = Form::new().text("busScheduleId", id);
    let resp = client
        .post(url)
        .multipart(form)
        .header(COOKIE, cookie.as_str())
        .send()
        .await?;

    if resp.status().as_u16() == 401 {
        return Err(anyhow!("cookie 已过期"));
    }

    let resp: ReserveResponse = resp.json().await?;

    if resp.errno != 0 {
        return Err(anyhow!("预约[{}]失败: {}", time, resp.error));
    }

    Ok(())
}
