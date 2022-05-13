use crate::constants::get_client;
use anyhow::{anyhow, Ok, Result};

use reqwest::header::COOKIE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EntityResponse {
    #[serde(rename = "errno", default)]
    pub errno: i64,

    #[serde(rename = "error", default)]
    pub error: String,

    #[serde(rename = "total", default)]
    pub total: i64,

    #[serde(rename = "entities", default)]
    pub entities: Vec<Entity>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Entity {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "isNonStop", default)]
    pub is_non_stop: bool,

    #[serde(rename = "startTime", default)]
    pub start_time: String,

    #[serde(rename = "endTime", default)]
    pub end_time: String,

    #[serde(rename = "stateDate", default)]
    pub state_date: String,

    #[serde(rename = "loginName", default)]
    pub login_name: String,

    #[serde(rename = "leftSeat", default)]
    pub left_seat: i64,

    #[serde(rename = "stopStation", default)]
    pub stop_station: String,

    #[serde(rename = "status", default)]
    pub status: String,

    #[serde(rename = "userAppointmentId", default)]
    pub user_appointment_id: String,

    #[serde(rename = "capacitySeat", default)]
    pub capacity_seat: i64,

    #[serde(rename = "isPublish", default)]
    pub is_publish: bool,

    #[serde(rename = "busNum", default)]
    pub bus_num: i64,

    #[serde(rename = "isBlackList", default)]
    pub is_black_list: bool,

    #[serde(rename = "isAppointmentLimited", default)]
    pub is_appointment_limited: bool,
}

pub async fn get_entity(url: &str, cookie: &str) -> Result<Vec<Entity>> {
    let client = get_client();

    let resp = client.get(url).header(COOKIE, cookie).send().await?;

    if resp.status().as_u16() == 401 {
        return Err(anyhow!("cookie 已过期"));
    }

    let resp: EntityResponse = resp.json().await?;
    if resp.errno != 0 {
        return Err(anyhow!("获取预约列表失败: {}", resp.error));
    }

    Ok(resp.entities)
}

#[tokio::test]
async fn test_get_entity() {
    let res = get_entity("https://dailyreport.sjtu.edu.cn/haircut/frontend/bus/schedule/list?lineType=TWO&date=2022-05-03", "agreeChecked=true; JSESSIONID=D16004E0991696C20025753981426DAA; dailyreport.sjtu=ffffffff097e1f5345525d5f4f58455e445a4a4229a0").await.unwrap();
    println!("{:?}", res);
}
