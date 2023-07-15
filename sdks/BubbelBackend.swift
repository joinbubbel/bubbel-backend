import Foundation

// -- Types --

struct InCreateUser: Codable {
    let email, password, username: String
}

extension InCreateUser {
    func with(
        email: String? = nil,
        password: String? = nil,
        username: String? = nil
    ) -> InCreateUser {
        return InCreateUser(
            email: email ?? self.email,
            password: password ?? self.password,
            username: username ?? self.username
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct ResCreateUser: Codable {
    let error: CreateUserError?
}

extension ResCreateUser {
    func with(
        error: CreateUserError?? = nil
    ) -> ResCreateUser {
        return ResCreateUser(
            error: error ?? self.error
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct CreateUserError: Codable {
    let type: CreateUserErrorType
    let ierror: String?
}

extension CreateUserError {
    func with(
        type: CreateUserErrorType? = nil,
        ierror: String?? = nil
    ) -> CreateUserError {
        return CreateUserError(
            type: type ?? self.type,
            ierror: ierror ?? self.ierror
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

enum CreateUserErrorType: String, Codable {
    case emailOrUsernametaken = "EmailOrUsernametaken"
    case invalidEmail = "InvalidEmail"
    case invalidPassword = "InvalidPassword"
    case invalidPasswordCryto = "InvalidPasswordCryto"
    case invalidUsername = "InvalidUsername"
    case sendVerification = "SendVerification"
    case typeInternal = "Internal"
}

struct InAuthUser: Codable {
    let password, username: String
}

extension InAuthUser {
    func with(
        password: String? = nil,
        username: String? = nil
    ) -> InAuthUser {
        return InAuthUser(
            password: password ?? self.password,
            username: username ?? self.username
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct ResAuthUser: Codable {
    let email: String?
    let error: AuthUserError?
    let token, username: String?
}

extension ResAuthUser {
    func with(
        email: String?? = nil,
        error: AuthUserError?? = nil,
        token: String?? = nil,
        username: String?? = nil
    ) -> ResAuthUser {
        return ResAuthUser(
            email: email ?? self.email,
            error: error ?? self.error,
            token: token ?? self.token,
            username: username ?? self.username
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct AuthUserError: Codable {
    let type: AuthUserErrorType
    let ierror: String?
}

extension AuthUserError {
    func with(
        type: AuthUserErrorType? = nil,
        ierror: String?? = nil
    ) -> AuthUserError {
        return AuthUserError(
            type: type ?? self.type,
            ierror: ierror ?? self.ierror
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

enum AuthUserErrorType: String, Codable {
    case invalidCredentials = "InvalidCredentials"
    case invalidPasswordCryto = "InvalidPasswordCryto"
    case typeInternal = "Internal"
    case userNotFound = "UserNotFound"
    case userNotVerified = "UserNotVerified"
}

struct InDeauthUser: Codable {
    let token: String
}

extension InDeauthUser {
    func with(
        token: String? = nil
    ) -> InDeauthUser {
        return InDeauthUser(
            token: token ?? self.token
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct ResDeauthUser: Codable {
    let error: JSONNull?
}

extension ResDeauthUser {
    func with(
        error: JSONNull?? = nil
    ) -> ResDeauthUser {
        return ResDeauthUser(
            error: error ?? self.error
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

class JSONNull: Codable, Hashable {

    public static func == (lhs: JSONNull, rhs: JSONNull) -> Bool {
        return true
    }

    public var hashValue: Int {
        return 0
    }

    public init() {}

    public required init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if !container.decodeNil() {
            throw DecodingError.typeMismatch(JSONNull.self, DecodingError.Context(codingPath: decoder.codingPath, debugDescription: "Wrong type for JSONNull"))
        }
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        try container.encodeNil()
    }
}

struct InVerifyAccount: Codable {
    let code: String
    let userID: Int

    enum CodingKeys: String, CodingKey {
        case code
        case userID = "user_id"
    }
}

extension InVerifyAccount {
    func with(
        code: String? = nil,
        userID: Int? = nil
    ) -> InVerifyAccount {
        return InVerifyAccount(
            code: code ?? self.code,
            userID: userID ?? self.userID
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct ResVerifyAccount: Codable {
    let error: VerifyAccountError?
}

extension ResVerifyAccount {
    func with(
        error: VerifyAccountError?? = nil
    ) -> ResVerifyAccount {
        return ResVerifyAccount(
            error: error ?? self.error
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct VerifyAccountError: Codable {
    let type: VerifyAccountErrorType
    let ierror: String?
}

extension VerifyAccountError {
    func with(
        type: VerifyAccountErrorType? = nil,
        ierror: String?? = nil
    ) -> VerifyAccountError {
        return VerifyAccountError(
            type: type ?? self.type,
            ierror: ierror ?? self.ierror
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

enum VerifyAccountErrorType: String, Codable {
    case codeTimedOutOrInvalidUser = "CodeTimedOutOrInvalidUser"
    case invalidCode = "InvalidCode"
    case typeInternal = "Internal"
}

struct InSetUserProfile: Codable {
    let banner, description, displayName, name: String?
    let pfp: String?
    let token: String

    enum CodingKeys: String, CodingKey {
        case banner, description
        case displayName = "display_name"
        case name, pfp, token
    }
}

extension InSetUserProfile {
    func with(
        banner: String?? = nil,
        description: String?? = nil,
        displayName: String?? = nil,
        name: String?? = nil,
        pfp: String?? = nil,
        token: String? = nil
    ) -> InSetUserProfile {
        return InSetUserProfile(
            banner: banner ?? self.banner,
            description: description ?? self.description,
            displayName: displayName ?? self.displayName,
            name: name ?? self.name,
            pfp: pfp ?? self.pfp,
            token: token ?? self.token
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct ResSetUserProfile: Codable {
    let error: SetUserProfileError?
}

extension ResSetUserProfile {
    func with(
        error: SetUserProfileError?? = nil
    ) -> ResSetUserProfile {
        return ResSetUserProfile(
            error: error ?? self.error
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct SetUserProfileError: Codable {
    let type: SetUserProfileErrorType
    let ierror: String?
}

extension SetUserProfileError {
    func with(
        type: SetUserProfileErrorType? = nil,
        ierror: String?? = nil
    ) -> SetUserProfileError {
        return SetUserProfileError(
            type: type ?? self.type,
            ierror: ierror ?? self.ierror
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

enum SetUserProfileErrorType: String, Codable {
    case noAuth = "NoAuth"
    case typeInternal = "Internal"
}

struct InDeleteUser: Codable {
    let token: String
}

extension InDeleteUser {
    func with(
        token: String? = nil
    ) -> InDeleteUser {
        return InDeleteUser(
            token: token ?? self.token
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct ResDeleteUser: Codable {
    let error: DeleteUserError?
}

extension ResDeleteUser {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResDeleteUser.self, from: data)
    }

    init(_ json: String, using encoding: String.Encoding = .utf8) throws {
        guard let data = json.data(using: encoding) else {
            throw NSError(domain: "JSONDecoding", code: 0, userInfo: nil)
        }
        try self.init(data: data)
    }

    init(fromURL url: URL) throws {
        try self.init(data: try Data(contentsOf: url))
    }

    func with(
        error: DeleteUserError?? = nil
    ) -> ResDeleteUser {
        return ResDeleteUser(
            error: error ?? self.error
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

struct DeleteUserError: Codable {
    let type: DeleteUserErrorType
    let ierror: String?
}

extension DeleteUserError {
    func with(
        type: DeleteUserErrorType? = nil,
        ierror: String?? = nil
    ) -> DeleteUserError {
        return DeleteUserError(
            type: type ?? self.type,
            ierror: ierror ?? self.ierror
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

enum DeleteUserErrorType: String, Codable {
    case noAuth = "NoAuth"
    case typeInternal = "Internal"
}

func newJSONDecoder() -> JSONDecoder {
    let decoder = JSONDecoder()
    if #available(iOS 10.0, OSX 10.12, tvOS 10.0, watchOS 3.0, *) {
        decoder.dateDecodingStrategy = .iso8601
    }
    return decoder
}

func newJSONEncoder() -> JSONEncoder {
    let encoder = JSONEncoder()
    if #available(iOS 10.0, OSX 10.12, tvOS 10.0, watchOS 3.0, *) {
        encoder.dateEncodingStrategy = .iso8601
    }
    return encoder
}

// -- API Bridge --

let bubbelBathDev = "https://bubbel-bath.onrender.com";

func bubbelApiCreateUser(bath: String, req: InCreateUser) async throws -> ResCreateUser {
    let jsonString = try req.jsonString()
    
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
    let jsonString = try req.jsonString()
    
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

func bubbelApiDeauthUser(bath: String, req: InDeauthUser) async throws -> ResDeauthUser {
    let jsonString = try req.jsonString()
    
    let url = URL(string: bath + "/api/deauth_user")!
    var urlRequest = URLRequest(url: url)
    urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
    urlRequest.httpMethod = "POST"
    urlRequest.httpBody = json
    
    let (data, response) = try await URLSession.shared.data(for: urlRequest)
    let (dataString) = String(data: data, encoding: .utf8) ?? ""
    
    let decoder = JSONDecoder()
    decoder.keyDecodingStrategy = .convertFromSnakeCase
    let result = try decoder.decode(ResDeauthUser.self, from: data)
    return result
}

func bubbelApiVerifyAccount(bath: String, req: InVerifyAccount) async throws -> ResVerifyAccount {
    let jsonString = try req.jsonString()
    
    let url = URL(string: bath + "/api/verify_account")!
    var urlRequest = URLRequest(url: url)
    urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
    urlRequest.httpMethod = "POST"
    urlRequest.httpBody = json
    
    let (data, response) = try await URLSession.shared.data(for: urlRequest)
    let (dataString) = String(data: data, encoding: .utf8) ?? ""
    
    let decoder = JSONDecoder()
    decoder.keyDecodingStrategy = .convertFromSnakeCase
    let result = try decoder.decode(ResVerifyAccount.self, from: data)
    return result
}

func bubbelApiSetUserProfile(bath: String, req: InSetUserProfile) async throws -> ResSetUserProfile {
    let jsonString = try req.jsonString()
    
    let url = URL(string: bath + "/api/set_user_profile")!
    var urlRequest = URLRequest(url: url)
    urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
    urlRequest.httpMethod = "POST"
    urlRequest.httpBody = json
    
    let (data, response) = try await URLSession.shared.data(for: urlRequest)
    let (dataString) = String(data: data, encoding: .utf8) ?? ""
    
    let decoder = JSONDecoder()
    decoder.keyDecodingStrategy = .convertFromSnakeCase
    let result = try decoder.decode(ResSetUserProfile.self, from: data)
    return result
}

func bubbelApiDeleteUser(bath: String, req: InDeleteUser) async throws -> ResDeleteUser {
    let jsonString = try req.jsonString()
    
    let url = URL(string: bath + "/api/delete_user")!
    var urlRequest = URLRequest(url: url)
    urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
    urlRequest.httpMethod = "POST"
    urlRequest.httpBody = json
    
    let (data, response) = try await URLSession.shared.data(for: urlRequest)
    let (dataString) = String(data: data, encoding: .utf8) ?? ""
    
    let decoder = JSONDecoder()
    decoder.keyDecodingStrategy = .convertFromSnakeCase
    let result = try decoder.decode(ResDeleteUser.self, from: data)
    return result
}
