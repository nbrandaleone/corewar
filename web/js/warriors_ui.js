const SLOT_COLORS = ["var(--w1)", "var(--w2)", "var(--w3)", "var(--w4)"];

export class WarriorsUI {
  constructor(wasm, presets, onChange) {
    this.wasm = wasm;
    this.presets = presets;
    this.onChange = onChange;
    this.activeTab = 0;

    this.slots = [0, 1, 2, 3].map((i) => ({
      id: i,
      enabled: i < 2,
      name: `Warrior ${i + 1}`,
      code: presets[i === 0 ? 0 : i === 1 ? 2 : i % presets.length].code,
      ok: true,
      statusMsg: "",
      wins: 0,
      ties: 0,
    }));

    this.tabsEl = document.getElementById("warrior-tabs");
    this.presetEl = document.getElementById("preset-select");
    this.enabledEl = document.getElementById("slot-enabled");
    this.editorEl = document.getElementById("code-editor");
    this.statusEl = document.getElementById("asm-status");

    this.initControls();
    this.compileAll();
    this.renderEditor();
  }

  initControls() {
    this.presetEl.innerHTML = `<option value="">-- Load Preset --</option>` +
      this.presets.map((p, idx) => `<option value="${idx}">${p.name} (${p.category})</option>`).join("");

    this.presetEl.addEventListener("change", () => {
      const idx = parseInt(this.presetEl.value, 10);
      if (!Number.isNaN(idx) && this.presets[idx]) {
        this.slots[this.activeTab].code = this.presets[idx].code;
        this.slots[this.activeTab].enabled = true;
        this.compileSlot(this.activeTab);
        this.renderEditor();
        this.onChange();
      }
      this.presetEl.value = "";
    });

    this.enabledEl.addEventListener("change", () => {
      this.slots[this.activeTab].enabled = this.enabledEl.checked;
      this.compileSlot(this.activeTab);
      this.renderTabs();
      this.onChange();
    });

    let timer = null;
    this.editorEl.addEventListener("input", () => {
      this.slots[this.activeTab].code = this.editorEl.value;
      clearTimeout(timer);
      timer = setTimeout(() => {
        this.compileSlot(this.activeTab);
        this.renderStatus();
        this.onChange();
      }, 180);
    });
  }

  compileAll() {
    this.slots.forEach((_, i) => this.compileSlot(i));
  }

  compileSlot(i) {
    const s = this.slots[i];
    if (!s.enabled) {
      this.wasm.clearWarrior(i);
      s.ok = true;
      s.statusMsg = "Slot disabled";
      return;
    }
    const res = this.wasm.loadWarrior(i, s.code);
    s.ok = res.ok;
    if (res.ok) {
      const [, name, author, len] = res.message.split("|");
      s.name = name || `Warrior ${i + 1}`;
      s.statusMsg = `OK • ${s.name} (${author}) • ${len} inst`;
    } else {
      s.statusMsg = res.message;
    }
  }

  renderTabs() {
    this.tabsEl.innerHTML = this.slots.map((s, i) => {
      const active = i === this.activeTab ? "active" : "";
      const dim = s.enabled ? "" : "opacity:0.45;";
      return `<button type="button" class="tab-btn ${active}" data-slot="${i}" style="border-bottom:2px solid ${SLOT_COLORS[i]};${dim}">W${i + 1}</button>`;
    }).join("");
    this.tabsEl.querySelectorAll(".tab-btn").forEach((btn) => {
      btn.addEventListener("click", () => {
        this.activeTab = Number(btn.dataset.slot);
        this.renderEditor();
      });
    });
  }

  renderStatus() {
    const s = this.slots[this.activeTab];
    this.statusEl.textContent = s.statusMsg;
    this.statusEl.className = `asm-status ${s.ok ? "ok" : "err"}`;
  }

  renderEditor() {
    this.renderTabs();
    const s = this.slots[this.activeTab];
    this.enabledEl.checked = s.enabled;
    this.editorEl.value = s.code;
    this.renderStatus();
  }

  getActiveSlots() {
    return this.slots.filter((s) => s.enabled && s.ok);
  }
}
