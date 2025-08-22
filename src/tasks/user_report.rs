use loco_rs::prelude::*;

use crate::models::users;

pub struct UserReport;
#[async_trait]
impl Task for UserReport {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "user_report".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        // println!("Task UserReport generated");
        let users = users::Entity::find().all(&_app_context.db).await?;
        users.iter().for_each(|e| println!("User: {e:?}"));
        Ok(())
    }
}
