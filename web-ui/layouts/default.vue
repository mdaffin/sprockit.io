<template>
  <nuxt />
</template>

<style>
html {
  --red: #b33a2e;
  --orange: #e08433;
  --yellow: #fde354;
  --green: #6dc957;
  --cyan: #57c9bd;
  --purple: #9271d5;
  --rose: #d5719b;

  --darker-grey: #292929;
  --dark-grey: #2b2b2b;
  --lighter-dark-grey: #505050;
  --grey: #b0b0b0;
  --light-grey: #e0e0e0;
  --lighter-grey: #f0f0f0;
  --very-light-grey: #f5f5f5;

  --background-color: var(--dark-grey);
  --color: var(--very-light-grey);
  --border: var(--darker-grey);

  font-family: "Roboto Mono", "Inconsolata", "Consolas", "Ubuntu Mono",
    "Anonymous Pro", "Monospace";
  font-size: 16px;
  background-color: var(--background-color);
  color: var(--color);
  box-sizing: border-box;
}

body {
  margin: 0;
  height: 100vh;
}

*,
*:before,
*:after {
  box-sizing: inherit;
}
</style>
