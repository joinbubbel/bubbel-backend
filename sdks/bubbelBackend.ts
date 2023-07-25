const bubbelBathDev = 'https://bubbel-bath.onrender.com';export interface BubbelCodegenOut {
    t0?:  InCreateUser;
    t1?:  ResCreateUser;
    t10?: InSetUserProfile;
    t11?: ResSetUserProfile;
    t12?: InGetUserProfile;
    t13?: ResGetUserProfile;
    t14?: InDeleteUser;
    t15?: ResDeleteUser;
    t2?:  InAuthUser;
    t3?:  ResAuthUser;
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
    error?:   null | CreateUserError;
    user_id?: number;
    [property: string]: any;
}

/**
 * Email is not valid by backend standards.
 *
 * Username is not valid by backend standards.
 *
 * Password is not valid by backend standards.
 *
 * Password failed to be encrypted.
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

export interface InDeleteUser {
    token: string;
    [property: string]: any;
}

export interface ResDeleteUser {
    error?: null | DeleteUserError;
    [property: string]: any;
}

export interface DeleteUserError {
    type:    FluffyType;
    ierror?: string;
    [property: string]: any;
}

export interface InAuthUser {
    password: string;
    username: string;
    [property: string]: any;
}

export interface ResAuthUser {
    email?:    string;
    error?:    null | AuthUserError;
    token?:    string;
    username?: string;
    [property: string]: any;
}

export interface AuthUserError {
    type:    StickyType;
    ierror?: string;
    [property: string]: any;
}

export enum StickyType {
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
    [property: string]: any;
}

export interface ResVerifyAccount {
    error?: null | VerifyAccountError;
    [property: string]: any;
}

export interface VerifyAccountError {
    type:    IndigoType;
    ierror?: string;
    [property: string]: any;
}

export enum IndigoType {
    CodeTimedOutOrAlreadyVerifiedOrInvalidCode = "CodeTimedOutOrAlreadyVerifiedOrInvalidCode",
    Internal = "Internal",
}

export interface InSendVerify {
    user_id: number;
    [property: string]: any;
}

export interface ResSendVerify {
    error?: null | SendVerifyError;
    [property: string]: any;
}

export interface SendVerifyError {
    type:    IndecentType;
    ierror?: string;
    [property: string]: any;
}

export enum IndecentType {
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
