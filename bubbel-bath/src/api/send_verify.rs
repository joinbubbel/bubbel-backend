use super::*;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct SendVerify {
    pub user_id: UserId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum SendVerifyError {
    UserNotFound,
    ResendTooSoon,
    /// Failed to send the verification message (usually an email error).
    SendVerification,
    Internal { ierror: String },
}

const VERIFICATION_RESEND_MIN_TIME: Duration = Duration::from_secs(10);

pub fn send_verify(
    acc_limbo: &mut AccountLimboState,
    req: SendVerify,
) -> Result<(), SendVerifyError> {
    send_verify_with_resend_time(acc_limbo, req, VERIFICATION_RESEND_MIN_TIME)
}

pub fn send_verify_with_resend_time(
    acc_limbo: &mut AccountLimboState,
    req: SendVerify,
    resend_time: Duration,
) -> Result<(), SendVerifyError> {
    if let Some(time) = acc_limbo.get_time(&req.user_id) {
        let now = SystemTime::now();
        ((time.elapsed().unwrap() - now.elapsed().unwrap()) > resend_time)
            .then_some(())
            .ok_or(SendVerifyError::ResendTooSoon)?;
    }
    acc_limbo.push_user(req.user_id);

    Ok(())
}
