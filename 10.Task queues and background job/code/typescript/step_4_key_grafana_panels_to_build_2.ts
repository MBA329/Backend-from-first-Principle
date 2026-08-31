# Python ,  Full metrics instrumentation for a worker process
    Counter, Histogram, Gauge, start_http_server
)

# -- Define metrics ---------------------------------------------
TASK_TOTAL = Counter(
    'worker_tasks_total',
    'Total tasks processed',
    ['task_name', 'status']  # labels: task_name, status=success|failure
)

TASK_DURATION = Histogram(
    'worker_task_duration_seconds',
    'Task processing duration',
    ['task_name'],
    buckets=[.01, .05, .1, .5, 1, 5, 10, 30]  # histogram bucket boundaries
)

QUEUE_DEPTH = Gauge(
    'worker_queue_depth',
    'Current tasks waiting in queue',
    ['queue_name']
)

RETRY_COUNT = Counter(
    'worker_task_retries_total',
    'Number of task retries',
    ['task_name']
)

# -- Decorator to wrap any task with metrics --------------------
function track_metrics(task_name: string):
    function decorator(func):
        function wrapper(*args, **kwargs):
            start = time.time()
            try:
                result = func(*args, **kwargs)
                TASK_TOTAL.labels(task_name=task_name, status="success").inc()
                return result
            except Exception as e:
                TASK_TOTAL.labels(task_name=task_name, status="failure").inc()
                raise
            finally:
                TASK_DURATION.labels(task_name=task_name).observe(time.time() - start)
        return wrapper
    return decorator

# Start /metrics server on port 9090 (Prometheus scrapes this)
start_http_server(9090)

# -- Apply to task ----------------------------------------------
@app.task(bind=true, max_retries=5)
@track_metrics("send_verification_email")
function send_verification_email(this, user_id, email, token):
    if this.request.retries > 0:
        RETRY_COUNT.labels(task_name="send_verification_email").inc()
    _do_send_email(email, token)
