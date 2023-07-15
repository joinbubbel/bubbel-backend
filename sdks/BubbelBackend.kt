import kotlinx.serialization.*
import kotlinx.serialization.json.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*
import java.net.HttpURLConnection
import java.net.URL

// -- Types --

@Serializable
data class InCreateUser (
    val email: String,
    val password: String,
    val username: String
)

@Serializable
data class ResCreateUser (
    val error: CreateUserError? = null
)

@Serializable
data class CreateUserError (
    val type: CreateUserErrorType,
    val ierror: String? = null
)

@Serializable
enum class CreateUserErrorType(val value: String) {
    @SerialName("EmailOrUsernametaken") EmailOrUsernametaken("EmailOrUsernametaken"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("InvalidEmail") InvalidEmail("InvalidEmail"),
    @SerialName("InvalidPassword") InvalidPassword("InvalidPassword"),
    @SerialName("InvalidPasswordCryto") InvalidPasswordCryto("InvalidPasswordCryto"),
    @SerialName("InvalidUsername") InvalidUsername("InvalidUsername"),
    @SerialName("SendVerification") SendVerification("SendVerification");
}

@Serializable
data class InAuthUser (
    val password: String,
    val username: String
)

@Serializable
data class ResAuthUser (
    val email: String? = null,
    val error: AuthUserError? = null,
    val token: String? = null,
    val username: String? = null
)

@Serializable
data class AuthUserError (
    val type: AuthUserErrorType,
    val ierror: String? = null
)

@Serializable
enum class AuthUserErrorType(val value: String) {
    @SerialName("Internal") Internal("Internal"),
    @SerialName("InvalidCredentials") InvalidCredentials("InvalidCredentials"),
    @SerialName("InvalidPasswordCryto") InvalidPasswordCryto("InvalidPasswordCryto"),
    @SerialName("UserNotFound") UserNotFound("UserNotFound"),
    @SerialName("UserNotVerified") UserNotVerified("UserNotVerified");
}

@Serializable
data class InDeauthUser (
    val token: String
)

@Serializable
data class ResDeauthUser (
    val error: JsonElement? = null
)

@Serializable
data class InVerifyAccount (
    val code: String,

    @SerialName("user_id")
    val userID: Long
)

@Serializable
data class ResVerifyAccount (
    val error: VerifyAccountError? = null
)

@Serializable
data class VerifyAccountError (
    val type: VerifyAccountErrorType,
    val ierror: String? = null
)

@Serializable
enum class VerifyAccountErrorType(val value: String) {
    @SerialName("CodeTimedOutOrInvalidUser") CodeTimedOutOrInvalidUser("CodeTimedOutOrInvalidUser"),
    @SerialName("Internal") Internal("Internal"),
    @SerialName("InvalidCode") InvalidCode("InvalidCode");
}

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
    val error: SetUserProfileError? = null
)

@Serializable
data class SetUserProfileError (
    val type: SetUserProfileErrorType,
    val ierror: String? = null
)

@Serializable
enum class SetUserProfileErrorType(val value: String) {
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth");
}

@Serializable
data class InDeleteUser (
    val token: String
)

@Serializable
data class ResDeleteUser (
    val error: DeleteUserError? = null
)

@Serializable
data class DeleteUserError (
    val type: DeleteUserErrorType,
    val ierror: String? = null
)

@Serializable
enum class DeleteUserErrorType(val value: String) {
    @SerialName("Internal") Internal("Internal"),
    @SerialName("NoAuth") NoAuth("NoAuth");
}

// -- API Bridge --

const val BUBBEL_BATH_DEV = "https://bubbel-bath.onrender.com";

suspend fun bubbelApiCreateUser(request: InCreateUser): ResCreateUser = withContext(Dispatchers.IO) {
    val encoder = Json { ignoreUnknownKeys = true }
    val json = encoder.encodeToString(request)
    println(json)
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
        println(responseString)
        val decoder = Json { ignoreUnknownKeys = true }
        try {
            decoder.decodeFromString(responseString)
        } catch (ex: SerializationException) {
            throw FetchErrorException("Error decoding response: ${ex.message}")
        }
    } else {
        throw FetchErrorException("Error fetching data. Response code: $responseCode")
    }
}

suspend fun bubbelApiAuthUser(request: InAuthUser): ResAuthUser = withContext(Dispatchers.IO) {
    val encoder = Json { ignoreUnknownKeys = true }
    val json = encoder.encodeToString(request)
    println(json)
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
        println(responseString)
        val decoder = Json { ignoreUnknownKeys = true }
        try {
            decoder.decodeFromString(responseString)
        } catch (ex: SerializationException) {
            throw FetchErrorException("Error decoding response: ${ex.message}")
        }
    } else {
        throw FetchErrorException("Error fetching data. Response code: $responseCode")
    }
}

suspend fun bubbelApiDeauthUser(request: InDeauthUser): ResDeauthUser = withContext(Dispatchers.IO) {
    val encoder = Json { ignoreUnknownKeys = true }
    val json = encoder.encodeToString(request)
    println(json)
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
        println(responseString)
        val decoder = Json { ignoreUnknownKeys = true }
        try {
            decoder.decodeFromString(responseString)
        } catch (ex: SerializationException) {
            throw FetchErrorException("Error decoding response: ${ex.message}")
        }
    } else {
        throw FetchErrorException("Error fetching data. Response code: $responseCode")
    }
}

suspend fun bubbelApiVerifyAccount(request: InVerifyAccount): ResVerifyAccount = withContext(Dispatchers.IO) {
    val encoder = Json { ignoreUnknownKeys = true }
    val json = encoder.encodeToString(request)
    println(json)
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
        println(responseString)
        val decoder = Json { ignoreUnknownKeys = true }
        try {
            decoder.decodeFromString(responseString)
        } catch (ex: SerializationException) {
            throw FetchErrorException("Error decoding response: ${ex.message}")
        }
    } else {
        throw FetchErrorException("Error fetching data. Response code: $responseCode")
    }
}

suspend fun bubbelApiSetUserProfile(request: InSetUserProfile): ResSetUserProfile = withContext(Dispatchers.IO) {
    val encoder = Json { ignoreUnknownKeys = true }
    val json = encoder.encodeToString(request)
    println(json)
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
        println(responseString)
        val decoder = Json { ignoreUnknownKeys = true }
        try {
            decoder.decodeFromString(responseString)
        } catch (ex: SerializationException) {
            throw FetchErrorException("Error decoding response: ${ex.message}")
        }
    } else {
        throw FetchErrorException("Error fetching data. Response code: $responseCode")
    }
}

suspend fun bubbelApiDeleteUser(request: InDeleteUser): ResDeleteUser = withContext(Dispatchers.IO) {
    val encoder = Json { ignoreUnknownKeys = true }
    val json = encoder.encodeToString(request)
    println(json)
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
        println(responseString)
        val decoder = Json { ignoreUnknownKeys = true }
        try {
            decoder.decodeFromString(responseString)
        } catch (ex: SerializationException) {
            throw FetchErrorException("Error decoding response: ${ex.message}")
        }
    } else {
        throw FetchErrorException("Error fetching data. Response code: $responseCode")
    }
}
