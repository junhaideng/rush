use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

pub fn get_client() -> &'static Client {
    &CLIENT
}

macro_rules! define_constants {
  ($(($name: ident, $value:expr);)+) => {
      $(
        pub const $name: &str = $value;
      )*
  };
}

define_constants! {
  (CAN_RESERVE,"CAN_RESERVE"); // 可预约
  (NOT_RESERVE, "NOT_RESERVE"); // 已过期
  (RESERVE_LIMITED, "RESERVE_LIMITED");  // 已约满
  (RESERVER_SUCCESS, "RESERVER_SUCCESS"); // 预约成功
}

define_constants! {
    // 超市
    (DQJYCS, "DQJYCS");
    (XQJYCS, "XQJYCS");
    (YLYLS, "YLYLS");

    // 理发
    (SECOND, "TWO");
    (THIRD, "THIRD");
    (FOURTH, "FOURTH");
}

define_constants! {
  (MARKET_SCHEDULE_LIST, "https://dailyreport.sjtu.edu.cn/market/frontend/market/schedule/list");  // ?lineType=YLYLS&date=xxxx-xx-xx
  (MARKET_APPOINTMENT, "https://dailyreport.sjtu.edu.cn/market/frontend/market/appointment/save");

  (HAIRCUT_SCHEDULE_LIST, "https://dailyreport.sjtu.edu.cn/haircut//frontend/bus/schedule/list");  // ?lineType=YLYLS&date=xxxx-xx-xx
  (HAIRCUT_APPOINTMENT, "https://dailyreport.sjtu.edu.cn/haircut/frontend/bus/appointment/save");
}

pub struct Market(String, String);
pub struct Haircut(String, String);

impl TryFrom<String> for Market {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "1" => Ok(Market(String::from(XQJYCS), String::from("西区教超"))),
            "2" => Ok(Market(String::from(DQJYCS), String::from("东区教超"))),
            "3" => Ok(Market(String::from(YLYLS), String::from("罗森"))),
            _ => Err(format!("未知参数 [{}], 仅支持 1,2,3", s)),
        }
    }
}

impl TryFrom<String> for Haircut {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "2" => Ok(Haircut(String::from(SECOND), String::from("二餐理发"))),
            "3" => Ok(Haircut(String::from(THIRD), String::from("三餐理发"))),
            "4" => Ok(Haircut(String::from(FOURTH), String::from("四餐理发"))),
            _ => Err(format!("未知参数 [{}], 仅支持 2,3,4", s)),
        }
    }
}

pub trait Rush {
    fn schedule_list_url(&self, date: &str) -> String;
    fn appoint_url(&self) -> &'static str;
    fn get_name(&self) -> &str;
}

macro_rules! impl_name {
    ($ty:ty, $schedule_list_url:expr, $appoint_url: expr) => {
        impl Rush for $ty {
            fn schedule_list_url(&self, date: &str) -> String {
                format!("{}?lineType={}&date={}", $schedule_list_url, self.0, date)
            }

            fn appoint_url(&self) -> &'static str {
                $appoint_url
            }
            fn get_name(&self) -> &str {
                &self.1
            }
        }
    };
}

impl_name!(Market, MARKET_SCHEDULE_LIST, MARKET_APPOINTMENT);
impl_name!(Haircut, HAIRCUT_SCHEDULE_LIST, HAIRCUT_APPOINTMENT);

#[test]
fn test_from() {
    let s = "1".to_string();
    let m: Result<Market, _> = s.try_into();
    assert!(m.is_ok());
    println!("{}", m.unwrap().0);
}
