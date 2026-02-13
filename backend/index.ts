const PORT = 4000;

const HEADERS = {
    "Access-Control-Allow-Origin": "http://localhost:5173",
    "Access-Control-Allow-Methods": "GET,POST,OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type",
};

const server = Bun.serve({
    port: PORT,
    routes: {
        '/hello-world': Response.json({ message: "Hello from Bun" }, { headers: HEADERS })
    }
})

console.log(`Server is running on ${server.url}`)
