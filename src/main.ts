import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";

// Création de l'application Vue
const app = createApp(App);

// Utilisation de Pinia
app.use(createPinia());

// Montage de l'application
app.mount("#app");
