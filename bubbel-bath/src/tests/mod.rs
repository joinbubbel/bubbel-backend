use super::*;

mod test_account_limbo_collect_garbage;
mod test_auth_collect_garbage;
mod test_create_club_profile;
mod test_create_user;
mod test_deauth_user;
mod test_delete_user;
mod test_friends;
mod test_join_clubs;
mod test_resend_verify;
mod test_set_user_profile;

pub fn new_data_state() -> DataState {
    let db_url = "postgresql://postgres:abc@localhost:5432/bubbel-test";

    std::process::Command::new("diesel")
        .arg("database")
        .arg("reset")
        .arg("--database-url")
        .arg(db_url)
        .output()
        .unwrap();

    DataState::new(db_url, "abcdefghijklmnop").unwrap()
}
