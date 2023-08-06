const bubbelBathDev = 'https://api.joinbubbel.com';export interface BubbelCodegenOut {
    t0?:  InCreateUser;
    t1?:  ResCreateUser;
    t10?: InSetUserProfile;
    t11?: ResSetUserProfile;
    t12?: InGetUserProfile;
    t13?: ResGetUserProfile;
    t14?: InDeleteUser;
    t15?: ResDeleteUser;
    t16?: InCreateClub;
    t17?: ResCreateClub;
    t18?: InGetClubProfile;
    t19?: ResGetClubProfile;
    t2?:  InAuthUser;
    t20?: InSetClubProfile;
    t21?: ResSetClubProfile;
    t22?: InDeleteClub;
    t23?: ResDeleteClub;
    t24?: InGetUserProfileWithUsername;
    t25?: ResGetUserProfileWithUsername;
    t26?: InAddFriendConnection;
    t27?: ResAddFriendConnection;
    t28?: InGetFriendConnections;
    t29?: ResGetFriendConnections;
    t3?:  ResAuthUser;
    t30?: InRemoveFriend;
    t31?: ResRemoveFriend;
    t32?: InJoinClub;
    t33?: ResJoinClub;
    t34?: InUnjoinClub;
    t35?: ResUnjoinClub;
    t36?: InGetClubMembers;
    t37?: ResGetClubMembers;
    t38?: InGetUserClubs;
    t39?: ResGetUserClubs;
    t4?:  InDeauthUser;
    t5?:  ResDeauthUser;
    t6?:  InVerifyAccount;
    t7?:  ResVerifyAccount;
    t8?:  InSendVerify;
    t9?:  ResSendVerify;
    [property: string]: any;
}

export interface InCreateUser {
    email:    string;
    password: string;
    username: string;
    [property: string]: any;
}

export interface ResCreateUser {
    error?: null | CreateUserError;
    res?:   null | CreateUserOut;
    [property: string]: any;
}

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
export interface CreateUserError {
    type:    PurpleType;
    ierror?: string;
    [property: string]: any;
}

export enum PurpleType {
    EmailOrUsernametaken = "EmailOrUsernametaken",
    Internal = "Internal",
    InvalidEmail = "InvalidEmail",
    InvalidPassword = "InvalidPassword",
    InvalidPasswordCryto = "InvalidPasswordCryto",
    InvalidUsername = "InvalidUsername",
}

export interface CreateUserOut {
    user_id: number;
    [property: string]: any;
}

export interface InSetUserProfile {
    banner?:       null | string;
    description?:  null | string;
    display_name?: null | string;
    name?:         null | string;
    pfp?:          null | string;
    token:         string;
    [property: string]: any;
}

export interface ResSetUserProfile {
    error?: null | SetUserProfileError;
    res?:   null;
    [property: string]: any;
}

export interface SetUserProfileError {
    type:    FluffyType;
    ierror?: string;
    [property: string]: any;
}

export enum FluffyType {
    Internal = "Internal",
    NoAuth = "NoAuth",
}

export interface InGetUserProfile {
    token?:  null | string;
    user_id: number;
    [property: string]: any;
}

export interface ResGetUserProfile {
    error?: null | GetUserProfileError;
    res?:   null | GetUserProfileOut;
    [property: string]: any;
}

export interface GetUserProfileError {
    type:    TentacledType;
    ierror?: string;
    [property: string]: any;
}

export enum TentacledType {
    Internal = "Internal",
    NoAuth = "NoAuth",
    UserNotFound = "UserNotFound",
}

export interface GetUserProfileOut {
    banner?:       null | string;
    description?:  null | string;
    display_name?: null | string;
    name?:         null | string;
    pfp?:          null | string;
    [property: string]: any;
}

export interface InDeleteUser {
    token: string;
    [property: string]: any;
}

export interface ResDeleteUser {
    error?: null | DeleteUserError;
    res?:   null;
    [property: string]: any;
}

export interface DeleteUserError {
    type:    FluffyType;
    ierror?: string;
    [property: string]: any;
}

export interface InCreateClub {
    name:  string;
    token: string;
    [property: string]: any;
}

