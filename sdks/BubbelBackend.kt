import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.net.HttpURLConnection
import java.net.URL
// To parse the JSON, install kotlin's serialization plugin and do:
//
// val json             = Json { allowStructuredMapKeys = true }
// val bubbelCodegenOut = json.parse(BubbelCodegenOut.serializer(), jsonString)



import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

@Serializable
data class BubbelCodegenOut (
    val t0: InCreateUser? = null,
    val t1: ResCreateUser? = null,
    val t10: InSetUserProfile? = null,
    val t11: ResSetUserProfile? = null,
    val t12: InGetUserProfile? = null,
    val t13: ResGetUserProfile? = null,
    val t14: InDeleteUser? = null,
    val t15: ResDeleteUser? = null,
    val t16: InCreateClub? = null,
    val t17: ResCreateClub? = null,
    val t18: InGetClubProfile? = null,
    val t19: ResGetClubProfile? = null,
    val t2: InAuthUser? = null,
    val t20: InSetClubProfile? = null,
    val t21: ResSetClubProfile? = null,
    val t22: InDeleteClub? = null,
    val t23: ResDeleteClub? = null,
    val t24: InGetUserProfileWithUsername? = null,
    val t25: ResGetUserProfileWithUsername? = null,
    val t26: InAddFriendConnection? = null,
    val t27: ResAddFriendConnection? = null,
    val t28: InGetFriendConnections? = null,
    val t29: ResGetFriendConnections? = null,
    val t3: ResAuthUser? = null,
    val t30: InRemoveFriend? = null,
    val t31: ResRemoveFriend? = null,
    val t32: InJoinClub? = null,
    val t33: ResJoinClub? = null,
    val t34: InUnjoinClub? = null,
    val t35: ResUnjoinClub? = null,
    val t36: InGetClubMembers? = null,
    val t37: ResGetClubMembers? = null,
    val t38: InGetUserClubs? = null,
    val t39: ResGetUserClubs? = null,
    val t4: InDeauthUser? = null,
    val t5: ResDeauthUser? = null,
    val t6: InVerifyAccount? = null,
    val t7: ResVerifyAccount? = null,
    val t8: InSendVerify? = null,
    val t9: ResSendVerify? = null
)

@Serializable
data class InCreateUser (
    val email: String,
    val password: String,
    val username: String
)

@Serializable
data class ResCreateUser (
    val error: CreateUserError? = null,
    val res: CreateUserOut? = null
)

/**
 * Email is not valid by backend standards.
 *
 * Username is not valid by backend standards.
 *
 * Password is not valid by backend standards.
 *
 * Got an error from a cryptography function. This error should never occur.
 *
 * Email or Username already taken.
 */
@Serializable
data class CreateUserError (
    val type: PurpleType,
    val ierror: String? = null
)

