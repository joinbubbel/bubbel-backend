// -- Types --

class InCreateUser: Codable {
	var email: String
	var username: String
	var password: String
}

class DatabaseError: Codable {
	var type: String
	var uerror: String?
}

class CreateUserError: Codable {
	var type: String
	var dberror: DatabaseError?
}

class ResCreateUser: Codable {
	var error: CreateUserError?
}

class InAuthUser: Codable {
	var username: String
	var password: String
}

class AuthUserError: Codable {
	var type: String
    var dberror: DatabaseError?
}

class ResAuthUser: Codable {
	var error: AuthUserError?
	var token: String
	var username: String
	var email: String
}

class InDeauthUser: Codable {
	var token: String
}

// -- API Bridge --

let bubbelBathDev = "https://bubbel-bath.onrender.com/api";

enum BubbelError: Error {
    case fetchError
}

func bubbelApiCreateUser(bath: String, req: InCreateUser) async throws -> ResCreateUser {
    let encoder = JSONEncoder()
    let json = encoder.encode(req)
    let url = URL(string: bath + "/api/create_user")!
    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.httpBody = json
    let (data, response) = await URLSession.shared.data(request)
    guard let httpResponse = response as? HTTPURLResponse,
        httpResponse.statusCode == 200 else {
            throw BubbelError.fetchError
        }
    if let data = data, let dataString = String(data: data, encoding: .utf8) {
        let decoder = JSONDecoder()
        decoder.keyDecodingStrategy = .convertFromSnakeCase
        return try decoder.decode(ResCreateUser.self, from: dataString)
    }
}

func bubbelApiAuthUser(bath: String, req: InAuthUser) async throws -> ResAuthUser {
    let encoder = JSONEncoder()
    let json = encoder.encode(req)
    let url = URL(string: bath + "/api/auth_user")!
    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.httpBody = json
    let (data, response) = await URLSession.shared.data(request)
    guard let httpResponse = response as? HTTPURLResponse,
        httpResponse.statusCode == 200 else {
            throw BubbelError.fetchError
        }
    if let data = data, let dataString = String(data: data, encoding: .utf8) {
        let decoder = JSONDecoder()
        decoder.keyDecodingStrategy = .convertFromSnakeCase
        return try decoder.decode(ResAuthUser.self, from: dataString)
    }
}

func bubbelApiDeauthUser(bath: String, req: InDeauthUser) async throws {
    let encoder = JSONEncoder()
    let json = encoder.encode(req)
    let url = URL(string: bath + "/api/deauth_user")!
    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.httpBody = json
    let (data, response) = await URLSession.shared.data(request)
    guard let httpResponse = response as? HTTPURLResponse,
        httpResponse.statusCode == 200 else {
            throw BubbelError.fetchError
        }
}