export interface ResCreateClub {
    error?: null | CreateClubError;
    res?:   null | CreateClubOut;
    [property: string]: any;
}

export interface CreateClubError {
    type:    FluffyType;
    ierror?: string;
    [property: string]: any;
}

export interface CreateClubOut {
    club_id: number;
    [property: string]: any;
}

export interface InGetClubProfile {
    club_id: number;
    token?:  null | string;
    [property: string]: any;
}

export interface ResGetClubProfile {
    error?: null | GetClubProfileError;
    res?:   null | GetClubProfileOut;
    [property: string]: any;
}

export interface GetClubProfileError {
    type:    StickyType;
    ierror?: string;
    [property: string]: any;
}

export enum StickyType {
    ClubNotFound = "ClubNotFound",
    Internal = "Internal",
    NoAuth = "NoAuth",
}

export interface GetClubProfileOut {
    banner?:       null | string;
    description?:  null | string;
    display_name?: null | string;
    name:          string;
    owner:         number;
    pfp?:          null | string;
    [property: string]: any;
}

export interface InAuthUser {
    password: string;
    username: string;
    [property: string]: any;
}

export interface InSetClubProfile {
    banner?:       null | string;
    club_id:       number;
    description?:  null | string;
    display_name?: null | string;
    name?:         null | string;
    owner?:        number | null;
    pfp?:          null | string;
    token:         string;
    [property: string]: any;
}

export interface ResSetClubProfile {
    error?: null | SetClubProfileError;
    res?:   { [key: string]: any } | null;
    [property: string]: any;
}

/**
 * The user is not the owner and therefore is not authorized.
 */
export interface SetClubProfileError {
    type:    IndigoType;
    ierror?: string;
    [property: string]: any;
}

export enum IndigoType {
    ClubNotFound = "ClubNotFound",
    Internal = "Internal",
    NoAuth = "NoAuth",
    NoAuthOwner = "NoAuthOwner",
    SettingOwnerNotSupportedYet = "SettingOwnerNotSupportedYet",
}

export interface InDeleteClub {
    club_id: number;
    token:   string;
    [property: string]: any;
}

export interface ResDeleteClub {
    error?: null | DeleteClubError;
    res?:   null;
    [property: string]: any;
}

/**
 * The user is not the owner and therefore is not authorized.
 */
export interface DeleteClubError {
    type:    IndecentType;
    ierror?: string;
    [property: string]: any;
}

export enum IndecentType {
    ClubNotFound = "ClubNotFound",
    Internal = "Internal",
    NoAuth = "NoAuth",
    NoAuthOwner = "NoAuthOwner",
}

export interface InGetUserProfileWithUsername {
    token?:   null | string;
    username: string;
    [property: string]: any;
}

export interface ResGetUserProfileWithUsername {
    error?: null | GetUserProfileWithUsernameError;
    res?:   null | GetUserProfileWithUsernameOut;
    [property: string]: any;
}

export interface GetUserProfileWithUsernameError {
    type:    TentacledType;
    ierror?: string;
    [property: string]: any;
}

export interface GetUserProfileWithUsernameOut {
    banner?:       null | string;
    description?:  null | string;
    display_name?: null | string;
    name?:         null | string;
    pfp?:          null | string;
    [property: string]: any;
}

export interface InAddFriendConnection {
    receiver_id: number;
    token:       string;
    [property: string]: any;
}

export interface ResAddFriendConnection {
    error?: null | AddFriendConnectionError;
    res?:   { [key: string]: any } | null;
    [property: string]: any;
}

export interface AddFriendConnectionError {
    type:    HilariousType;
    ierror?: string;
    [property: string]: any;
}

export enum HilariousType {
    AlreadyConnected = "AlreadyConnected",
    CannotAddSelf = "CannotAddSelf",
    Internal = "Internal",
    NoAuth = "NoAuth",
}

export interface InGetFriendConnections {
    token: string;
    [property: string]: any;
}

export interface ResGetFriendConnections {
    error?: null | GetFriendConnectionsError;
    res?:   null | GetFriendConnectionsOut;
    [property: string]: any;
}