@Serializable
enum class PurpleType(val value: String) {
    @SerialName("EmailOrUsernametaken") EmailOrUsernametaken("EmailOrUsernametaken"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("InvalidEmail") InvalidEmail("InvalidEmail"),
    @SerialName("InvalidPassword") InvalidPassword("InvalidPassword"),
    @SerialName("InvalidPasswordCryto") InvalidPasswordCryto("InvalidPasswordCryto"),
    @SerialName("InvalidUsername") InvalidUsername("InvalidUsername");
}

@Serializable
data class CreateUserOut (
    @SerialName("user_id")
    val userID: Long
)

@Serializable
data class InSetUserProfile (
    val banner: String? = null,
    val description: String? = null,

    @SerialName("display_name")
    val displayName: String? = null,

    val name: String? = null,
    val pfp: String? = null,
    val token: String
)

@Serializable
data class ResSetUserProfile (
    val error: SetUserProfileError? = null,
    val res: JsonElement? = null
)

@Serializable
data class SetUserProfileError (
    val type: FluffyType,
    val ierror: String? = null
)

@Serializable
enum class FluffyType(val value: String) {
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth");
}

@Serializable
data class InGetUserProfile (
    val token: String? = null,

    @SerialName("user_id")
    val userID: Long
)

@Serializable
data class ResGetUserProfile (
    val error: GetUserProfileError? = null,
    val res: GetUserProfileOut? = null
)

@Serializable
data class GetUserProfileError (
    val type: TentacledType,
    val ierror: String? = null
)

@Serializable
enum class TentacledType(val value: String) {
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

@Serializable
data class InDeleteUser (
    val token: String
)

@Serializable
data class ResDeleteUser (
    val error: DeleteUserError? = null,
    val res: JsonElement? = null
)

@Serializable
data class DeleteUserError (
    val type: FluffyType,
    val ierror: String? = null
)

@Serializable
data class InCreateClub (
    val name: String,
    val token: String
)

@Serializable
data class ResCreateClub (
    val error: CreateClubError? = null,
    val res: CreateClubOut? = null
)

@Serializable
data class CreateClubError (
    val type: FluffyType,
    val ierror: String? = null
)

@Serializable
data class CreateClubOut (
    @SerialName("club_id")
    val clubID: Long
)

@Serializable
data class InGetClubProfile (
    @SerialName("club_id")
    val clubID: Long,

    val token: String? = null
)

@Serializable
data class ResGetClubProfile (
    val error: GetClubProfileError? = null,
    val res: GetClubProfileOut? = null
)

@Serializable
data class GetClubProfileError (
    val type: StickyType,
    val ierror: String? = null
)

@Serializable
enum class StickyType(val value: String) {
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

@Serializable
data class InAuthUser (
    val password: String,
    val username: String
)

@Serializable
data class InSetClubProfile (
    val banner: String? = null,

    @SerialName("club_id")
    val clubID: Long,

    val description: String? = null,

    @SerialName("display_name")
    val displayName: String? = null,

    val name: String? = null,
    val owner: Long? = null,
    val pfp: String? = null,
    val token: String
)

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
    val type: IndigoType,
    val ierror: String? = null
)

@Serializable
enum class IndigoType(val value: String) {
    @SerialName("ClubNotFound") ClubNotFound("ClubNotFound"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth"),
    @SerialName("NoAuthOwner") NoAuthOwner("NoAuthOwner"),
    @SerialName("SettingOwnerNotSupportedYet") SettingOwnerNotSupportedYet("SettingOwnerNotSupportedYet");
}

@Serializable
data class InDeleteClub (
    @SerialName("club_id")
    val clubID: Long,

    val token: String
)

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
    val type: IndecentType,
    val ierror: String? = null
)

@Serializable
enum class IndecentType(val value: String) {
    @SerialName("ClubNotFound") ClubNotFound("ClubNotFound"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth"),
    @SerialName("NoAuthOwner") NoAuthOwner("NoAuthOwner");
}

@Serializable
data class InGetUserProfileWithUsername (
    val token: String? = null,
    val username: String
)

@Serializable
data class ResGetUserProfileWithUsername (
    val error: GetUserProfileWithUsernameError? = null,
    val res: GetUserProfileWithUsernameOut? = null
)

@Serializable
data class GetUserProfileWithUsernameError (
    val type: TentacledType,
    val ierror: String? = null
)

@Serializable
data class GetUserProfileWithUsernameOut (
    val banner: String? = null,
    val description: String? = null,

    @SerialName("display_name")
    val displayName: String? = null,

    val name: String? = null,
    val pfp: String? = null
)

@Serializable
data class InAddFriendConnection (
    @SerialName("receiver_id")
    val receiverID: Long,

    val token: String
)

@Serializable
data class ResAddFriendConnection (
    val error: AddFriendConnectionError? = null,
    val res: JsonObject? = null
)

@Serializable
data class AddFriendConnectionError (
    val type: HilariousType,
    val ierror: String? = null
)

@Serializable
enum class HilariousType(val value: String) {
    @SerialName("AlreadyConnected") AlreadyConnected("AlreadyConnected"),
    @SerialName("CannotAddSelf") CannotAddSelf("CannotAddSelf"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth");
}

@Serializable
data class InGetFriendConnections (
    val token: String
)

@Serializable
data class ResGetFriendConnections (
    val error: GetFriendConnectionsError? = null,
    val res: GetFriendConnectionsOut? = null
)

@Serializable
data class GetFriendConnectionsError (
    val type: FluffyType,
    val ierror: String? = null
)

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
    val type: AmbitiousType,
    val ierror: String? = null
)

@Serializable
enum class AmbitiousType(val value: String) {
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

@Serializable
data class InRemoveFriend (
    @SerialName("removal_id")
    val removalID: Long,

    val token: String
)

@Serializable
data class ResRemoveFriend (
    val error: RemoveFriendError? = null,
    val res: JsonObject? = null
)

@Serializable
data class RemoveFriendError (
    val type: FluffyType,
    val ierror: String? = null
)

@Serializable
data class InJoinClub (
    @SerialName("club_id")
    val clubID: Long,

    val token: String
)

@Serializable
data class ResJoinClub (
    val error: JoinClubError? = null,
    val res: JsonObject? = null
)

@Serializable
data class JoinClubError (
    val type: CunningType,
    val ierror: String? = null
)

@Serializable
enum class CunningType(val value: String) {
    @SerialName("AlreadyJoined") AlreadyJoined("AlreadyJoined"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth");
}

@Serializable
data class InUnjoinClub (
    @SerialName("club_id")
    val clubID: Long,

    val token: String
)

@Serializable
data class ResUnjoinClub (
    val error: UnjoinClubError? = null,
    val res: JsonObject? = null
)

@Serializable
data class UnjoinClubError (
    val type: MagentaType,
    val ierror: String? = null
)

@Serializable
enum class MagentaType(val value: String) {
    @SerialName("CannotUnjoinAsOwner") CannotUnjoinAsOwner("CannotUnjoinAsOwner"),
    @SerialName("ClubNotFound") ClubNotFound("ClubNotFound"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth");
}

@Serializable
data class InGetClubMembers (
    @SerialName("club_id")
    val clubID: Long
)

@Serializable
data class ResGetClubMembers (
    val error: GetClubMembersError? = null,
    val res: GetClubMembersOut? = null
)

@Serializable
data class GetClubMembersError (
    val ierror: String,
    val type: GetClubMembersErrorType
)

@Serializable
enum class GetClubMembersErrorType(val value: String) {
    @SerialName("Internal") Internal("Internal");
}

@Serializable
data class GetClubMembersOut (
    val users: List<Long>
)

@Serializable
data class InGetUserClubs (
    @SerialName("user_id")
    val userID: Long
)

@Serializable
data class ResGetUserClubs (
    val error: GetUserClubsError? = null,
    val res: GetUserClubsOut? = null
)

@Serializable
data class GetUserClubsError (
    val ierror: String,
    val type: GetClubMembersErrorType
)

@Serializable
data class GetUserClubsOut (
    val clubs: List<Long>
)

@Serializable
data class InDeauthUser (
    val token: String
)

@Serializable
data class ResDeauthUser (
    val error: JsonElement? = null,
    val res: JsonElement? = null
)

@Serializable
data class InVerifyAccount (
    val code: String
)

@Serializable
data class ResVerifyAccount (
    val error: VerifyAccountError? = null,
    val res: JsonElement? = null
)

/**
 * My favorite error message.
 */
@Serializable
data class VerifyAccountError (
    val type: FriskyType,
    val ierror: String? = null
)

@Serializable
enum class FriskyType(val value: String) {
    @SerialName("CodeTimedOutOrAlreadyVerifiedOrInvalidCode") CodeTimedOutOrAlreadyVerifiedOrInvalidCode("CodeTimedOutOrAlreadyVerifiedOrInvalidCode"),
    @SerialName("Internal") Internal("Internal");
}

@Serializable
data class InSendVerify (
    @SerialName("user_id")
    val userID: Long
)

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
    val type: MischievousType,
    val ierror: String? = null
)

@Serializable
enum class MischievousType(val value: String) {
    @SerialName("Internal") Internal("Internal"),
    @SerialName("ResendTooSoon") ResendTooSoon("ResendTooSoon"),
    @SerialName("SendVerification") SendVerification("SendVerification"),
    @SerialName("UserNotFound") UserNotFound("UserNotFound");
}

val BUBBEL_BATH_DEV = "https://api.joinbubbel.com"

suspend fun bubbelApiCreateUser(request: InCreateUser): ResCreateUser = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/create_user")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiAuthUser(request: InAuthUser): ResAuthUser = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/auth_user")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiDeauthUser(request: InDeauthUser): ResDeauthUser = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/deauth_user")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiVerifyAccount(request: InVerifyAccount): ResVerifyAccount = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/verify_account")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiSendVerify(request: InSendVerify): ResSendVerify = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/send_verify")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiSetUserProfile(request: InSetUserProfile): ResSetUserProfile = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/set_user_profile")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiGetUserProfile(request: InGetUserProfile): ResGetUserProfile = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/get_user_profile")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiDeleteUser(request: InDeleteUser): ResDeleteUser = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/delete_user")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiCreateClub(request: InCreateClub): ResCreateClub = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/create_club")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiGetClubProfile(request: InGetClubProfile): ResGetClubProfile = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/get_club_profile")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiSetClubProfile(request: InSetClubProfile): ResSetClubProfile = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/set_club_profile")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiDeleteClub(request: InDeleteClub): ResDeleteClub = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/delete_club")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiGetUserProfileWithUsername(request: InGetUserProfileWithUsername): ResGetUserProfileWithUsername = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/get_user_profile_with_username")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiAddFriendConnection(request: InAddFriendConnection): ResAddFriendConnection = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/add_friend_connection")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiGetFriendConnections(request: InGetFriendConnections): ResGetFriendConnections = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/get_friend_connections")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiRemoveFriend(request: InRemoveFriend): ResRemoveFriend = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/remove_friend")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiJoinClub(request: InJoinClub): ResJoinClub = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/join_club")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiUnjoinClub(request: InUnjoinClub): ResUnjoinClub = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/unjoin_club")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiGetClubMembers(request: InGetClubMembers): ResGetClubMembers = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/get_club_members")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
suspend fun bubbelApiGetUserClubs(request: InGetUserClubs): ResGetUserClubs = withContext(Dispatchers.IO) {
        val encoder = Json { ignoreUnknownKeys = true }
        val json = encoder.encodeToString(request)
        val url = URL("$BUBBEL_BATH_DEV/api/get_user_clubs")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = "POST"
        urlConnection.setRequestProperty("Content-Type", "application/json")
        urlConnection.doOutput = true
        urlConnection.outputStream.use { outputStream ->
            outputStream.write(json.toByteArray())
        }

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {
            val responseString = urlConnection.inputStream.bufferedReader().use { it.readText() }
            val decoder = Json { ignoreUnknownKeys = true }
            try {
                decoder.decodeFromString(responseString)
            } catch (ex: SerializationException) {
                throw Exception("Error decoding response: ${ex.message}")
            }
        } else {
            throw Exception("Error fetching data. Response code: $responseCode")
        }
    }
