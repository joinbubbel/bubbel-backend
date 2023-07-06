import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.encodeToString
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse

// -- Types --

@Serializable
class InCreateUser {
	val email: String,
	val username: String,
	val password: String,
}

@Serializable
class ResCreateUser {
	val error: String,
}

@Serializable
class InAuthUser {
	val username: String,
	val password: String,
}

@Serializable
class ResAuthUser {
	val error: String,
	val token: String,
	val username: String,
	val email: String,
}

@Serializable
class InDeauthUser {
	val token: String,
}

// -- API Bridge --

const val BUBBEL_BATH_DEV = "https://bubbel-bath.onrender.com/api";

fun bubbelApiCreateUser(bath: String, req: InCreateUser): ResCreateUser {
    val client = HttpClient.newBuilder().build();
    val json = Json.encodeToString(req)
    val request = HttpRequest.newBuilder()
        .uri(URI.create(bath + "/create_user"))
        .POST(HttpRequest.BodyPublishers.ofString(json))
        .build();
        
    val response = client.send(request, HttpResponse.BodyHandlers.ofString());
    return Json.decodeFromString<ResCreateUser>(response.body());
}

fun bubbelApiAuthUser(bath: String, req: InAuthUser): ResAuthUser {
    val client = HttpClient.newBuilder().build();
    val json = Json.encodeToString(req)
    val request = HttpRequest.newBuilder()
        .uri(URI.create(bath + "/auth_user"))
        .POST(HttpRequest.BodyPublishers.ofString(json))
        .build();
        
    val response = client.send(request, HttpResponse.BodyHandlers.ofString());
    return Json.decodeFromString<ResAuthUser>(response.body());
}

fun bubbelApiDeauthUser(bath: String, req: InDeauthUser) {
    val client = HttpClient.newBuilder().build();
    val json = Json.encodeToString(req)
    val request = HttpRequest.newBuilder()
        .uri(URI.create(bath + "/deauth_user"))
        .POST(HttpRequest.BodyPublishers.ofString(json))
        .build();
        
    val response = client.send(request, HttpResponse.BodyHandlers.ofString());
}

