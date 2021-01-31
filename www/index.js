import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
import { Universe } from "wasm-game-of-life";

const CELL_SIZE_PX = 10;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLORS = [
  "#6B7280",
  "#EF4444",
  "#F59E0B",
  "#10B981",
  "#3B82F6",
  "#6366F1",
  "#8B5CF6",
  "#EC4899",
];

// Construct the universe, and get its width and height.
const universe = Universe.new(
  Math.ceil(window.outerWidth / CELL_SIZE_PX),
  Math.ceil(window.outerHeight / CELL_SIZE_PX)
);
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = window.outerHeight;
canvas.width = window.outerWidth;

const ctx = canvas.getContext("2d");

const renderLoop = () => {
  universe.tick();

  drawCells();

  setTimeout(() => {
    requestAnimationFrame(renderLoop);
  }, 50);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE_PX + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE_PX + 1) + 1, (CELL_SIZE_PX + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE_PX + 1) + 1);
    ctx.lineTo((CELL_SIZE_PX + 1) * width + 1, j * (CELL_SIZE_PX + 1) + 1);
  }

  ctx.stroke();
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << n % 8;
  return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) / 8);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = universe.getIndex(row, col);

      ctx.fillStyle = bitIsSet(idx, cells)
        ? ALIVE_COLORS[Math.floor(Math.random() * ALIVE_COLORS.length)]
        : DEAD_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE_PX + 1) + 1,
        row * (CELL_SIZE_PX + 1) + 1,
        CELL_SIZE_PX,
        CELL_SIZE_PX
      );
    }
  }

  ctx.stroke();
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
