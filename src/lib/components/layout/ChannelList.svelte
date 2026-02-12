<div class="channel-list">
  <div class="header">
    <h2>CHANNELS</h2>
  </div>

  <div class="empty-state">
    <p>Join a swarm to see channels</p>
  </div>
</div>

<style>
  .channel-list {
    width: 200px;
    background: var(--bg-primary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .header {
    padding: 1rem;
    border-bottom: 1px solid var(--border-color);
  }

  .header h2 {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
    letter-spacing: 0.15em;
    font-weight: 500;
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
    text-align: center;
  }

  .empty-state p {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-secondary);
    line-height: 1.5;
  }
</style>
