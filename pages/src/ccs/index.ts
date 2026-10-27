import { createVirid } from "@virid/core";
import { VuePlugin } from "@virid/vue";
import { bindLayoutControllers } from "@/layouts/controllers";
const app = createVirid();
const vuePlugin = new VuePlugin();

app.use(vuePlugin, { disableBorrowChecker: false });

export function bootstrapVirid() {
  bindLayoutControllers(app);
  return app;
}
