module.exports = {
  siteTitle: "Komado",
  siteUrl: "https://substrateplatform.github.io/Komado/docs",
  logo: { alt: "Komado", href: "./" },
  favicon: "",
  srcDir: "docs",
  outputDir: "site",
  theme: {
    name: "ruby",
    defaultMode: "system",
    enableModeToggle: true,
    positionMode: "top",
    codeHighlight: true,
    customCss: [],
  },
  plugins: {
    sitemap: {
      defaultChangefreq: "weekly",
      defaultPriority: 0.8,
    },
  },
  search: true,
  minify: true,
  autoTitleFromH1: true,
  copyCode: true,
  pageNavigation: true,
  navigation: [
    { title: "Home", path: "/", icon: "home" },
    {
      title: "GitHub",
      path: "https://github.com/SubstratePlatform/Komado",
      icon: "github",
      external: true,
    },
  ],
  footer: "Built with docmd.",
};
