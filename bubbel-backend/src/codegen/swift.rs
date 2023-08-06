use super::*;

pub fn get_args() -> Vec<&'static str> {
    vec!["-l", "swift"]
}

pub fn post_process(s: String) -> String {
    format!(
        "let bubbelBathDev = \"https://api.joinbubbel.com\"\n{}",
        s.replace("package a", "")
    )
}

pub fn get_fetch(e: &Endpoint) -> String {
    format!(
        "func {}(req: {}) async throws -> {} {{
            let json = try req.jsonData()
            
            let url = URL(string: bubbelBathDev + \"{}\")!
            var urlRequest = URLRequest(url: url)
            urlRequest.addValue(\"application/json\", forHTTPHeaderField: \"Content-Type\")
            urlRequest.httpMethod = \"POST\"
            urlRequest.httpBody = json
            
            let (data, response) = try await URLSession.shared.data(for: urlRequest)
            let (dataString) = String(data: data, encoding: .utf8) ?? \"\"
            
            let decoder = JSONDecoder()
            decoder.keyDecodingStrategy = .convertFromSnakeCase
            let result = try decoder.decode({}.self, from: data)
            return result
        }}",
        e.fn_name, e.in_ty, e.out_ty, e.endpoint, e.out_ty
    )
}
