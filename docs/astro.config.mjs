// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

// https://astro.build/config
export default defineConfig({
  site: "https://brequet.github.io",
  base: "/tedlt",
  integrations: [
    starlight({
      title: "tedlt",
      description:
        "A CLI tool to create Jira tickets from the command line with just a title",
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/brequet/tedlt",
        },
      ],
      sidebar: [
        {
          label: "Getting Started",
          items: [
            { label: "Introduction", slug: "index" },
            { label: "Installation", slug: "getting-started/installation" },
            { label: "Quick Start", slug: "getting-started/quick-start" },
          ],
        },
        {
          label: "Usage",
          items: [
            { label: "Creating Tickets", slug: "usage/creating-tickets" },
            { label: "Using Profiles", slug: "usage/profiles" },
            { label: "Discovering Jira Info", slug: "usage/info-commands" },
          ],
        },
        {
          label: "Configuration",
          items: [
            { label: "Overview", slug: "configuration/overview" },
            {
              label: "Environment Variables",
              slug: "configuration/environment",
            },
            { label: "Configuration File", slug: "configuration/config-file" },
            { label: "Profiles", slug: "configuration/profiles" },
            { label: "Profile Inheritance", slug: "configuration/inheritance" },
            { label: "Property Templates", slug: "configuration/properties" },
          ],
        },
        {
          label: "Reference",
          items: [
            { label: "Commands", slug: "reference/commands" },
            { label: "Configuration Schema", slug: "reference/config-schema" },
          ],
        },
      ],
      customCss: ["./src/styles/custom.css"],
    }),
  ],
});
