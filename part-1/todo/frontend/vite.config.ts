import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],

  define: {
    BACKEND_URL: JSON.stringify("http://localhost:3000"),
  },
});
