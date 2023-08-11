// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json              = Json { allowStructuredMapKeys = true }
// val resGetClubProfile = json.parse(ResGetClubProfile.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class ResGetClubProfile (
    val error: GetClubProfileError? = null,
    val res: GetClubProfileOut? = null
)

@Serializable
data class GetClubProfileError (
    val type: Type,
    val ierror: String? = null
)

@Serializable
enum class Type(val value: String) {
    @SerialName("ClubNotFound") ClubNotFound("ClubNotFound"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth");
}

@Serializable
data class GetClubProfileOut (
    val banner: String? = null,
    val description: String? = null,

    @SerialName("display_name")
    val displayName: String? = null,

    val name: String,
    val owner: Long,
    val pfp: String? = null
)
