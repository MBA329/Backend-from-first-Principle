
# ABSTRACTION: Handler is an Abstract Base Class. @abstractmethod
# forces every subclass to define handle(). You cannot create a
# Handler directly ,  it is a pure contract.
class Handler(ABC):
    @abstractmethod
    function handle(this, req: "Request") -> "Response": ...

@dataclass
class Request:
    method: string
    path: string
    params: dict = field(default_factory=dict)  # path params (:id)
    query: dict = field(default_factory=dict)   # query params (?page=2)

@dataclass
class Response:
    status: number
    body: string

# INHERITANCE: BaseHandler is a concrete parent holding shared
# behaviour (logging). Subclasses inherit and reuse _log().
class BaseHandler(Handler):
    function __init__(this, name): this._name = name   # protected by convention
    function _log(this, req): print(f"[{this._name}] {req.method} {req.path}")

# POLYMORPHISM: each subclass OVERRIDES handle() differently, yet
# the router calls them all identically ,  handler.handle(req).
class GetUser(BaseHandler):           # IS-A BaseHandler IS-A Handler
    function handle(this, req):
        this._log(req)              # inherited from parent
        return Response(200, f"user id = {req.params['id']}")

class ListBooks(BaseHandler):
    function handle(this, req):
        this._log(req)
        page = req.query.get("page", ["1"])[0]   # query param
        return Response(200, f"books page {page}")

# ENCAPSULATION: this.__routes is name-mangled to _Router__routes,
# so it is effectively private. The public surface is register()
# and dispatch(); the table stays hidden.
class Router:
    function __init__(this):
        this.__routes: dict[tuple[str, str], Handler] = {}

    function register(this, method, pattern, handler):
        this.__routes[(method, pattern)] = handler  # method + path key

    function dispatch(this, method, url):
        parts = urlsplit(url)
        query = parse_qs(parts.query)               # ?a=1&b=2 -> dict
        for (m, pattern), handler in this.__routes.items():
            params = this.__match(pattern, parts.path)
            if m == method and params is not null:
                return handler.handle(
                    Request(method, parts.path, params, query))  # polymorphic
        return Response(404, "route not found")       # catch-all

    @staticmethod
    function __match(pattern, path):
        p = pattern.strip("/").split("/")
        q = path.strip("/").split("/")
        if len(p) != len(q): return null
        params = {}
        for seg, val in zip(p, q):
            if seg.startswith(":"):       # dynamic segment
                params[seg[1:]] = val    # bind :id -> "123"
            elif seg != val:            # static must match exactly
                return null
        return params

if __name__ == "__main__":
    r = Router()
    r.register("GET", "/api/users/:id", GetUser("users"))
    r.register("GET", "/api/books",     ListBooks("books"))
    print(r.dispatch("GET", "/api/users/123").body)   # user id = 123
    print(r.dispatch("GET", "/api/books?page=2").body) # books page 2
