export interface InCreateUser {
    email: string;
    password: string;
    username: string;
    [property: string]: any;
}

export interface ResCreateUser {
    error?: null | CreateUserError;
    [property: string]: any;
}

export interface CreateUserError {
    type: CreateUserErrorType;
    ierror?: string;
    [property: string]: any;
}

export enum CreateUserErrorType {
    EmailOrUsernametaken = "EmailOrUsernametaken",
    Internal = "Internal",
    InvalidEmail = "InvalidEmail",
    InvalidPassword = "InvalidPassword",
    InvalidPasswordCryto = "InvalidPasswordCryto",
    InvalidUsername = "InvalidUsername",
    SendVerification = "SendVerification",
}

export interface InAuthUser {
    password: string;
    username: string;
    [property: string]: any;
}

export interface ResAuthUser {
    email?: string;
    error?: null | AuthUserError;
    token?: string;
    username?: string;
    [property: string]: any;
}

export interface AuthUserError {
    type: AuthUserErrorType;
    ierror?: string;
    [property: string]: any;
}

export enum AuthUserErrorType {
    Internal = "Internal",
    InvalidCredentials = "InvalidCredentials",
    InvalidPasswordCryto = "InvalidPasswordCryto",
    UserNotFound = "UserNotFound",
    UserNotVerified = "UserNotVerified",
}

export interface InDeauthUser {
    token: string;
    [property: string]: any;
}

export interface ResDeauthUser {
    error?: null;
    [property: string]: any;
}

export interface InVerifyAccount {
    code: string;
    user_id: number;
    [property: string]: any;
}

export interface ResVerifyAccount {
    error?: null | VerifyAccountError;
    [property: string]: any;
}

export interface VerifyAccountError {
    type: VerifyAccountErrorType;
    ierror?: string;
    [property: string]: any;
}

export enum VerifyAccountErrorType {
    CodeTimedOutOrInvalidUser = "CodeTimedOutOrInvalidUser",
    Internal = "Internal",
    InvalidCode = "InvalidCode",
}

export interface InSetUserProfile {
    banner?: null | string;
    description?: null | string;
    display_name?: null | string;
    name?: null | string;
    pfp?: null | string;
    token: string;
    [property: string]: any;
}

export interface ResSetUserProfile {
    error?: null | SetUserProfileError;
    [property: string]: any;
}

export interface SetUserProfileError {
    type: SetUserProfileErrorType;
    ierror?: string;
    [property: string]: any;
}

export enum SetUserProfileErrorType {
    Internal = "Internal",
    NoAuth = "NoAuth",
}

export interface InDeleteUser {
    token: string;
    [property: string]: any;
}

export interface ResDeleteUser {
    error?: null | DeleteUserError;
    [property: string]: any;
}

export interface DeleteUserError {
    type: DeleteUserErrorType;
    ierror?: string;
    [property: string]: any;
}

export enum DeleteUserErrorType {
    Internal = "Internal",
    NoAuth = "NoAuth",
}

export var bubbelBathDev = "https://bubbel-bath.onrender.com";

export async function bubbelApiCreateUser(
    bath: String,
    req: InCreateUser,
): Promise<ResCreateUser> {
    let fetchRes = await fetch(bath + "/api/create_user", {
        method: "post",
        headers: {
            "Content-Type": "application/json",
        },

        body: JSON.stringify(req),
    });
    let resText = await fetchRes.text();
    return JSON.parse(resText);
}

export async function bubbelApiAuthUser(
    bath: String,
    req: InAuthUser,
): Promise<ResAuthUser> {
    let fetchRes = await fetch(bath + "/api/auth_user", {
        method: "post",
        headers: {
            "Content-Type": "application/json",
        },

        body: JSON.stringify(req),
    });
    let resText = await fetchRes.text();
    return JSON.parse(resText);
}

export async function bubbelApiDeauthUser(
    bath: String,
    req: InDeauthUser,
): Promise<ResDeauthUser> {
    let fetchRes = await fetch(bath + "/api/deauth_user", {
        method: "post",
        headers: {
            "Content-Type": "application/json",
        },

        body: JSON.stringify(req),
    });
    let resText = await fetchRes.text();
    return JSON.parse(resText);
}

export async function bubbelApiVerifyAccount(
    bath: String,
    req: InDeauthUser,
): Promise<ResDeauthUser> {
    let fetchRes = await fetch(bath + "/api/verify_account", {
        method: "post",
        headers: {
            "Content-Type": "application/json",
        },

        body: JSON.stringify(req),
    });
    let resText = await fetchRes.text();
    return JSON.parse(resText);
}

export async function bubbelApiSetUserProfile(
    bath: String,
    req: InSetUserProfile,
): Promise<ResSetUserProfile> {
    let fetchRes = await fetch(bath + "/api/set_user_profile", {
        method: "post",
        headers: {
            "Content-Type": "application/json",
        },

        body: JSON.stringify(req),
    });
    let resText = await fetchRes.text();
    return JSON.parse(resText);
}

export async function bubbelApiDeleteUser(
    bath: String,
    req: InDeleteUser,
): Promise<ResDeleteUser> {
    let fetchRes = await fetch(bath + "/api/delete_user", {
        method: "post",
        headers: {
            "Content-Type": "application/json",
        },

        body: JSON.stringify(req),
    });
    let resText = await fetchRes.text();
    return JSON.parse(resText);
}
