<script lang="ts">
  import { voiceStore } from '../../stores/voice.svelte';

  let { identity }: { identity: any } = $props();

  // Get local peer ID from identity public key (simplified for display)
  const localPeerId = identity?.short_id || '';

  async function handleJoin() {
    try {
      await voiceStore.join();
    } catch (err) {
      console.error('Failed to join voice:', err);
    }
  }

  async function handleLeave() {
    try {
      await voiceStore.leave();
    } catch (err) {
      console.error('Failed to leave voice:', err);
    }
  }

  async function handleToggleMute() {
    try {
      await voiceStore.toggleMute();
    } catch (err) {
      console.error('Failed to toggle mute:', err);
    }
  }
</script>

<div class="voice-panel">
  <div class="voice-header">
    <span class="voice-label">VOICE</span>
    <span class="participant-badge">
      {voiceStore.participantCount}/{voiceStore.maxParticipants}
    </span>
  </div>

  <div class="voice-controls">
    {#if !voiceStore.active && !voiceStore.joining}
      <button class="voice-button join-button" onclick={handleJoin}>
        [ JOIN VOICE ]
      </button>
    {:else if voiceStore.joining}
      <button class="voice-button join-button" disabled>
        CONNECTING...
      </button>
    {:else}
      <button class="voice-button leave-button" onclick={handleLeave}>
        [ LEAVE ]
      </button>
    {/if}
  </div>

  {#if voiceStore.active}
    <div class="voice-status">
      <button
        class="mute-toggle"
        class:muted={voiceStore.muted}
        onclick={handleToggleMute}
      >
        <div class="mic-indicator" class:muted={voiceStore.muted}>
          <div class="mic-pulse" class:muted={voiceStore.muted}></div>
          <span class="mic-label">
            {voiceStore.muted ? 'MIC MUTED' : 'MIC LIVE'}
          </span>
        </div>
      </button>

      {#if voiceStore.participants.length > 0}
        <div class="participant-list">
          <div class="participant-list-header">Participants:</div>
          {#each voiceStore.participants as peerId}
            <div class="participant-item">
              <code class="peer-id">{peerId.substring(0, 16)}...</code>
              {#if peerId.includes(localPeerId)}
                <span class="self-badge">YOU</span>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}

  {#if voiceStore.error}
    <div class="voice-error">
      {voiceStore.error}
    </div>
  {/if}
</div>

<style>
  .voice-panel {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 1rem;
    font-family: var(--font-mono);
    max-width: 400px;
    margin: 1rem auto;
  }

  .voice-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .voice-label {
    font-size: 0.9rem;
    color: var(--accent-primary);
    letter-spacing: 0.1em;
    font-weight: bold;
  }

  .participant-badge {
    font-size: 0.85rem;
    color: var(--text-secondary);
    padding: 0.25rem 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: 3px;
  }

  .voice-controls {
    margin-bottom: 1rem;
  }

  .voice-button {
    width: 100%;
    padding: 0.75rem 1rem;
    font-family: var(--font-mono);
    font-size: 0.9rem;
    font-weight: bold;
    letter-spacing: 0.05em;
    background: transparent;
    cursor: pointer;
    border-radius: 4px;
    transition: all 0.2s ease;
  }

  .join-button {
    border: 2px solid var(--accent-primary);
    color: var(--accent-primary);
  }

  .join-button:hover:not(:disabled) {
    background: var(--accent-primary);
    color: var(--bg-primary);
    box-shadow: 0 0 10px var(--accent-primary);
  }

  .join-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .leave-button {
    border: 2px solid #ff4444;
    color: #ff4444;
  }

  .leave-button:hover {
    background: #ff4444;
    color: var(--bg-primary);
    box-shadow: 0 0 10px #ff4444;
  }

  .voice-status {
    border-top: 1px solid var(--border-color);
    padding-top: 1rem;
  }

  .mute-toggle {
    width: 100%;
    padding: 0.5rem;
    background: transparent;
    border: none;
    cursor: pointer;
    border-radius: 4px;
    transition: background 0.2s ease;
    margin-bottom: 1rem;
  }

  .mute-toggle:hover {
    background: var(--bg-tertiary);
  }

  .mic-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .mic-pulse {
    width: 10px;
    height: 10px;
    background: var(--accent-primary);
    border-radius: 50%;
    animation: pulse 1.5s ease-in-out infinite;
  }

  .mic-pulse.muted {
    background: #ff4444;
    animation: none;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.6;
      transform: scale(1.2);
    }
  }

  .mic-label {
    font-size: 0.85rem;
    color: var(--accent-primary);
    letter-spacing: 0.05em;
    font-weight: bold;
  }

  .mic-indicator.muted .mic-label {
    color: #ff4444;
  }

  .participant-list {
    margin-top: 1rem;
  }

  .participant-list-header {
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin-bottom: 0.5rem;
    letter-spacing: 0.05em;
  }

  .participant-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 0;
  }

  .peer-id {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--text-primary);
  }

  .self-badge {
    font-size: 0.7rem;
    color: var(--accent-secondary);
    padding: 0.1rem 0.3rem;
    border: 1px solid var(--accent-secondary);
    border-radius: 2px;
    letter-spacing: 0.05em;
  }

  .voice-error {
    margin-top: 1rem;
    padding: 0.5rem;
    background: rgba(255, 68, 68, 0.1);
    border: 1px solid #ff4444;
    border-radius: 3px;
    color: #ff4444;
    font-size: 0.85rem;
  }
</style>
