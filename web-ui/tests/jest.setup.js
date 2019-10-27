import VueTestUtils from "@vue/test-utils";

// Mock Nuxt components
VueTestUtils.config.stubs["nuxt-link"] = "<a><slot /></a>";
VueTestUtils.config.stubs["client-only"] = "<span><slot /></span>";
VueTestUtils.config.stubs["codemirror"] = "<span><slot /></span>";
