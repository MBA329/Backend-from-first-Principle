import express from 'express';
// Node.js doesn't have a direct equivalent to Go's pprof out of the box,
// but we can use the 'clinic' suite or built-in inspector for profiling.
// This is a placeholder showing how one might start an inspector session.
import inspector from 'inspector';

const app = express();

app.listen(6060, () => {
    // Start the inspector to allow profiling connections (equivalent to exposing debug endpoints)
    inspector.open(9229, 'localhost');
    console.log('App running on port 6060. Inspector open on 9229.');
});
