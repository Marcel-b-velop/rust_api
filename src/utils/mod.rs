use uuid::Uuid;

pub fn get_guid() -> String {
    let my_uuid = Uuid::new_v4();
    my_uuid.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_guid() {
        let guid = get_guid();

        // Überprüfen, ob der generierte GUID gültig ist
        let parsed_guid = uuid::Uuid::parse_str(&guid);
        assert!(parsed_guid.is_ok(), "Die GUID ist nicht gültig!");
    }
}