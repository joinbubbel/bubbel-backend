// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json          = Json { allowStructuredMapKeys = true }
// val resSendVerify = json.parse(ResSendVerify.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class ResSendVerify (
    val error: SendVerifyError? = null,
    val res: JsonElement? = null
)

/**
 * Failed to send the verification message (usually an email error).
 */
@Serializable
data class SendVerifyError (
    val type: Type,
    val ierror: String? = null
)

@Serializable
enum class Type(val value: String) {
    @SerialName("Internal") Internal("Internal"),
    @SerialName("ResendTooSoon") ResendTooSoon("ResendTooSoon"),
    @SerialName("SendVerification") SendVerification("SendVerification"),
    @SerialName("UserNotFound") UserNotFound("UserNotFound");
}
