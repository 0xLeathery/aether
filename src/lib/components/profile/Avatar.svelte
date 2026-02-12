<script lang="ts">
  import { onMount } from 'svelte';

  let { publicKeyHex, size = 40 } = $props<{ publicKeyHex: string; size?: number }>();

  let canvas: HTMLCanvasElement;

  // Simple blockies-style deterministic avatar generator
  function generateAvatar() {
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Configuration
    const gridSize = 8;
    const pixelSize = size / gridSize;

    // Color palette (terminal aesthetic - greens, ambers, teals)
    const colors = [
      '#00ff41', // primary green
      '#00d936', // darker green
      '#00a82b', // even darker
      '#ffb000', // amber
      '#ff8800', // orange
      '#00ffcc', // teal
      '#00d9aa', // darker teal
    ];

    // Use the hex string to generate deterministic pattern
    const seed = publicKeyHex.substring(0, 32);

    // Helper to get deterministic value from seed
    function getValue(index: number): number {
      const charCode = seed.charCodeAt(index % seed.length);
      return charCode;
    }

    // Pick a color based on first few characters
    const colorIndex = getValue(0) % colors.length;
    const bgColor = '#1a1a1a'; // dark background
    const fgColor = colors[colorIndex];

    // Clear canvas
    ctx.fillStyle = bgColor;
    ctx.fillRect(0, 0, size, size);

    // Generate symmetric grid pattern
    const halfGrid = Math.floor(gridSize / 2);
    for (let y = 0; y < gridSize; y++) {
      for (let x = 0; x < halfGrid; x++) {
        const index = y * halfGrid + x;
        const value = getValue(index);

        // Determine if this pixel should be filled
        if (value % 2 === 0) {
          ctx.fillStyle = fgColor;
          // Draw pixel and its mirror
          ctx.fillRect(x * pixelSize, y * pixelSize, pixelSize, pixelSize);
          ctx.fillRect((gridSize - 1 - x) * pixelSize, y * pixelSize, pixelSize, pixelSize);
        }
      }
    }
  }

  onMount(() => {
    generateAvatar();
  });

  $effect(() => {
    // Regenerate when publicKeyHex or size changes
    publicKeyHex;
    size;
    generateAvatar();
  });
</script>

<canvas
  bind:this={canvas}
  width={size}
  height={size}
  class="avatar"
  style="width: {size}px; height: {size}px;"
></canvas>

<style>
  .avatar {
    display: block;
    image-rendering: pixelated;
    image-rendering: -moz-crisp-edges;
    image-rendering: crisp-edges;
  }
</style>
