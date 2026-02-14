import type { BunRequest } from "bun";
import { HEADERS } from "../../constants";

export async function post(req: BunRequest): Promise<Response> {
    await req.json();
    return Response.json({ foo: 'bar' }, { headers: HEADERS });
}
