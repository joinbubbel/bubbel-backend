// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json          = Json { allowStructuredMapKeys = true }
// val resDeleteClub = json.parse(ResDeleteClub.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class ResDeleteClub (
    val error: DeleteClubError? = null,
    val res: JsonElement? = null
)

/**
 * The user is not the owner and therefore is not authorized.
 */
@Serializable
data class DeleteClubError (
    val type: Type,
    val ierror: String? = null
)

@Serializable
enum class Type(val value: String) {
    @SerialName("ClubNotFound") ClubNotFound("ClubNotFound"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth"),
    @SerialName("NoAuthOwner") NoAuthOwner("NoAuthOwner");
}
