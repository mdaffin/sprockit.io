import { config } from "@vue/test-utils";

// Mock Nuxt components
config.stubs["nuxt-link"] = "<a><slot /></a>";
config.stubs["client-only"] = "<span><slot /></span>";
config.stubs["codemirror"] = "<span><slot /></span>";