export interface GetFriendConnectionsError {
    type:    FluffyType;
    ierror?: string;
    [property: string]: any;
}

export interface GetFriendConnectionsOut {
    friend_connections: { [key: string]: FriendStatus };
    [property: string]: any;
}

export enum FriendStatus {
    Full = "Full",
    RecievedPending = "RecievedPending",
    SentPending = "SentPending",
}

export interface ResAuthUser {
    error?: null | AuthUserError;
    res?:   null | AuthUserOut;
    [property: string]: any;
}

/**
 * Got an error from a cryptography function. This error should never occur.
 */
export interface AuthUserError {
    type:    AmbitiousType;
    ierror?: string;
    [property: string]: any;
}

export enum AmbitiousType {
    Internal = "Internal",
    InvalidCredentials = "InvalidCredentials",
    InvalidPasswordCryto = "InvalidPasswordCryto",
    UserNotFound = "UserNotFound",
    UserNotVerified = "UserNotVerified",
}

export interface AuthUserOut {
    email:    string;
    token:    string;
    username: string;
    [property: string]: any;
}

export interface InRemoveFriend {
    removal_id: number;
    token:      string;
    [property: string]: any;
}

export interface ResRemoveFriend {
    error?: null | RemoveFriendError;
    res?:   { [key: string]: any } | null;
    [property: string]: any;
}

export interface RemoveFriendError {
    type:    FluffyType;
    ierror?: string;
    [property: string]: any;
}

export interface InJoinClub {
    club_id: number;
    token:   string;
    [property: string]: any;
}

export interface ResJoinClub {
    error?: null | JoinClubError;
    res?:   { [key: string]: any } | null;
    [property: string]: any;
}

export interface JoinClubError {
    type:    CunningType;
    ierror?: string;
    [property: string]: any;
}

export enum CunningType {
    AlreadyJoined = "AlreadyJoined",
    Internal = "Internal",
    NoAuth = "NoAuth",
}

export interface InUnjoinClub {
    club_id: number;
    token:   string;
    [property: string]: any;
}

export interface ResUnjoinClub {
    error?: null | UnjoinClubError;
    res?:   { [key: string]: any } | null;
    [property: string]: any;
}

export interface UnjoinClubError {
    type:    MagentaType;
    ierror?: string;
    [property: string]: any;
}

export enum MagentaType {
    CannotUnjoinAsOwner = "CannotUnjoinAsOwner",
    ClubNotFound = "ClubNotFound",
    Internal = "Internal",
    NoAuth = "NoAuth",
}

export interface InGetClubMembers {
    club_id: number;
    [property: string]: any;
}

export interface ResGetClubMembers {
    error?: null | GetClubMembersError;
    res?:   null | GetClubMembersOut;
    [property: string]: any;
}

export interface GetClubMembersError {
    ierror: string;
    type:   GetClubMembersErrorType;
    [property: string]: any;
}

export enum GetClubMembersErrorType {
    Internal = "Internal",
}

export interface GetClubMembersOut {
    users: number[];
    [property: string]: any;
}

export interface InGetUserClubs {
    user_id: number;
    [property: string]: any;
}

export interface ResGetUserClubs {
    error?: null | GetUserClubsError;
    res?:   null | GetUserClubsOut;
    [property: string]: any;
}

export interface GetUserClubsError {
    ierror: string;
    type:   GetClubMembersErrorType;
    [property: string]: any;
}

export interface GetUserClubsOut {
    clubs: number[];
    [property: string]: any;
}

export interface InDeauthUser {
    token: string;
    [property: string]: any;
}

export interface ResDeauthUser {
    error?: null;
    res?:   null;
    [property: string]: any;
}

export interface InVerifyAccount {
    code: string;
    [property: string]: any;
}

export interface ResVerifyAccount {
    error?: null | VerifyAccountError;
    res?:   null;
    [property: string]: any;
}

/**
 * My favorite error message.
 */
export interface VerifyAccountError {
    type:    FriskyType;
    ierror?: string;
    [property: string]: any;
}

export enum FriskyType {
    CodeTimedOutOrAlreadyVerifiedOrInvalidCode = "CodeTimedOutOrAlreadyVerifiedOrInvalidCode",
    Internal = "Internal",
}

