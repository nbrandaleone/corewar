import { CorewarWasm } from "./wasm_bridge.js";
import { CoreCanvas } from "./core_canvas.js";
import { WarriorsUI } from "./warriors_ui.js";
import { BattleUI } from "./battle_ui.js";
import { initReferenceModal } from "./reference_ui.js";

async function bootstrap() {
  initReferenceModal();
  const wasm = await CorewarWasm.load("./corewar.wasm");
  const presets = wasm.getPresets();

  let battleUI = null;
  const canvasUI = new CoreCanvas(
    document.getElementById("core-canvas"),
    (addr) => battleUI && battleUI.inspect(addr)
  );
  const warriorsUI = new WarriorsUI(wasm, presets, () => {
    if (battleUI && !battleUI.playing) battleUI.resetRound();
  });
  battleUI = new BattleUI(wasm, canvasUI, warriorsUI);
  battleUI.resetRound();
  if (new URLSearchParams(location.search).has("demo")) {
    battleUI.step(1500);
  }
}

bootstrap().catch((err) => {
  console.error("Failed to initialize Core War WASM:", err);
  document.getElementById("round-banner").textContent = "WASM Load Error";
});
