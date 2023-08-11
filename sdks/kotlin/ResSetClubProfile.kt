// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json              = Json { allowStructuredMapKeys = true }
// val resSetClubProfile = json.parse(ResSetClubProfile.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class ResSetClubProfile (
    val error: SetClubProfileError? = null,
    val res: JsonObject? = null
)

/**
 * The user is not the owner and therefore is not authorized.
 */
@Serializable
data class SetClubProfileError (
    val type: Type,
    val ierror: String? = null
)

@Serializable
enum class Type(val value: String) {
    @SerialName("ClubNotFound") ClubNotFound("ClubNotFound"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth"),
    @SerialName("NoAuthOwner") NoAuthOwner("NoAuthOwner"),
    @SerialName("SettingOwnerNotSupportedYet") SettingOwnerNotSupportedYet("SettingOwnerNotSupportedYet");
}
