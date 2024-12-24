use uuid::Uuid;

pub fn get_guid() -> String {
    let my_uuid = Uuid::new_v4();
    return my_uuid.to_string();
}