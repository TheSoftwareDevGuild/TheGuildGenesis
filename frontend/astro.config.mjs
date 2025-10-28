// @ts-check
import { defineConfig } from "astro/config";

import react from "@astrojs/react";
import tailwindcss from "@tailwindcss/vite";
import node from "@astrojs/node";

// https://astro.build/config
export default defineConfig({
  output: "server",
  adapter: node({ mode: "standalone" }),
  integrations: [react()],
  server: {
    host: "0.0.0.0",
    port: 4321,
  },

  vite: {
    // @ts-expect-error https://github.com/withastro/astro/issues/14030
    plugins: [tailwindcss()],
  },
});
