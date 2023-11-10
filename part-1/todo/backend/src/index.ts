import { Elysia } from "elysia";
import { staticPlugin } from "@elysiajs/static";
import { html } from "@elysiajs/html";

let is_dev = process.env.NODE_ENV === "development";
let basePath = is_dev ? "./public" : "/tmp/kube";
let picsumPath = is_dev ? "./public/picsum.jpg" : "/tmp/kube/picsum.jpg";
console.log(`🦊 Running in ${is_dev ? "development" : "production"} mode`);

const app = new Elysia()
  .use(
    staticPlugin({
      prefix: "/",
    })
  )
  .use(html())
  .get("/", async () => {
    return Bun.file(`./public/index.html`);
  })
  .get("/api", () => {
    return {
      message: "Hello Elysia",
    };
  })
  .get("/picsum", async () => {
    let file = Bun.file(picsumPath);
    let exists = await file.exists();
    let lastModified = await file.lastModified;
    let youngerThanDay = Date.now() - lastModified < 24 * 60 * 60 * 1000;
    if (exists && youngerThanDay) {
      return Bun.file(picsumPath);
    } else {
      const res = await fetch("https://picsum.photos/500");
      if (!res.ok || !res.body) {
        throw new Error("Network response was not ok");
      }
      const blob = await Bun.readableStreamToBlob(res.body);
      await Bun.write(picsumPath, blob);
      return Bun.file(picsumPath);
    }
  })
  .listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
