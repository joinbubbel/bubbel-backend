let bubbelBathDev = "http://api.joinbubbel.com"
// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse the JSON, add this file to your project and do:
//
//   let bubbelCodegenOut = try BubbelCodegenOut(json)

import Foundation

// MARK: - BubbelCodegenOut
struct BubbelCodegenOut: Codable {
    let t0: InCreateUser?
    let t1: ResCreateUser?
    let t10: InSetUserProfile?
    let t11: ResSetUserProfile?
    let t12: InGetUserProfile?
    let t13: ResGetUserProfile?
    let t14: InDeleteUser?
    let t15: ResDeleteUser?
    let t16: InCreateClub?
    let t17: ResCreateClub?
    let t18: InGetClubProfile?
    let t19: ResGetClubProfile?
    let t2: InAuthUser?
    let t20: InSetClubProfile?
    let t21: ResSetClubProfile?
    let t22: InDeleteClub?
    let t23: ResDeleteClub?
    let t3: ResAuthUser?
    let t4: InDeauthUser?
    let t5: ResDeauthUser?
    let t6: InVerifyAccount?
    let t7: ResVerifyAccount?
    let t8: InSendVerify?
    let t9: ResSendVerify?
}

// MARK: BubbelCodegenOut convenience initializers and mutators

extension BubbelCodegenOut {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(BubbelCodegenOut.self, from: data)
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
        t0: InCreateUser?? = nil,
        t1: ResCreateUser?? = nil,
        t10: InSetUserProfile?? = nil,
        t11: ResSetUserProfile?? = nil,
        t12: InGetUserProfile?? = nil,
        t13: ResGetUserProfile?? = nil,
        t14: InDeleteUser?? = nil,
        t15: ResDeleteUser?? = nil,
        t16: InCreateClub?? = nil,
        t17: ResCreateClub?? = nil,
        t18: InGetClubProfile?? = nil,
        t19: ResGetClubProfile?? = nil,
        t2: InAuthUser?? = nil,
        t20: InSetClubProfile?? = nil,
        t21: ResSetClubProfile?? = nil,
        t22: InDeleteClub?? = nil,
        t23: ResDeleteClub?? = nil,
        t3: ResAuthUser?? = nil,
        t4: InDeauthUser?? = nil,
        t5: ResDeauthUser?? = nil,
        t6: InVerifyAccount?? = nil,
        t7: ResVerifyAccount?? = nil,
        t8: InSendVerify?? = nil,
        t9: ResSendVerify?? = nil
    ) -> BubbelCodegenOut {
        return BubbelCodegenOut(
            t0: t0 ?? self.t0,
            t1: t1 ?? self.t1,
            t10: t10 ?? self.t10,
            t11: t11 ?? self.t11,
            t12: t12 ?? self.t12,
            t13: t13 ?? self.t13,
            t14: t14 ?? self.t14,
            t15: t15 ?? self.t15,
            t16: t16 ?? self.t16,
            t17: t17 ?? self.t17,
            t18: t18 ?? self.t18,
            t19: t19 ?? self.t19,
            t2: t2 ?? self.t2,
            t20: t20 ?? self.t20,
            t21: t21 ?? self.t21,
            t22: t22 ?? self.t22,
            t23: t23 ?? self.t23,
            t3: t3 ?? self.t3,
            t4: t4 ?? self.t4,
            t5: t5 ?? self.t5,
            t6: t6 ?? self.t6,
            t7: t7 ?? self.t7,
            t8: t8 ?? self.t8,
            t9: t9 ?? self.t9
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

// MARK: - InCreateUser
struct InCreateUser: Codable {
    let email, password, username: String
}

// MARK: InCreateUser convenience initializers and mutators

extension InCreateUser {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InCreateUser.self, from: data)
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

// MARK: - ResCreateUser
struct ResCreateUser: Codable {
    let error: CreateUserError?
    let userID: Int?

    enum CodingKeys: String, CodingKey {
        case error
        case userID = "user_id"
    }
}

// MARK: ResCreateUser convenience initializers and mutators

extension ResCreateUser {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResCreateUser.self, from: data)
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
        error: CreateUserError?? = nil,
        userID: Int?? = nil
    ) -> ResCreateUser {
        return ResCreateUser(
            error: error ?? self.error,
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

/// Email is not valid by backend standards.
///
/// Username is not valid by backend standards.
///
/// Password is not valid by backend standards.
///
/// Got an error from a cryptography function. This error should never occur.
///
/// Email or Username already taken.
// MARK: - CreateUserError
struct CreateUserError: Codable {
    let type: PurpleType
    let ierror: String?
}

// MARK: CreateUserError convenience initializers and mutators

extension CreateUserError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(CreateUserError.self, from: data)
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
        type: PurpleType? = nil,
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

enum PurpleType: String, Codable {
    case emailOrUsernametaken = "EmailOrUsernametaken"
    case invalidEmail = "InvalidEmail"
    case invalidPassword = "InvalidPassword"
    case invalidPasswordCryto = "InvalidPasswordCryto"
    case invalidUsername = "InvalidUsername"
    case typeInternal = "Internal"
}

// MARK: - InSetUserProfile
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

// MARK: InSetUserProfile convenience initializers and mutators

extension InSetUserProfile {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InSetUserProfile.self, from: data)
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

// MARK: - ResSetUserProfile
struct ResSetUserProfile: Codable {
    let error: SetUserProfileError?
}

// MARK: ResSetUserProfile convenience initializers and mutators

extension ResSetUserProfile {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResSetUserProfile.self, from: data)
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

// MARK: - SetUserProfileError
struct SetUserProfileError: Codable {
    let type: FluffyType
    let ierror: String?
}

// MARK: SetUserProfileError convenience initializers and mutators

extension SetUserProfileError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(SetUserProfileError.self, from: data)
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
        type: FluffyType? = nil,
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

enum FluffyType: String, Codable {
    case noAuth = "NoAuth"
    case typeInternal = "Internal"
}

// MARK: - InGetUserProfile
struct InGetUserProfile: Codable {
    let token: String?
    let userID: Int

    enum CodingKeys: String, CodingKey {
        case token
        case userID = "user_id"
    }
}

// MARK: InGetUserProfile convenience initializers and mutators

extension InGetUserProfile {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InGetUserProfile.self, from: data)
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
        token: String?? = nil,
        userID: Int? = nil
    ) -> InGetUserProfile {
        return InGetUserProfile(
            token: token ?? self.token,
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

// MARK: - ResGetUserProfile
struct ResGetUserProfile: Codable {
    let banner, description, displayName: String?
    let error: GetUserProfileError?
    let name, pfp: String?

    enum CodingKeys: String, CodingKey {
        case banner, description
        case displayName = "display_name"
        case error, name, pfp
    }
}

// MARK: ResGetUserProfile convenience initializers and mutators

extension ResGetUserProfile {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResGetUserProfile.self, from: data)
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
        banner: String?? = nil,
        description: String?? = nil,
        displayName: String?? = nil,
        error: GetUserProfileError?? = nil,
        name: String?? = nil,
        pfp: String?? = nil
    ) -> ResGetUserProfile {
        return ResGetUserProfile(
            banner: banner ?? self.banner,
            description: description ?? self.description,
            displayName: displayName ?? self.displayName,
            error: error ?? self.error,
            name: name ?? self.name,
            pfp: pfp ?? self.pfp
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

// MARK: - GetUserProfileError
struct GetUserProfileError: Codable {
    let type: TentacledType
    let ierror: String?
}

// MARK: GetUserProfileError convenience initializers and mutators

extension GetUserProfileError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(GetUserProfileError.self, from: data)
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
        type: TentacledType? = nil,
        ierror: String?? = nil
    ) -> GetUserProfileError {
        return GetUserProfileError(
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

enum TentacledType: String, Codable {
    case noAuth = "NoAuth"
    case typeInternal = "Internal"
    case userNotFound = "UserNotFound"
}

// MARK: - InDeleteUser
struct InDeleteUser: Codable {
    let token: String
}

// MARK: InDeleteUser convenience initializers and mutators

extension InDeleteUser {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InDeleteUser.self, from: data)
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

// MARK: - ResDeleteUser
struct ResDeleteUser: Codable {
    let error: DeleteUserError?
}

// MARK: ResDeleteUser convenience initializers and mutators

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

// MARK: - DeleteUserError
struct DeleteUserError: Codable {
    let type: FluffyType
    let ierror: String?
}

// MARK: DeleteUserError convenience initializers and mutators

extension DeleteUserError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(DeleteUserError.self, from: data)
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
        type: FluffyType? = nil,
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

// MARK: - InCreateClub
struct InCreateClub: Codable {
    let name, token: String
}

// MARK: InCreateClub convenience initializers and mutators

extension InCreateClub {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InCreateClub.self, from: data)
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
        name: String? = nil,
        token: String? = nil
    ) -> InCreateClub {
        return InCreateClub(
            name: name ?? self.name,
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

// MARK: - ResCreateClub
struct ResCreateClub: Codable {
    let clubID: Int?
    let error: CreateClubError?

    enum CodingKeys: String, CodingKey {
        case clubID = "club_id"
        case error
    }
}

// MARK: ResCreateClub convenience initializers and mutators

extension ResCreateClub {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResCreateClub.self, from: data)
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
        clubID: Int?? = nil,
        error: CreateClubError?? = nil
    ) -> ResCreateClub {
        return ResCreateClub(
            clubID: clubID ?? self.clubID,
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

// MARK: - CreateClubError
struct CreateClubError: Codable {
    let type: FluffyType
    let ierror: String?
}

// MARK: CreateClubError convenience initializers and mutators

extension CreateClubError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(CreateClubError.self, from: data)
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
        type: FluffyType? = nil,
        ierror: String?? = nil
    ) -> CreateClubError {
        return CreateClubError(
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

// MARK: - InGetClubProfile
struct InGetClubProfile: Codable {
    let clubID: Int
    let token: String?

    enum CodingKeys: String, CodingKey {
        case clubID = "club_id"
        case token
    }
}

// MARK: InGetClubProfile convenience initializers and mutators

extension InGetClubProfile {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InGetClubProfile.self, from: data)
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
        clubID: Int? = nil,
        token: String?? = nil
    ) -> InGetClubProfile {
        return InGetClubProfile(
            clubID: clubID ?? self.clubID,
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

// MARK: - ResGetClubProfile
struct ResGetClubProfile: Codable {
    let banner, description, displayName: String?
    let error: GetClubProfileError?
    let name: String?
    let owner: Int?
    let pfp: String?

    enum CodingKeys: String, CodingKey {
        case banner, description
        case displayName = "display_name"
        case error, name, owner, pfp
    }
}

// MARK: ResGetClubProfile convenience initializers and mutators

extension ResGetClubProfile {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResGetClubProfile.self, from: data)
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
        banner: String?? = nil,
        description: String?? = nil,
        displayName: String?? = nil,
        error: GetClubProfileError?? = nil,
        name: String?? = nil,
        owner: Int?? = nil,
        pfp: String?? = nil
    ) -> ResGetClubProfile {
        return ResGetClubProfile(
            banner: banner ?? self.banner,
            description: description ?? self.description,
            displayName: displayName ?? self.displayName,
            error: error ?? self.error,
            name: name ?? self.name,
            owner: owner ?? self.owner,
            pfp: pfp ?? self.pfp
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

// MARK: - GetClubProfileError
struct GetClubProfileError: Codable {
    let type: StickyType
    let ierror: String?
}

// MARK: GetClubProfileError convenience initializers and mutators

extension GetClubProfileError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(GetClubProfileError.self, from: data)
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
        type: StickyType? = nil,
        ierror: String?? = nil
    ) -> GetClubProfileError {
        return GetClubProfileError(
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

enum StickyType: String, Codable {
    case clubNotFound = "ClubNotFound"
    case noAuth = "NoAuth"
    case typeInternal = "Internal"
}

// MARK: - InAuthUser
struct InAuthUser: Codable {
    let password, username: String
}

// MARK: InAuthUser convenience initializers and mutators

extension InAuthUser {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InAuthUser.self, from: data)
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

// MARK: - InSetClubProfile
struct InSetClubProfile: Codable {
    let banner: String?
    let clubID: Int
    let description, displayName, name: String?
    let owner: Int?
    let pfp: String?
    let token: String

    enum CodingKeys: String, CodingKey {
        case banner
        case clubID = "club_id"
        case description
        case displayName = "display_name"
        case name, owner, pfp, token
    }
}

// MARK: InSetClubProfile convenience initializers and mutators

extension InSetClubProfile {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InSetClubProfile.self, from: data)
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
        banner: String?? = nil,
        clubID: Int? = nil,
        description: String?? = nil,
        displayName: String?? = nil,
        name: String?? = nil,
        owner: Int?? = nil,
        pfp: String?? = nil,
        token: String? = nil
    ) -> InSetClubProfile {
        return InSetClubProfile(
            banner: banner ?? self.banner,
            clubID: clubID ?? self.clubID,
            description: description ?? self.description,
            displayName: displayName ?? self.displayName,
            name: name ?? self.name,
            owner: owner ?? self.owner,
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

// MARK: - ResSetClubProfile
struct ResSetClubProfile: Codable {
    let error: SetClubProfileError?
}

// MARK: ResSetClubProfile convenience initializers and mutators

extension ResSetClubProfile {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResSetClubProfile.self, from: data)
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
        error: SetClubProfileError?? = nil
    ) -> ResSetClubProfile {
        return ResSetClubProfile(
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

/// The user is not the owner and therefore is not authorized.
// MARK: - SetClubProfileError
struct SetClubProfileError: Codable {
    let type: IndigoType
    let ierror: String?
}

// MARK: SetClubProfileError convenience initializers and mutators

extension SetClubProfileError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(SetClubProfileError.self, from: data)
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
        type: IndigoType? = nil,
        ierror: String?? = nil
    ) -> SetClubProfileError {
        return SetClubProfileError(
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

enum IndigoType: String, Codable {
    case clubNotFound = "ClubNotFound"
    case noAuth = "NoAuth"
    case noAuthOwner = "NoAuthOwner"
    case typeInternal = "Internal"
}

// MARK: - InDeleteClub
struct InDeleteClub: Codable {
    let clubID: Int
    let token: String

    enum CodingKeys: String, CodingKey {
        case clubID = "club_id"
        case token
    }
}

// MARK: InDeleteClub convenience initializers and mutators

extension InDeleteClub {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InDeleteClub.self, from: data)
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
        clubID: Int? = nil,
        token: String? = nil
    ) -> InDeleteClub {
        return InDeleteClub(
            clubID: clubID ?? self.clubID,
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

// MARK: - ResDeleteClub
struct ResDeleteClub: Codable {
    let error: DeleteClubError?
}

// MARK: ResDeleteClub convenience initializers and mutators

extension ResDeleteClub {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResDeleteClub.self, from: data)
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
        error: DeleteClubError?? = nil
    ) -> ResDeleteClub {
        return ResDeleteClub(
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

/// The user is not the owner and therefore is not authorized.
// MARK: - DeleteClubError
struct DeleteClubError: Codable {
    let type: IndigoType
    let ierror: String?
}

// MARK: DeleteClubError convenience initializers and mutators

extension DeleteClubError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(DeleteClubError.self, from: data)
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
        type: IndigoType? = nil,
        ierror: String?? = nil
    ) -> DeleteClubError {
        return DeleteClubError(
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

// MARK: - ResAuthUser
struct ResAuthUser: Codable {
    let email: String?
    let error: AuthUserError?
    let token, username: String?
}

// MARK: ResAuthUser convenience initializers and mutators

extension ResAuthUser {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResAuthUser.self, from: data)
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

/// Got an error from a cryptography function. This error should never occur.
// MARK: - AuthUserError
struct AuthUserError: Codable {
    let type: IndecentType
    let ierror: String?
}

// MARK: AuthUserError convenience initializers and mutators

extension AuthUserError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(AuthUserError.self, from: data)
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
        type: IndecentType? = nil,
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

enum IndecentType: String, Codable {
    case invalidCredentials = "InvalidCredentials"
    case invalidPasswordCryto = "InvalidPasswordCryto"
    case typeInternal = "Internal"
    case userNotFound = "UserNotFound"
    case userNotVerified = "UserNotVerified"
}

// MARK: - InDeauthUser
struct InDeauthUser: Codable {
    let token: String
}

// MARK: InDeauthUser convenience initializers and mutators

extension InDeauthUser {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InDeauthUser.self, from: data)
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

// MARK: - ResDeauthUser
struct ResDeauthUser: Codable {
    let error: JSONNull?
}

// MARK: ResDeauthUser convenience initializers and mutators

extension ResDeauthUser {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResDeauthUser.self, from: data)
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

// MARK: - InVerifyAccount
struct InVerifyAccount: Codable {
    let code: String
}

// MARK: InVerifyAccount convenience initializers and mutators

extension InVerifyAccount {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InVerifyAccount.self, from: data)
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
        code: String? = nil
    ) -> InVerifyAccount {
        return InVerifyAccount(
            code: code ?? self.code
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

// MARK: - ResVerifyAccount
struct ResVerifyAccount: Codable {
    let error: VerifyAccountError?
}

// MARK: ResVerifyAccount convenience initializers and mutators

extension ResVerifyAccount {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResVerifyAccount.self, from: data)
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

/// My favorite error message.
// MARK: - VerifyAccountError
struct VerifyAccountError: Codable {
    let type: HilariousType
    let ierror: String?
}

// MARK: VerifyAccountError convenience initializers and mutators

extension VerifyAccountError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(VerifyAccountError.self, from: data)
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
        type: HilariousType? = nil,
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

enum HilariousType: String, Codable {
    case codeTimedOutOrAlreadyVerifiedOrInvalidCode = "CodeTimedOutOrAlreadyVerifiedOrInvalidCode"
    case typeInternal = "Internal"
}

// MARK: - InSendVerify
struct InSendVerify: Codable {
    let userID: Int

    enum CodingKeys: String, CodingKey {
        case userID = "user_id"
    }
}

// MARK: InSendVerify convenience initializers and mutators

extension InSendVerify {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(InSendVerify.self, from: data)
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
        userID: Int? = nil
    ) -> InSendVerify {
        return InSendVerify(
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

// MARK: - ResSendVerify
struct ResSendVerify: Codable {
    let error: SendVerifyError?
}

// MARK: ResSendVerify convenience initializers and mutators

extension ResSendVerify {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(ResSendVerify.self, from: data)
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
        error: SendVerifyError?? = nil
    ) -> ResSendVerify {
        return ResSendVerify(
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

/// Failed to send the verification message (usually an email error).
// MARK: - SendVerifyError
struct SendVerifyError: Codable {
    let type: AmbitiousType
    let ierror: String?
}

// MARK: SendVerifyError convenience initializers and mutators

extension SendVerifyError {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(SendVerifyError.self, from: data)
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
        type: AmbitiousType? = nil,
        ierror: String?? = nil
    ) -> SendVerifyError {
        return SendVerifyError(
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

enum AmbitiousType: String, Codable {
    case resendTooSoon = "ResendTooSoon"
    case sendVerification = "SendVerification"
    case typeInternal = "Internal"
    case userNotFound = "UserNotFound"
}

// MARK: - Helper functions for creating encoders and decoders

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

// MARK: - Encode/decode helpers

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

func bubbelApiCreateUser(req: InCreateUser) async throws -> ResCreateUser {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/create_user")!
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
func bubbelApiAuthUser(req: InAuthUser) async throws -> ResAuthUser {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/auth_user")!
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
func bubbelApiDeauthUser(req: InDeauthUser) async throws -> ResDeauthUser {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/deauth_user")!
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
func bubbelApiVerifyAccount(req: InVerifyAccount) async throws -> ResVerifyAccount {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/verify_account")!
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
func bubbelApiSendVerify(req: InSendVerify) async throws -> ResSendVerify {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/send_verify")!
            var urlRequest = URLRequest(url: url)
            urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
            urlRequest.httpMethod = "POST"
            urlRequest.httpBody = json
            
            let (data, response) = try await URLSession.shared.data(for: urlRequest)
            let (dataString) = String(data: data, encoding: .utf8) ?? ""
            
            let decoder = JSONDecoder()
            decoder.keyDecodingStrategy = .convertFromSnakeCase
            let result = try decoder.decode(ResSendVerify.self, from: data)
            return result
        }
func bubbelApiSetUserProfile(req: InSetUserProfile) async throws -> ResSetUserProfile {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/set_user_profile")!
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
func bubbelApiGetUserProfile(req: InGetUserProfile) async throws -> ResGetUserProfile {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/get_user_profile")!
            var urlRequest = URLRequest(url: url)
            urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
            urlRequest.httpMethod = "POST"
            urlRequest.httpBody = json
            
            let (data, response) = try await URLSession.shared.data(for: urlRequest)
            let (dataString) = String(data: data, encoding: .utf8) ?? ""
            
            let decoder = JSONDecoder()
            decoder.keyDecodingStrategy = .convertFromSnakeCase
            let result = try decoder.decode(ResGetUserProfile.self, from: data)
            return result
        }
func bubbelApiDeleteUser(req: InDeleteUser) async throws -> ResDeleteUser {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/delete_user")!
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
func bubbelApiCreateClub(req: InCreateClub) async throws -> ResCreateClub {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/create_club")!
            var urlRequest = URLRequest(url: url)
            urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
            urlRequest.httpMethod = "POST"
            urlRequest.httpBody = json
            
            let (data, response) = try await URLSession.shared.data(for: urlRequest)
            let (dataString) = String(data: data, encoding: .utf8) ?? ""
            
            let decoder = JSONDecoder()
            decoder.keyDecodingStrategy = .convertFromSnakeCase
            let result = try decoder.decode(ResCreateClub.self, from: data)
            return result
        }
func bubbelApiGetClubProfile(req: InGetClubProfile) async throws -> ResGetClubProfile {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/get_club_profile")!
            var urlRequest = URLRequest(url: url)
            urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
            urlRequest.httpMethod = "POST"
            urlRequest.httpBody = json
            
            let (data, response) = try await URLSession.shared.data(for: urlRequest)
            let (dataString) = String(data: data, encoding: .utf8) ?? ""
            
            let decoder = JSONDecoder()
            decoder.keyDecodingStrategy = .convertFromSnakeCase
            let result = try decoder.decode(ResGetClubProfile.self, from: data)
            return result
        }
func bubbelApiSetClubProfile(req: InSetClubProfile) async throws -> ResSetClubProfile {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/set_club_profile")!
            var urlRequest = URLRequest(url: url)
            urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
            urlRequest.httpMethod = "POST"
            urlRequest.httpBody = json
            
            let (data, response) = try await URLSession.shared.data(for: urlRequest)
            let (dataString) = String(data: data, encoding: .utf8) ?? ""
            
            let decoder = JSONDecoder()
            decoder.keyDecodingStrategy = .convertFromSnakeCase
            let result = try decoder.decode(ResSetClubProfile.self, from: data)
            return result
        }
func bubbelApiDeleteClub(req: InDeleteClub) async throws -> ResDeleteClub {
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + "/api/delete_club")!
            var urlRequest = URLRequest(url: url)
            urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
            urlRequest.httpMethod = "POST"
            urlRequest.httpBody = json
            
            let (data, response) = try await URLSession.shared.data(for: urlRequest)
            let (dataString) = String(data: data, encoding: .utf8) ?? ""
            
            let decoder = JSONDecoder()
            decoder.keyDecodingStrategy = .convertFromSnakeCase
            let result = try decoder.decode(ResDeleteClub.self, from: data)
            return result
        }