use sea_orm::DatabaseConnection;
use sea_orm_rocket::Connection;



pub trait CrudService {
    fn create(&self, data: &str) -> Result<String, String>;
    fn read(&self, id: u32) -> Result<String, String>;
    fn update(&self, id: u32, data: &str) -> Result<String, String>;
    fn delete(&self, id: u32) -> Result<String, String>;
}

pub struct CrudServiceStruct {
    db: Connection<'_, _>,
}

impl CrudService for CrudServiceStruct {
    fn create(&self, data: &str) -> Result<String, String> {
        Ok(format!("Created: {}", data))
    }

    fn read(&self, id: u32) -> Result<String, String> {
        Ok(format!("Read: {}", id))
    }

    fn update(&self, id: u32, data: &str) -> Result<String, String> {
        Ok(format!("Updated: {} with {}", id, data))
    }

    fn delete(&self, id: u32) -> Result<String, String> {
        Ok(format!("Deleted: {}", id))
    }
}

