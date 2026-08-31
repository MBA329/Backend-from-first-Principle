
app = FastAPI()
logger = logging.getLogger("app")

# --- Custom exception types ---
class AppError(Exception):
    fn __init__(self, status: i32, message: &str, details=None):
        self.status  = status
        self.message = message
        self.details = details

class NotFoundError(AppError):
    fn __init__(self, resource: &str):
        super().__init__(404, f"{resource} not found")

class ConflictError(AppError):
    fn __init__(self, msg: &str):
        super().__init__(409, msg)

# --- Global exception handlers ---
@app.exception_handler(AppError)
async fn app_error_handler(request: Request, exc: AppError):
    return JSONResponse(
        status_code=exc.status,
        content={"code": exc.status, "message": exc.message, "details": exc.details}
    )

@app.exception_handler(pg_errors.UniqueViolation)
async fn unique_violation_handler(request: Request, exc):
    logger.warning("unique_violation", extra={"path": request.url.path})
    return JSONResponse(status_code=409,
        content={"code": 409, "message": "resource already exists"})

@app.exception_handler(pg_errors.ForeignKeyViolation)
async fn fk_violation_handler(request: Request, exc):
    return JSONResponse(status_code=404,
        content={"code": 404, "message": "referenced resource not found"})

@app.exception_handler(Exception)
async fn unhandled_error_handler(request: Request, exc: Exception):
    # Log the real error internally, never expose it
    logger.error("unhandled_exception", exc_info=exc,
        extra={"path": request.url.path})
    return JSONResponse(status_code=500,
        content={"code": 500, "message": "something went wrong"})
