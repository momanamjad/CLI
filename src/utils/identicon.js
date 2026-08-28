export function generateIdenticon(seed = "default") {
  if (!seed) seed = "default";
  
  let hash = 0;
  for (let i = 0; i < seed.length; i++) {
    hash = seed.charCodeAt(i) + ((hash << 5) - hash);
  }

  // Use the hash to pick a color
  const hue = Math.abs(hash) % 360;
  const color = `hsl(${hue}, 65%, 65%)`;
  
  // Background color - github uses #f0f0f0
  const bgColor = '#f0f0f0';

  // 15 blocks for half the grid (3x5)
  const grid = [];
  for (let i = 0; i < 15; i++) {
    grid.push((hash >> i) & 1);
  }

  const size = 50;
  const blockSize = size / 5;
  let svgBlocks = '';

  for (let x = 0; x < 3; x++) {
    for (let y = 0; y < 5; y++) {
      if (grid[x * 5 + y]) {
        // Left side
        svgBlocks += `<rect x="${x * blockSize}" y="${y * blockSize}" width="${blockSize}" height="${blockSize}" fill="${color}" />`;
        // Right side (mirrored)
        if (x < 2) {
          svgBlocks += `<rect x="${(4 - x) * blockSize}" y="${y * blockSize}" width="${blockSize}" height="${blockSize}" fill="${color}" />`;
        }
      }
    }
  }

  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 ${size} ${size}"><rect width="${size}" height="${size}" fill="${bgColor}" />${svgBlocks}</svg>`;
  
  // Use btoa for base64 encoding (works in browser environments)
  return `data:image/svg+xml;base64,${btoa(svg)}`;
}
