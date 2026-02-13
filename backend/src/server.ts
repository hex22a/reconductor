import { DASHBOARD_URL } from "./constants";

const PORT = 4000;

const HEADERS = {
    "Access-Control-Allow-Origin": DASHBOARD_URL,
    "Access-Control-Allow-Methods": "GET,POST,OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type",
};

const server = Bun.serve({
    port: PORT,
    routes: {
        '/hello-world': Response.json({ message: "Hello from Bun" }, { headers: HEADERS })
    }
})

console.log(process.env.NODE_ENV)
console.log(`Server is running on ${server.url}`)
