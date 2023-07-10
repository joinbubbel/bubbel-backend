// -- Types --

class InCreateUser: Codable {
	var email: String
	var username: String
	var password: String
}

class CreateUserError: Codable {
	var type: String
	var ierror: String?
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
    var ierror: String?
}

class ResAuthUser: Codable {
	var error: AuthUserError?
	var token: String?
	var username: String?
	var email: String?
}

class InDeauthUser: Codable {
	var token: String
}

// -- API Bridge --

let bubbelBathDev = "https://bubbel-bath.onrender.com";

enum BubbelError: Error {
    case fetchError
}

func bubbelApiCreateUser(bath: String, req: InCreateUser) async throws -> ResCreateUser {
    let encoder = JSONEncoder()
    let json = try encoder.encode(req)
    let jsonString = String(data: json, encoding: .utf8) ?? ""
    
    let url = URL(string: bath + "/api/create_user")!
    var urlRequest = URLRequest(url: url)
    urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
    urlRequest.httpMethod = "POST"
    urlRequest.httpBody = json
    
    let (data, response) = try await URLSession.shared.data(for: urlRequest)
    let (dataString) = String(data: data, encoding: .utf8) ?? ""
    
    let decoder = JSONDecoder()
    decoder.keyDecodingStrategy = .convertFromSnakeCase
    let result = try decoder.decode(ResCreateUser.self, from: data)
    return result
}

func bubbelApiAuthUser(bath: String, req: InAuthUser) async throws -> ResAuthUser {
    let encoder = JSONEncoder()
    let json = try encoder.encode(req)
    let jsonString = String(data: json, encoding: .utf8) ?? ""
    
    let url = URL(string: bath + "/api/auth_user")!
    var urlRequest = URLRequest(url: url)
    urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
    urlRequest.httpMethod = "POST"
    urlRequest.httpBody = json
    
    let (data, response) = try await URLSession.shared.data(for: urlRequest)
    let (dataString) = String(data: data, encoding: .utf8) ?? ""
    
    let decoder = JSONDecoder()
    decoder.keyDecodingStrategy = .convertFromSnakeCase
    let result = try decoder.decode(ResAuthUser.self, from: data)
    return result
}

func bubbelApiDeauthUser(bath: String, req: InDeauthUser) async throws {
    let encoder = JSONEncoder()
    let json = try encoder.encode(req)
    let jsonString = String(data: json, encoding: .utf8) ?? ""
    
    let url = URL(string: bath + "/api/deauth_user")!
    var urlRequest = URLRequest(url: url)
    urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
    urlRequest.httpMethod = "POST"
    urlRequest.httpBody = json
    
    try await URLSession.shared.data(for: urlRequest)
}

