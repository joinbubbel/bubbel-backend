// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json            = Json { allowStructuredMapKeys = true }
// val resGetUserClubs = json.parse(ResGetUserClubs.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class ResGetUserClubs (
    val error: GetUserClubsError? = null,
    val res: GetUserClubsOut? = null
)

@Serializable
data class GetUserClubsError (
    val ierror: String,
    val type: Type
)

@Serializable
enum class Type(val value: String) {
    @SerialName("Internal") Internal("Internal");
}

@Serializable
data class GetUserClubsOut (
    val clubs: List<Long>
)
