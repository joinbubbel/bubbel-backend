use super::*;

pub fn get_args() -> Vec<&'static str> {
    vec!["-l", "kotlin", "--framework", "kotlinx", "--package", "a"]
}

pub fn post_process(s: String) -> String {
    format!(
        "import kotlinx.coroutines.Dispatchers\nimport kotlinx.coroutines.withContext\nimport java.net.HttpURLConnection\nimport java.net.URL\n{}\nval BUBBEL_BATH_DEV = \"http://api.joinbubbel.com\"\n",
        s.replace("package a", "")
    )
}

pub fn get_fetch(e: &Endpoint) -> String {
    format!(
        "suspend fun {}(request: {}): {} = withContext(Dispatchers.IO) {{
        val encoder = Json {{ ignoreUnknownKeys = true }}
        val json = encoder.encodeToString(request)
        val url = URL(\"$BUBBEL_BATH_DEV{}\")
        val urlConnection = url.openConnection() as HttpURLConnection
        urlConnection.requestMethod = \"POST\"
        urlConnection.setRequestProperty(\"Content-Type\", \"application/json\")
        urlConnection.doOutput = true
        urlConnection.outputStream.use {{ outputStream ->
            outputStream.write(json.toByteArray())
        }}

        val responseCode = urlConnection.responseCode
        if (responseCode == HttpURLConnection.HTTP_OK) {{
            val responseString = urlConnection.inputStream.bufferedReader().use {{ it.readText() }}
            val decoder = Json {{ ignoreUnknownKeys = true }}
            try {{
                decoder.decodeFromString(responseString)
            }} catch (ex: SerializationException) {{
                throw Exception(\"Error decoding response: ${{ex.message}}\")
            }}
        }} else {{
            throw Exception(\"Error fetching data. Response code: $responseCode\")
        }}
    }}",
        e.fn_name, e.in_ty, e.out_ty, e.endpoint
    )
}
