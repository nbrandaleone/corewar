export class CorewarWasm {
  constructor(instance) {
    this.ex = instance.exports;
    this.enc = new TextEncoder();
    this.dec = new TextDecoder();
  }

  static async load(url = "./corewar.wasm") {
    const resp = await fetch(url);
    const bytes = await resp.arrayBuffer();
    const { instance } = await WebAssembly.instantiate(bytes, {});
    const api = new CorewarWasm(instance);
    api.ex.init_arena(8000, 80000, 8000, 100);
    return api;
  }

  readText(ptr) {
    const len = this.ex.get_text_len();
    const view = new Uint8Array(this.ex.memory.buffer, ptr, len);
    return this.dec.decode(view);
  }

  loadWarrior(slot, source) {
    const bytes = this.enc.encode(source);
    const ptr = this.ex.alloc_buf(bytes.length || 1);
    new Uint8Array(this.ex.memory.buffer, ptr, bytes.length).set(bytes);
    const rc = this.ex.load_warrior(slot, ptr, bytes.length);
    this.ex.free_buf(ptr, bytes.length || 1);
    const msg = this.readText(this.ex.get_text_ptr());
    return { ok: rc === 0, line: rc, message: msg };
  }

  clearWarrior(slot) {
    this.ex.clear_warrior(slot);
  }

  startRound(seed, startOffset) {
    return this.ex.start_round(seed >>> 0, startOffset >>> 0);
  }

  stepCycles(count) {
    return this.ex.step_cycles(count >>> 0);
  }

  getRenderCells() {
    const ptr = this.ex.update_render_buf();
    return new Uint32Array(this.ex.memory.buffer, ptr, 8000);
  }

  getStats() {
    const ptr = this.ex.update_stats_buf();
    return new Uint32Array(this.ex.memory.buffer, ptr, 20);
  }

  inspectCell(addr) {
    const ptr = this.ex.inspect_cell(addr >>> 0);
    return this.readText(ptr);
  }

  getPresets() {
    const count = this.ex.get_preset_count();
    const out = [];
    for (let i = 0; i < count; i++) {
      const ptr = this.ex.get_preset_info(i);
      const raw = this.readText(ptr);
      const [id, name, author, category, ...rest] = raw.split("|");
      out.push({ id, name, author, category, code: rest.join("|") });
    }
    return out;
  }
}
