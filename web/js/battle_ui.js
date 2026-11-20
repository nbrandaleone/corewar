const COLORS = ["#f59e0b", "#06b6d4", "#10b981", "#f43f5e"];

export class BattleUI {
  constructor(wasm, canvasUI, warriorsUI) {
    this.wasm = wasm;
    this.canvasUI = canvasUI;
    this.warriorsUI = warriorsUI;
    this.playing = false;
    this.rafId = null;
    this.roundSeed = 1001;
    this.roundIndex = 0;
    this.speed = 40;

    this.btnPlay = document.getElementById("btn-play");
    this.cycleEl = document.getElementById("cycle-readout");
    this.bannerEl = document.getElementById("round-banner");
    this.statsEl = document.getElementById("warrior-stats");
    this.inspectorEl = document.getElementById("cell-inspector");
    this.speedSlider = document.getElementById("speed-slider");
    this.speedVal = document.getElementById("speed-val");

    this.bindControls();
  }

  bindControls() {
    this.btnPlay.addEventListener("click", () => (this.playing ? this.pause() : this.play()));
    document.getElementById("btn-step1").addEventListener("click", () => { this.pause(); this.step(1); });
    document.getElementById("btn-step100").addEventListener("click", () => { this.pause(); this.step(100); });
    document.getElementById("btn-reset").addEventListener("click", () => { this.pause(); this.resetRound(); });
    document.getElementById("btn-match").addEventListener("click", () => this.runMatch(20));
    document.getElementById("btn-clear-scores").addEventListener("click", () => {
      this.warriorsUI.slots.forEach((s) => { s.wins = 0; s.ties = 0; });
      this.renderStats();
    });
    this.speedSlider.addEventListener("input", () => {
      this.speed = Number(this.speedSlider.value);
      this.speedVal.textContent = String(this.speed);
    });
  }

  resetRound() {
    this.roundSeed = (this.roundSeed + 7919) >>> 0;
    this.roundIndex += 1;
    const count = this.wasm.startRound(this.roundSeed, this.roundIndex);
    if (count <= 0) {
      this.bannerEl.textContent = "Enable at least 1 valid warrior";
      return false;
    }
    this.bannerEl.textContent = "Running";
    this.renderFrame();
    return true;
  }

  play() {
    const stats = this.wasm.getStats();
    if (stats[1] !== 0 && !this.resetRound()) return;
    this.playing = true;
    this.btnPlay.textContent = "Pause";
    const loop = () => {
      if (!this.playing) return;
      const status = this.wasm.stepCycles(this.speed);
      this.renderFrame();
      if (status !== 0) {
        this.recordOutcome(status);
        this.pause();
        return;
      }
      this.rafId = requestAnimationFrame(loop);
    };
    this.rafId = requestAnimationFrame(loop);
  }

  pause() {
    this.playing = false;
    this.btnPlay.textContent = "Play";
    if (this.rafId) cancelAnimationFrame(this.rafId);
  }

  step(n) {
    const status = this.wasm.stepCycles(n);
    this.renderFrame();
    if (status !== 0) this.recordOutcome(status);
  }

  runMatch(rounds) {
    this.pause();
    const active = this.warriorsUI.getActiveSlots();
    if (active.length < 2) {
      this.bannerEl.textContent = "Need 2+ active warriors for a match";
      return;
    }
    for (let r = 0; r < rounds; r++) {
      this.roundSeed = (this.roundSeed + 7919) >>> 0;
      this.roundIndex += 1;
      this.wasm.startRound(this.roundSeed, this.roundIndex);
      const status = this.wasm.stepCycles(80000);
      this.recordOutcome(status);
    }
    this.renderFrame();
    this.bannerEl.textContent = `Completed ${rounds}-round match`;
  }

  recordOutcome(status) {
    const stats = this.wasm.getStats();
    const active = this.warriorsUI.getActiveSlots();
    if (status === 1) {
      const wid = stats[2];
      const winner = active[wid - 1];
      if (winner) {
        winner.wins += 1;
        this.bannerEl.textContent = `Winner: W${winner.id + 1} (${winner.name})`;
      }
    } else if (status === 2) {
      active.forEach((w) => { w.ties += 1; });
      this.bannerEl.textContent = "Tie (Max cycles reached)";
    }
  }

  renderFrame() {
    const cells = this.wasm.getRenderCells();
    this.canvasUI.render(cells);
    this.renderStats();
  }

  renderStats() {
    const stats = this.wasm.getStats();
    this.cycleEl.textContent = `Cycle: ${stats[0].toLocaleString()} / 80,000`;
    const active = this.warriorsUI.getActiveSlots();
    this.statsEl.innerHTML = active.map((s, idx) => {
      const base = 4 + idx * 4;
      const alive = stats[base] === 1;
      const procs = stats[base + 1];
      const writes = stats[base + 2];
      const score = s.wins * 3 + s.ties;
      return `
        <div class="stat-row" style="border-left:3px solid ${COLORS[s.id]}">
          <div class="stat-top">
            <span>W${s.id + 1}: ${s.name}</span>
            <span>${alive ? `${procs} proc` : "DEAD"}</span>
          </div>
          <div class="stat-sub">
            <span>Writes: ${writes}</span>
            <span>W:${s.wins} T:${s.ties} • ${score} pts</span>
          </div>
        </div>`;
    }).join("");
  }

  inspect(addr) {
    if (addr < 0) {
      this.inspectorEl.textContent = "Hover core to inspect cell";
      return;
    }
    const [cell, owner, dis] = this.wasm.inspectCell(addr).split("|");
    this.inspectorEl.textContent = `[${cell}] ${owner}: ${dis}`;
  }
}
