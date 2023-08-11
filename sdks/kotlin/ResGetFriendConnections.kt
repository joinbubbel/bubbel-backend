// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json                    = Json { allowStructuredMapKeys = true }
// val resGetFriendConnections = json.parse(ResGetFriendConnections.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class ResGetFriendConnections (
    val error: GetFriendConnectionsError? = null,
    val res: GetFriendConnectionsOut? = null
)

@Serializable
data class GetFriendConnectionsError (
    val type: Type,
    val ierror: String? = null
)

@Serializable
enum class Type(val value: String) {
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth");
}

@Serializable
data class GetFriendConnectionsOut (
    @SerialName("friend_connections")
    val friendConnections: Map<String, FriendStatus>
)

@Serializable
enum class FriendStatus(val value: String) {
    @SerialName("Full") Full("Full"),
    @SerialName("RecievedPending") RecievedPending("RecievedPending"),
    @SerialName("SentPending") SentPending("SentPending");
}
