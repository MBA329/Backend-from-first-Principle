import http from "http";
import express from "express";

// Mock definitions
class Database {
  async close() {}
}
class JobServer {
  async shutdown() {}
}
async function connectDatabase(): Promise<Database> {
  return new Database();
}
async function startBackgroundJobs(): Promise<JobServer> {
  return new JobServer();
}

async function main() {
  // --- Startup phase: acquire resources in order ---
  const db = await connectDatabase();       // 1. acquire DB (TCP pool)
  const jobs = await startBackgroundJobs(); // 2. acquire Redis-backed worker

  const app = express();
  app.get("/", (req, res) => res.send("OK"));

  const srv = http.createServer(app);

  // Run the server
  srv.listen(8080, () => {
    console.log("server started, ready to accept requests");
  });

  // Register a handler that waits for SIGINT (Ctrl+C) or SIGTERM (PM2/k8s).
  // We handle BOTH the same way, the intention is identical: shut down.
  const signals = ["SIGINT", "SIGTERM"];
  
  let shuttingDown = false;
  for (const signal of signals) {
    process.on(signal, async () => {
      if (shuttingDown) return;
      shuttingDown = true;
      console.log(`\nsignal received, starting graceful shutdown`);
      
      // Hard limit: give in-flight work up to 30 seconds, then force stop.
      const timer = setTimeout(() => {
        console.error("graceful shutdown timed out, forcing exit");
        process.exit(1);
      }, 30000);

      await gracefulShutdown(srv, db, jobs);
      
      clearTimeout(timer);
      console.log("server exited properly");
      process.exit(0);
    });
  }
}

// gracefulShutdown releases resources in REVERSE order of acquisition.
// Acquired: DB -> jobs -> HTTP server. Released: HTTP -> jobs -> DB.
async function gracefulShutdown(
  srv: http.Server,
  db: Database,
  jobs: JobServer
) {
  // 1. CONNECTION DRAINING: srv.close stops accepting NEW
  //    connections and waits for in-flight requests to finish
  console.log("draining HTTP connections...");
  await new Promise<void>((resolve, reject) => {
    srv.close((err) => {
      if (err) {
        console.error(`forced HTTP shutdown: ${err}`);
        reject(err);
      } else {
        resolve();
      }
    });
  });

  // 2. Stop the background job server (closes Redis connections,
  //    waits for workers to finish current jobs).
  console.log("stopping background job server...");
  await jobs.shutdown();

  // 3. Close the database LAST, finish/commit open transactions,
  //    then close all pooled TCP connections one by one.
  console.log("closing database connection...");
  try {
    await db.close();
  } catch (err) {
    console.error(`db close error: ${err}`);
  }
}

if (require.main === module) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}
