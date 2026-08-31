// ABSTRACTION: Handler is an interface, it names a capability
// ("be able to handle a request") without saying how. Any class
// implementing handle IS-A Handler. The router depends only on
// this contract, never on a concrete type.
export interface Handler {
    handle(req: Request): Response;
}

export interface Request {
    method: string;
    path: string;
    params: Record<string, string>; // extracted path params (:id)
    query: Record<string, string>;  // query params (?page=2)
}

export interface Response {
    status: number;
    body: string;
}

// ENCAPSULATION: "routes" is private to the class. Outsiders
// cannot touch the table directly; they go through the methods
// we expose (register / dispatch).
export class Router {
    private routes: Map<string, Handler> = new Map(); // key = "METHOD /route"

    // register binds method + route -> handler (the dispatch table).
    public register(method: string, pattern: string, h: Handler): void {
        this.routes.set(`${method} ${pattern}`, h);
    }

    // POLYMORPHISM: dispatch calls h.handle(req) without knowing which
    // concrete type h is. Same call, different behaviour per handler.
    public dispatch(method: string, urlStr: string): Response {
        const [path, queryStr] = urlStr.split('?');
        const query = parseQuery(queryStr || "");
        
        for (const [key, h] of this.routes.entries()) {
            const [m, pattern] = key.split(' ');
            if (m === method) {
                const params = match(pattern, path);
                if (params !== null) {
                    return h.handle({ method, path, params, query }); // polymorphic dispatch
                }
            }
        }
        return { status: 404, body: "route not found" }; // catch-all fallback
    }
}

function parseQuery(queryStr: string): Record<string, string> {
    const query: Record<string, string> = {};
    if (!queryStr) return query;
    for (const pair of queryStr.split('&')) {
        const [k, v] = pair.split('=');
        if (k) query[k] = v || "";
    }
    return query;
}

// match compares "/users/:id" against "/users/123", returning params.
function match(pattern: string, path: string): Record<string, string> | null {
    const p = pattern.replace(/^\/|\/$/g, '').split('/');
    const q = path.replace(/^\/|\/$/g, '').split('/');
    
    if (p.length !== q.length) return null;
    
    const params: Record<string, string> = {};
    for (let i = 0; i < p.length; i++) {
        if (p[i].startsWith(':')) { // dynamic segment
            params[p[i].substring(1)] = q[i]; // bind :id => "123"
        } else if (p[i] !== q[i]) { // static segment must match exactly
            return null;
        }
    }
    return params;
}

// "INHERITANCE" / SHARED BEHAVIOUR: BaseHandler holds shared logic;
// concrete handlers EXTEND it and reuse its methods.
export class BaseHandler implements Handler {
    protected name: string;
    
    constructor(name: string) {
        this.name = name;
    }
    
    protected log(req: Request): void {
        console.log(`[${this.name}] ${req.method} ${req.path}`);
    }
    
    public handle(req: Request): Response {
        throw new Error("Method not implemented.");
    }
}

export class GetUser extends BaseHandler {
    public handle(req: Request): Response {
        this.log(req); // reused (inherited) behaviour
        return { status: 200, body: `user id = ${req.params['id']}` };
    }
}

export class ListBooks extends BaseHandler {
    public handle(req: Request): Response {
        this.log(req);
        let page = req.query['page'];
        if (!page) { page = "1"; }
        return { status: 200, body: `books page ${page}` };
    }
}

if (require.main === module) {
    const r = new Router();
    r.register("GET", "/api/users/:id", new GetUser("users"));
    r.register("GET", "/api/books", new ListBooks("books"));

    console.log(r.dispatch("GET", "/api/users/123").body);   // => user id = 123
    console.log(r.dispatch("GET", "/api/books?page=2").body); // => books page 2
}
