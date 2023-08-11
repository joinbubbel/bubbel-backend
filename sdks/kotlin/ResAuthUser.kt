// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json        = Json { allowStructuredMapKeys = true }
// val resAuthUser = json.parse(ResAuthUser.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class ResAuthUser (
    val error: AuthUserError? = null,
    val res: AuthUserOut? = null
)

/**
 * Got an error from a cryptography function. This error should never occur.
 */
@Serializable
data class AuthUserError (
    val type: Type,
    val ierror: String? = null
)

@Serializable
enum class Type(val value: String) {
    @SerialName("Internal") Internal("Internal"),
    @SerialName("InvalidCredentials") InvalidCredentials("InvalidCredentials"),
    @SerialName("InvalidPasswordCryto") InvalidPasswordCryto("InvalidPasswordCryto"),
    @SerialName("UserNotFound") UserNotFound("UserNotFound"),
    @SerialName("UserNotVerified") UserNotVerified("UserNotVerified");
}

@Serializable
data class AuthUserOut (
    val email: String,
    val token: String,
    val username: String
)
