use crate::time::get_current_time;
use anyhow::{anyhow, Result};
use model::{appoint, constants::Rush};

pub async fn handle<T>(cookie: &str, choices: &str, asc: &str) -> Result<()>
where
    T: TryFrom<String, Error = String> + Rush,
{
    let time = get_current_time();

    for choice in choices.split(',') {
        let choice: T = String::from(choice)
            .try_into()
            .map_err(|e: String| anyhow!(e))?;

        match appoint(
            &choice.schedule_list_url(&time),
            choice.appoint_url(),
            cookie.to_string(),
            asc.to_lowercase() == "true",
        )
        .await
        {
            Ok(true) => println!("预约[{}]成功", choice.get_name()),
            Ok(false) => println!("预约[{}]失败", choice.get_name()),
            Err(e) => println!("预约[{}]失败: ", e),
        }
    }

    Ok(())
}
