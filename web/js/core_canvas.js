const COLS = 100;
const ROWS = 80;
const CELL = 9;

const BASE_COLORS = [
  "#131922", // 0: empty
  "#b45309", // 1: W1 amber
  "#0e7490", // 2: W2 cyan
  "#047857", // 3: W3 emerald
  "#be123c", // 4: W4 rose
];

const BRIGHT_COLORS = [
  "#263142",
  "#fbbf24",
  "#22d3ee",
  "#34d399",
  "#fb7185",
];

export class CoreCanvas {
  constructor(canvas, onHoverCell) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");
    this.onHoverCell = onHoverCell;
    this.hoverAddr = -1;

    canvas.addEventListener("mousemove", (e) => {
      const addr = this.eventToAddr(e);
      if (addr !== this.hoverAddr) {
        this.hoverAddr = addr;
        this.onHoverCell(addr);
      }
    });
    canvas.addEventListener("mouseleave", () => {
      this.hoverAddr = -1;
      this.onHoverCell(-1);
    });
  }

  eventToAddr(e) {
    const rect = this.canvas.getBoundingClientRect();
    const x = Math.floor(((e.clientX - rect.left) / rect.width) * COLS);
    const y = Math.floor(((e.clientY - rect.top) / rect.height) * ROWS);
    if (x < 0 || x >= COLS || y < 0 || y >= ROWS) return -1;
    return y * COLS + x;
  }

  render(cells) {
    const ctx = this.ctx;
    ctx.fillStyle = "#070a0f";
    ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

    for (let i = 0; i < cells.length; i++) {
      const v = cells[i];
      const owner = v & 0xff;
      const isDat = (v >> 8) & 1;
      const nonZero = (v >> 9) & 1;
      const wrote = (v >> 20) & 1;
      const writeWid = (v >> 16) & 0x0f;
      const exec = (v >> 28) & 1;
      const execWid = (v >> 24) & 0x0f;

      const x = (i % COLS) * CELL;
      const y = Math.floor(i / COLS) * CELL;

      if (exec) {
        ctx.fillStyle = "#ffffff";
        ctx.fillRect(x, y, CELL - 1, CELL - 1);
        ctx.fillStyle = BRIGHT_COLORS[execWid] || "#fff";
        ctx.fillRect(x + 1, y + 1, CELL - 3, CELL - 3);
      } else if (wrote) {
        ctx.fillStyle = BRIGHT_COLORS[writeWid] || "#fff";
        ctx.fillRect(x, y, CELL - 1, CELL - 1);
      } else if (owner > 0) {
        if (isDat) {
          ctx.fillStyle = BASE_COLORS[owner] || "#475569";
          ctx.fillRect(x + 2, y + 2, CELL - 5, CELL - 5);
        } else {
          ctx.fillStyle = BRIGHT_COLORS[owner] || "#94a3b8";
          ctx.fillRect(x, y, CELL - 1, CELL - 1);
        }
      } else if (nonZero) {
        ctx.fillStyle = "#334155";
        ctx.fillRect(x + 1, y + 1, CELL - 3, CELL - 3);
      } else {
        ctx.fillStyle = "#0f151e";
        ctx.fillRect(x, y, CELL - 1, CELL - 1);
      }
    }

    if (this.hoverAddr >= 0) {
      const hx = (this.hoverAddr % COLS) * CELL;
      const hy = Math.floor(this.hoverAddr / COLS) * CELL;
      ctx.strokeStyle = "#f8fafc";
      ctx.lineWidth = 1.5;
      ctx.strokeRect(hx - 0.5, hy - 0.5, CELL, CELL);
    }
  }
}
