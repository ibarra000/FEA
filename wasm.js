import init, { calculate_global_stiffness } from "./pkg/truss_calculator.js";

async function run() {
  await init();

  // Example calculation
  const e_mod = 120.0;
  const area = 500.0;
  const result = calculate_global_stiffness(e_mod, area, 1200.0, 180.0, [1, 2]);

  // Log the result (you can use it in your web page as needed)
  console.log(result);
}

run();