export interface InSendVerify {
    user_id: number;
    [property: string]: any;
}

export interface ResSendVerify {
    error?: null | SendVerifyError;
    res?:   null;
    [property: string]: any;
}

/**
 * Failed to send the verification message (usually an email error).
 */
export interface SendVerifyError {
    type:    MischievousType;
    ierror?: string;
    [property: string]: any;
}

export enum MischievousType {
    Internal = "Internal",
    ResendTooSoon = "ResendTooSoon",
    SendVerification = "SendVerification",
    UserNotFound = "UserNotFound",
}

export async function bubbelApiCreateUser(req: InCreateUser): Promise<ResCreateUser> {
            let fetchRes = await fetch(bubbelBathDev + '/api/create_user', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiAuthUser(req: InAuthUser): Promise<ResAuthUser> {
            let fetchRes = await fetch(bubbelBathDev + '/api/auth_user', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiDeauthUser(req: InDeauthUser): Promise<ResDeauthUser> {
            let fetchRes = await fetch(bubbelBathDev + '/api/deauth_user', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiVerifyAccount(req: InVerifyAccount): Promise<ResVerifyAccount> {
            let fetchRes = await fetch(bubbelBathDev + '/api/verify_account', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiSendVerify(req: InSendVerify): Promise<ResSendVerify> {
            let fetchRes = await fetch(bubbelBathDev + '/api/send_verify', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiSetUserProfile(req: InSetUserProfile): Promise<ResSetUserProfile> {
            let fetchRes = await fetch(bubbelBathDev + '/api/set_user_profile', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiGetUserProfile(req: InGetUserProfile): Promise<ResGetUserProfile> {
            let fetchRes = await fetch(bubbelBathDev + '/api/get_user_profile', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiDeleteUser(req: InDeleteUser): Promise<ResDeleteUser> {
            let fetchRes = await fetch(bubbelBathDev + '/api/delete_user', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiCreateClub(req: InCreateClub): Promise<ResCreateClub> {
            let fetchRes = await fetch(bubbelBathDev + '/api/create_club', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiGetClubProfile(req: InGetClubProfile): Promise<ResGetClubProfile> {
            let fetchRes = await fetch(bubbelBathDev + '/api/get_club_profile', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiSetClubProfile(req: InSetClubProfile): Promise<ResSetClubProfile> {
            let fetchRes = await fetch(bubbelBathDev + '/api/set_club_profile', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiDeleteClub(req: InDeleteClub): Promise<ResDeleteClub> {
            let fetchRes = await fetch(bubbelBathDev + '/api/delete_club', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiGetUserProfileWithUsername(req: InGetUserProfileWithUsername): Promise<ResGetUserProfileWithUsername> {
            let fetchRes = await fetch(bubbelBathDev + '/api/get_user_profile_with_username', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiAddFriendConnection(req: InAddFriendConnection): Promise<ResAddFriendConnection> {
            let fetchRes = await fetch(bubbelBathDev + '/api/add_friend_connection', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiGetFriendConnections(req: InGetFriendConnections): Promise<ResGetFriendConnections> {
            let fetchRes = await fetch(bubbelBathDev + '/api/get_friend_connections', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiRemoveFriend(req: InRemoveFriend): Promise<ResRemoveFriend> {
            let fetchRes = await fetch(bubbelBathDev + '/api/remove_friend', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiJoinClub(req: InJoinClub): Promise<ResJoinClub> {
            let fetchRes = await fetch(bubbelBathDev + '/api/join_club', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiUnjoinClub(req: InUnjoinClub): Promise<ResUnjoinClub> {
            let fetchRes = await fetch(bubbelBathDev + '/api/unjoin_club', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiGetClubMembers(req: InGetClubMembers): Promise<ResGetClubMembers> {
            let fetchRes = await fetch(bubbelBathDev + '/api/get_club_members', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }
export async function bubbelApiGetUserClubs(req: InGetUserClubs): Promise<ResGetUserClubs> {
            let fetchRes = await fetch(bubbelBathDev + '/api/get_user_clubs', {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                },

                body: JSON.stringify(req),
            });
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }