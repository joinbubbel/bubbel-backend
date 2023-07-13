interface InCreateUser {
    email: string;
    username: string;
    password: string;
}

interface CreateUserError {
    type:
    | "InvalidEmail"
    | "InvalidUsername"
    | "InvalidPassword"
    | "InvalidPasswordCrypto"
    | "EmailOrPasswordTaken"
    | "Internal";
    ierror: string | null;
}

interface ResCreateUser {
    error: CreateUserError | null;
}

interface InAuthUser {
    username: string;
    password: string;
}

interface AuthUserError {
    type:
    | "InvalidCredentials"
    | "InvalidPasswordCryto"
    | "UserNotFound"
    | "UserNotVerified"
    | "Internal";
    ierror: string | null;
}

interface ResAuthUser {
    error: AuthUserError | null;
    token: string | null;
    username: string | null;
    email: string | null;
}

interface InDeauthUser {
    token: string;
}

interface ResDeauthUser {
    error: null;
}

var bubbelBathDev = "https://bubbel-bath.onrender.com";

async function bubbelApiCreateUser(bath: String, req: InCreateUser) {
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

async function bubbelApiAuthUser(bath: String, req: InAuthUser) {
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

async function bubbelApiDeauthUser(bath: String, req: InDeauthUser) {
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

export {
    InCreateUser,
    CreateUserError,
    ResCreateUser,
    InAuthUser,
    AuthUserError,
    ResAuthUser,
    InDeauthUser,
    ResDeauthUser,
    bubbelBathDev,
    bubbelApiCreateUser,
    bubbelApiAuthUser,
    bubbelApiDeauthUser,
};
