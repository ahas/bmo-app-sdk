// Generated from bmo-app.toml by bmo-app-sdk's bmo-app-declare; change the manifest and regenerate.
// The Nuxt defaults every BMO application web app starts from, as bmo-app-sdk's docs/web.md states them:
// the UI and one Korean locale, the head block and icon set, the runtime config the page server fills
// from NUXT_* environment, the Bun preset and the API proxy. The app's own nuxt.config.ts calls
// withAppDefaults and adds what is its own; a key the app names stands over the default.

export function appDefaults() {
  return {
    modules: ["@nuxt/ui", "@nuxtjs/i18n"],
    compatibilityDate: "2026-06-30",
    devtools: { enabled: false },
    app: {
      head: {
        htmlAttrs: { lang: "ko" },
        meta: [
          { name: "viewport", content: "width=device-width, initial-scale=1, viewport-fit=cover" },
          { name: "theme-color", content: "#ffffff" },
          { name: "format-detection", content: "telephone=no" }
        ],
        link: [
          { rel: "icon", type: "image/svg+xml", href: "/favicon.svg" },
          { rel: "apple-touch-icon", href: "/apple-touch-icon.png" },
          { rel: "manifest", href: "/manifest.webmanifest" }
        ]
      }
    },
    runtimeConfig: {
      // The service the page server reads the API from while rendering; NUXT_API_BASE sets it.
      apiBase: "@@api_base@@",
      public: {
        // The address canonical links, previews and structured data are written on; NUXT_PUBLIC_SITE_URL sets it.
        siteUrl: "@@public_url@@",
        // The Search Console token; NUXT_PUBLIC_GOOGLE_SITE_VERIFICATION sets it, and no tag is written while empty.
        googleSiteVerification: ""
      }
    },
    i18n: {
      locales: [{ code: "ko", language: "ko-KR", file: "ko.json" }],
      defaultLocale: "ko",
      strategy: "no_prefix",
      detectBrowserLanguage: false
    },
    ui: { fonts: false },
    icon: { serverBundle: { collections: ["material-symbols"] }, clientBundle: { scan: true } },
    devServer: { host: "127.0.0.1", port: @@dev_port@@ },
    // A cached page carries its data inside its own HTML: a payload fetched apart is cached apart,
    // and the two could come from different renders, which the browser cannot take over cleanly.
    experimental: { payloadExtraction: false },
    // Every style rides in the rendered page, so a phone paints without a blocking stylesheet.
    features: { inlineStyles: true },
    nitro: {
      preset: "bun",
      devProxy: { "/api": { target: "@@api_base@@/api", changeOrigin: true } }
    },
    typescript: { strict: true }
  };
}

// The defaults with the application's own config laid over them: a top-level key the app names
// replaces the default of that key, and everything else the app leaves unnamed stands as it is.
export function withAppDefaults(own = {}) {
  return { ...appDefaults(), ...own };
}
