import { type Context, Hono } from "hono";

const app = new Hono();

app.get("/", (c: Context) => {
    return c.text("Hello Hono!");
});

Deno.serve(app.fetch);
