use std::collections::HashMap;

// ABSTRACTION: Handler is a trait, it names a capability
// ("be able to Handle a request") without saying how. Any type
// implementing Handler IS-A Handler. The router depends only on
// this contract, never on a concrete type.
pub trait Handler {
    fn handle(&self, req: &Request) -> Response;
}

pub struct Request {
    pub method: String,
    pub path: String,
    pub params: HashMap<String, String>, // extracted path params (:id)
    pub query: HashMap<String, String>,  // query params (?page=2)
}

pub struct Response {
    pub status: u16,
    pub body: String,
}

// ENCAPSULATION: "routes" is private to the struct. Outsiders cannot
// touch the table directly; they go through the methods we expose
// (register / dispatch).
pub struct Router {
    // using Box<dyn Handler> for polymorphism
    routes: HashMap<String, Box<dyn Handler>>, // key = "METHOD /route"
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    // register binds method + route -> handler (the dispatch table).
    pub fn register(&mut self, method: &str, pattern: &str, h: Box<dyn Handler>) {
        let key = format!("{} {}", method, pattern);
        self.routes.insert(key, h);
    }

    // POLYMORPHISM: dispatch calls h.handle(req) without knowing which
    // concrete type h is. Same call, different behaviour per handler.
    pub fn dispatch(&self, method: &str, url: &str) -> Response {
        let parts: Vec<&str> = url.splitn(2, '?').collect();
        let path = parts[0];
        let query_str = if parts.len() > 1 { parts[1] } else { "" };
        let query = parse_query(query_str);

        for (key, h) in &self.routes {
            let key_parts: Vec<&str> = key.splitn(2, ' ').collect();
            if key_parts.len() != 2 { continue; }
            let r_method = key_parts[0];
            let r_pattern = key_parts[1];

            if r_method == method {
                if let Some(params) = match_pattern(r_pattern, path) {
                    let req = Request {
                        method: method.to_string(),
                        path: path.to_string(),
                        params,
                        query: query.clone(),
                    };
                    return h.handle(&req); // polymorphic dispatch
                }
            }
        }
        Response { status: 404, body: "route not found".to_string() } // catch-all fallback
    }
}

fn parse_query(query_str: &str) -> HashMap<String, String> {
    let mut query = HashMap::new();
    if query_str.is_empty() { return query; }
    for pair in query_str.split('&') {
        let kv: Vec<&str> = pair.splitn(2, '=').collect();
        if kv.len() == 2 {
            query.insert(kv[0].to_string(), kv[1].to_string());
        }
    }
    query
}

// match compares "/users/:id" against "/users/123", filling Params.
fn match_pattern(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let p_parts: Vec<&str> = pattern.trim_matches('/').split('/').collect();
    let q_parts: Vec<&str> = path.trim_matches('/').split('/').collect();
    
    if p_parts.len() != q_parts.len() {
        return None;
    }
    
    let mut params = HashMap::new();
    for (p, q) in p_parts.iter().zip(q_parts.iter()) {
        if p.starts_with(':') { // dynamic segment
            let key = &p[1..];
            params.insert(key.to_string(), q.to_string()); // bind :id => "123"
        } else if p != q { // static segment must match exactly
            return None;
        }
    }
    Some(params)
}

// "INHERITANCE" / COMPOSITION: BaseHandler holds shared logic;
// concrete handlers use it for reused behaviour.
pub struct BaseHandler {
    pub name: String,
}

impl BaseHandler {
    pub fn log(&self, req: &Request) {
        println!("[{}] {} {}", self.name, req.method, req.path);
    }
}

pub struct GetUser {
    pub base: BaseHandler,
}

impl Handler for GetUser {
    fn handle(&self, req: &Request) -> Response {
        self.base.log(req); // reused (inherited) behaviour
        let id = req.params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
        Response { status: 200, body: format!("user id = {}", id) }
    }
}

pub struct ListBooks {
    pub base: BaseHandler,
}

impl Handler for ListBooks {
    fn handle(&self, req: &Request) -> Response {
        self.base.log(req);
        let default_page = "1".to_string();
        let page = req.query.get("page").unwrap_or(&default_page);
        Response { status: 200, body: format!("books page {}", page) }
    }
}

fn main() {
    let mut r = Router::new();
    
    r.register("GET", "/api/users/:id", Box::new(GetUser { 
        base: BaseHandler { name: "users".to_string() } 
    }));
    
    r.register("GET", "/api/books", Box::new(ListBooks { 
        base: BaseHandler { name: "books".to_string() } 
    }));

    println!("{}", r.dispatch("GET", "/api/users/123").body);   // => user id = 123
    println!("{}", r.dispatch("GET", "/api/books?page=2").body); // => books page 2
}
