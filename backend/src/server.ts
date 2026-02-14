import { HEADERS } from "./constants";

const PORT = 4000;

const server = Bun.serve({
    port: PORT,
    routes: {
        '/hello-world': { POST: Response.json({ message: "Hello from Bun" }, { headers: HEADERS }) }
    }
})

console.log(`Server is running on ${server.url}`)
