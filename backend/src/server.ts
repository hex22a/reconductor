import routes from "./routes";

const PORT = 4000;

const server = Bun.serve({
    port: PORT,
    routes,
});

console.log(`Server is running on ${server.url}`);
