use super::*;

mod test_create_user;

pub fn new_data_state() -> DataState {
    let db_url = "postgresql://postgres:abc@localhost:5432";

    std::process::Command::new("diesel")
        .arg("migration")
        .arg("redo")
        .arg("--database-url")
        .arg(db_url)
        .spawn()
        .unwrap();

    DataState::new(db_url, "yummy").unwrap()
}
