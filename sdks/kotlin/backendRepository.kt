package com.example.bubbel.repository

import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import com.example.bubbel.model.backend.*
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.POST
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.SerializationException
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

interface backendService {
    @POST("/api/create_user")
    fun createUser(@Body userData: InCreateUser): Call<ResCreateUser>
    @POST("/api/auth_user")
    fun authUser(@Body userData: InAuthUser): Call<ResAuthUser>
    @POST("/api/deauth_user")
    fun deauthUser(@Body userData: InDeauthUser): Call<ResDeauthUser>
    @POST("/api/verify_account")
    fun verifyAccount(@Body userData: InVerifyAccount): Call<ResVerifyAccount>
    @POST("/api/send_verify")
    fun sendVerify(@Body userData: InSendVerify): Call<ResSendVerify>
    @POST("/api/set_user_profile")
    fun setUserProfile(@Body userData: InSetUserProfile): Call<ResSetUserProfile>
    @POST("/api/get_user_profile")
    fun getUserProfile(@Body userData: InGetUserProfile): Call<ResGetUserProfile>
    @POST("/api/delete_user")
    fun deleteUser(@Body userData: InDeleteUser): Call<ResDeleteUser>
    @POST("/api/create_club")
    fun createClub(@Body userData: InCreateClub): Call<ResCreateClub>
    @POST("/api/get_club_profile")
    fun getClubProfile(@Body userData: InGetClubProfile): Call<ResGetClubProfile>
    @POST("/api/set_club_profile")
    fun setClubProfile(@Body userData: InSetClubProfile): Call<ResSetClubProfile>
    @POST("/api/delete_club")
    fun deleteClub(@Body userData: InDeleteClub): Call<ResDeleteClub>
    @POST("/api/get_user_profile_with_username")
    fun getUserProfileWithUsername(@Body userData: InGetUserProfileWithUsername): Call<ResGetUserProfileWithUsername>
    @POST("/api/add_friend_connection")
    fun addFriendConnection(@Body userData: InAddFriendConnection): Call<ResAddFriendConnection>
    @POST("/api/get_friend_connections")
    fun getFriendConnections(@Body userData: InGetFriendConnections): Call<ResGetFriendConnections>
    @POST("/api/remove_friend")
    fun removeFriend(@Body userData: InRemoveFriend): Call<ResRemoveFriend>
    @POST("/api/join_club")
    fun joinClub(@Body userData: InJoinClub): Call<ResJoinClub>
    @POST("/api/unjoin_club")
    fun unjoinClub(@Body userData: InUnjoinClub): Call<ResUnjoinClub>
    @POST("/api/get_club_members")
    fun getClubMembers(@Body userData: InGetClubMembers): Call<ResGetClubMembers>
    @POST("/api/get_user_clubs")
    fun getUserClubs(@Body userData: InGetUserClubs): Call<ResGetUserClubs>
}

//  This was originally went in "RetrofitClient.kt"
object RetrofitClient {
    val api: backendService by lazy {
        Retrofit.Builder()
            .baseUrl("https://api.joinbubbel.com")
            .addConverterFactory(GsonConverterFactory.create())
            .build()
            .create(loginService::class.java)
    }
}

//  This was originally went in "XXXRepository.kt"
class BackendRepository {
    private val backendService = RetrofitClient.api

