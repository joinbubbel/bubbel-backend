// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json              = Json { allowStructuredMapKeys = true }
// val resGetUserProfile = json.parse(ResGetUserProfile.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class ResGetUserProfile (
    val error: GetUserProfileError? = null,
    val res: GetUserProfileOut? = null
)

@Serializable
data class GetUserProfileError (
    val type: Type,
    val ierror: String? = null
)

@Serializable
enum class Type(val value: String) {
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth"),
    @SerialName("UserNotFound") UserNotFound("UserNotFound");
}

@Serializable
data class GetUserProfileOut (
    val banner: String? = null,
    val description: String? = null,

    @SerialName("display_name")
    val displayName: String? = null,

    val name: String? = null,
    val pfp: String? = null
)
