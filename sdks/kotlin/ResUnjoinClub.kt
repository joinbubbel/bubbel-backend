// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json          = Json { allowStructuredMapKeys = true }
// val resUnjoinClub = json.parse(ResUnjoinClub.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class ResUnjoinClub (
    val error: UnjoinClubError? = null,
    val res: JsonObject? = null
)

@Serializable
data class UnjoinClubError (
    val type: Type,
    val ierror: String? = null
)

@Serializable
enum class Type(val value: String) {
    @SerialName("CannotUnjoinAsOwner") CannotUnjoinAsOwner("CannotUnjoinAsOwner"),
    @SerialName("ClubNotFound") ClubNotFound("ClubNotFound"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth");
}