    suspend fun createUser(request: InCreateUser,  onSuccess: (ResCreateUser?) -> Unit, onError: (String) -> Unit){
        backendService.createUser(request).enqueue(object : Callback<ResCreateUser> {
            override fun onResponse(call: Call<ResCreateUser>, response: Response<ResCreateUser>) {
                if (response.isSuccessful) {
                    val out: ResCreateUser? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResCreateUser>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun authUser(request: InAuthUser,  onSuccess: (ResAuthUser?) -> Unit, onError: (String) -> Unit){
        backendService.authUser(request).enqueue(object : Callback<ResAuthUser> {
            override fun onResponse(call: Call<ResAuthUser>, response: Response<ResAuthUser>) {
                if (response.isSuccessful) {
                    val out: ResAuthUser? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResAuthUser>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun deauthUser(request: InDeauthUser,  onSuccess: (ResDeauthUser?) -> Unit, onError: (String) -> Unit){
        backendService.deauthUser(request).enqueue(object : Callback<ResDeauthUser> {
            override fun onResponse(call: Call<ResDeauthUser>, response: Response<ResDeauthUser>) {
                if (response.isSuccessful) {
                    val out: ResDeauthUser? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResDeauthUser>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun verifyAccount(request: InVerifyAccount,  onSuccess: (ResVerifyAccount?) -> Unit, onError: (String) -> Unit){
        backendService.verifyAccount(request).enqueue(object : Callback<ResVerifyAccount> {
            override fun onResponse(call: Call<ResVerifyAccount>, response: Response<ResVerifyAccount>) {
                if (response.isSuccessful) {
                    val out: ResVerifyAccount? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResVerifyAccount>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun sendVerify(request: InSendVerify,  onSuccess: (ResSendVerify?) -> Unit, onError: (String) -> Unit){
        backendService.sendVerify(request).enqueue(object : Callback<ResSendVerify> {
            override fun onResponse(call: Call<ResSendVerify>, response: Response<ResSendVerify>) {
                if (response.isSuccessful) {
                    val out: ResSendVerify? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResSendVerify>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun setUserProfile(request: InSetUserProfile,  onSuccess: (ResSetUserProfile?) -> Unit, onError: (String) -> Unit){
        backendService.setUserProfile(request).enqueue(object : Callback<ResSetUserProfile> {
            override fun onResponse(call: Call<ResSetUserProfile>, response: Response<ResSetUserProfile>) {
                if (response.isSuccessful) {
                    val out: ResSetUserProfile? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResSetUserProfile>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun getUserProfile(request: InGetUserProfile,  onSuccess: (ResGetUserProfile?) -> Unit, onError: (String) -> Unit){
        backendService.getUserProfile(request).enqueue(object : Callback<ResGetUserProfile> {
            override fun onResponse(call: Call<ResGetUserProfile>, response: Response<ResGetUserProfile>) {
                if (response.isSuccessful) {
                    val out: ResGetUserProfile? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResGetUserProfile>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun deleteUser(request: InDeleteUser,  onSuccess: (ResDeleteUser?) -> Unit, onError: (String) -> Unit){
        backendService.deleteUser(request).enqueue(object : Callback<ResDeleteUser> {
            override fun onResponse(call: Call<ResDeleteUser>, response: Response<ResDeleteUser>) {
                if (response.isSuccessful) {
                    val out: ResDeleteUser? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResDeleteUser>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun createClub(request: InCreateClub,  onSuccess: (ResCreateClub?) -> Unit, onError: (String) -> Unit){
        backendService.createClub(request).enqueue(object : Callback<ResCreateClub> {
            override fun onResponse(call: Call<ResCreateClub>, response: Response<ResCreateClub>) {
                if (response.isSuccessful) {
                    val out: ResCreateClub? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResCreateClub>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun getClubProfile(request: InGetClubProfile,  onSuccess: (ResGetClubProfile?) -> Unit, onError: (String) -> Unit){
        backendService.getClubProfile(request).enqueue(object : Callback<ResGetClubProfile> {
            override fun onResponse(call: Call<ResGetClubProfile>, response: Response<ResGetClubProfile>) {
                if (response.isSuccessful) {
                    val out: ResGetClubProfile? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResGetClubProfile>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun setClubProfile(request: InSetClubProfile,  onSuccess: (ResSetClubProfile?) -> Unit, onError: (String) -> Unit){
        backendService.setClubProfile(request).enqueue(object : Callback<ResSetClubProfile> {
            override fun onResponse(call: Call<ResSetClubProfile>, response: Response<ResSetClubProfile>) {
                if (response.isSuccessful) {
                    val out: ResSetClubProfile? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResSetClubProfile>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun deleteClub(request: InDeleteClub,  onSuccess: (ResDeleteClub?) -> Unit, onError: (String) -> Unit){
        backendService.deleteClub(request).enqueue(object : Callback<ResDeleteClub> {
            override fun onResponse(call: Call<ResDeleteClub>, response: Response<ResDeleteClub>) {
                if (response.isSuccessful) {
                    val out: ResDeleteClub? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResDeleteClub>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun getUserProfileWithUsername(request: InGetUserProfileWithUsername,  onSuccess: (ResGetUserProfileWithUsername?) -> Unit, onError: (String) -> Unit){
        backendService.getUserProfileWithUsername(request).enqueue(object : Callback<ResGetUserProfileWithUsername> {
            override fun onResponse(call: Call<ResGetUserProfileWithUsername>, response: Response<ResGetUserProfileWithUsername>) {
                if (response.isSuccessful) {
                    val out: ResGetUserProfileWithUsername? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResGetUserProfileWithUsername>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun addFriendConnection(request: InAddFriendConnection,  onSuccess: (ResAddFriendConnection?) -> Unit, onError: (String) -> Unit){
        backendService.addFriendConnection(request).enqueue(object : Callback<ResAddFriendConnection> {
            override fun onResponse(call: Call<ResAddFriendConnection>, response: Response<ResAddFriendConnection>) {
                if (response.isSuccessful) {
                    val out: ResAddFriendConnection? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResAddFriendConnection>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun getFriendConnections(request: InGetFriendConnections,  onSuccess: (ResGetFriendConnections?) -> Unit, onError: (String) -> Unit){
        backendService.getFriendConnections(request).enqueue(object : Callback<ResGetFriendConnections> {
            override fun onResponse(call: Call<ResGetFriendConnections>, response: Response<ResGetFriendConnections>) {
                if (response.isSuccessful) {
                    val out: ResGetFriendConnections? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResGetFriendConnections>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun removeFriend(request: InRemoveFriend,  onSuccess: (ResRemoveFriend?) -> Unit, onError: (String) -> Unit){
        backendService.removeFriend(request).enqueue(object : Callback<ResRemoveFriend> {
            override fun onResponse(call: Call<ResRemoveFriend>, response: Response<ResRemoveFriend>) {
                if (response.isSuccessful) {
                    val out: ResRemoveFriend? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResRemoveFriend>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun joinClub(request: InJoinClub,  onSuccess: (ResJoinClub?) -> Unit, onError: (String) -> Unit){
        backendService.joinClub(request).enqueue(object : Callback<ResJoinClub> {
            override fun onResponse(call: Call<ResJoinClub>, response: Response<ResJoinClub>) {
                if (response.isSuccessful) {
                    val out: ResJoinClub? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResJoinClub>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun unjoinClub(request: InUnjoinClub,  onSuccess: (ResUnjoinClub?) -> Unit, onError: (String) -> Unit){
        backendService.unjoinClub(request).enqueue(object : Callback<ResUnjoinClub> {
            override fun onResponse(call: Call<ResUnjoinClub>, response: Response<ResUnjoinClub>) {
                if (response.isSuccessful) {
                    val out: ResUnjoinClub? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResUnjoinClub>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun getClubMembers(request: InGetClubMembers,  onSuccess: (ResGetClubMembers?) -> Unit, onError: (String) -> Unit){
        backendService.getClubMembers(request).enqueue(object : Callback<ResGetClubMembers> {
            override fun onResponse(call: Call<ResGetClubMembers>, response: Response<ResGetClubMembers>) {
                if (response.isSuccessful) {
                    val out: ResGetClubMembers? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResGetClubMembers>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
    suspend fun getUserClubs(request: InGetUserClubs,  onSuccess: (ResGetUserClubs?) -> Unit, onError: (String) -> Unit){
        backendService.getUserClubs(request).enqueue(object : Callback<ResGetUserClubs> {
            override fun onResponse(call: Call<ResGetUserClubs>, response: Response<ResGetUserClubs>) {
                if (response.isSuccessful) {
                    val out: ResGetUserClubs? = response.body()
                    onSuccess(out)
                } else {
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }
            }

            override fun onFailure(call: Call<ResGetUserClubs>, t: Throwable) {
                onError(t.message ?: "Network request failed")
            }
        })
    }
}