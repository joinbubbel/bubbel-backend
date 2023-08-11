// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json                         = Json { allowStructuredMapKeys = true }
// val inGetUserProfileWithUsername = json.parse(InGetUserProfileWithUsername.serializer(), jsonString)

package com.example.bubbel.model.backend

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class InGetUserProfileWithUsername (
    val token: String? = null,
    val username: String
)
