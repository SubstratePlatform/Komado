// vite.config.js
import { defineConfig } from "vite";
import { VitePWA } from "vite-plugin-pwa";

export default defineConfig({
  root: "./",
  base: "/komado/",
  plugins: [
    VitePWA({
      registerType: "autoUpdate",
      includeAssets: ["robots.txt"],
      manifest: {
        name: "Komado Demo",
        short_name: "KomadoDemo",
        start_url: "/komado/",
        display: "standalone",
        background_color: "#f5f5f5",
        theme_color: "#2196F3",
      },
      workbox: {
        // Keep docs paths out of app-shell fallback so /komado/docs pages are served normally.
        navigateFallbackDenylist: [/^\/komado\/docs(?:\/|$)/],
        runtimeCaching: [
          {
            urlPattern: /.*\.(js|css)$/,
            handler: "NetworkFirst",
            options: { cacheName: "app-shell" },
          },
          {
            urlPattern: /.*\.(png|ico|json)$/,
            handler: "CacheFirst",
            options: { cacheName: "assets" },
          },
        ],
      },
    }),
  ],
  server: { open: true, allowedHosts: true },
  build: { sourcemap: true, outDir: "./dist", emptyOutDir: true },
});
