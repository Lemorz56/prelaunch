use loco_rs::prelude::*;

use crate::models::_entities::contacts;

pub struct DumpContacts;
#[async_trait]
impl Task for DumpContacts {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "dump_contacts".to_string(),
            detail: "Dump all contacts".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let contacts = contacts::Entity::find().all(&_app_context.db).await?;
        for (idx, contact) in contacts.iter().enumerate() {
            println!("{} #{}", contact.email, idx + 1);
        }
        Ok(())
    }
}
