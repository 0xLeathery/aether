<script lang="ts">
  import type { ChatMessage } from '../../tauri';
  import { contactsStore } from '../../stores/contacts.svelte';
  import { moderationStore } from '../../stores/moderation.svelte';
  import ContactEditor from '../contacts/ContactEditor.svelte';
  import PeerContextMenu from '../moderation/PeerContextMenu.svelte';
  import BlockConfirmDialog from '../moderation/BlockConfirmDialog.svelte';

  let { messages, currentUserKey, swarmId }: {
    messages: ChatMessage[];
    currentUserKey: string;
    swarmId: string;
  } = $props();

  let scrollContainer: HTMLDivElement | undefined = $state();
  let editingContact = $state<{ publicKey: string; x: number; y: number } | null>(null);
  let contextMenu = $state<{ publicKey: string; x: number; y: number } | null>(null);
  let showBlockConfirm = $state<string | null>(null);
  let revealedIds = $state<Set<string>>(new Set());

  // Auto-scroll to bottom when messages change
  $effect(() => {
    // Access messages.length to track changes
    const _len = messages.length;
    if (scrollContainer) {
      // Use tick-like delay to let DOM update
      requestAnimationFrame(() => {
        if (scrollContainer) {
          scrollContainer.scrollTop = scrollContainer.scrollHeight;
        }
      });
    }
  });

  interface ContentPart {
    type: 'text' | 'mention';
    value: string;
    publicKey?: string;
  }

  function renderMentionContent(content: string, mentions: string[]): ContentPart[] {
    const parts: ContentPart[] = [];
    const regex = /@\[([a-f0-9]+)\]/g;
    let lastIndex = 0;
    let match;

    while ((match = regex.exec(content)) !== null) {
      if (match.index > lastIndex) {
        parts.push({ type: 'text', value: content.slice(lastIndex, match.index) });
      }
      const pubkeyRef = match[1];
      // Find the full public key from the mentions array that starts with this prefix
      const fullKey = mentions.find(m => m.startsWith(pubkeyRef)) ?? pubkeyRef;
      const displayName = contactsStore.resolveName(fullKey, pubkeyRef.substring(0, 8) + '...');
      parts.push({ type: 'mention', value: `@${displayName}`, publicKey: fullKey });
      lastIndex = match.index + match[0].length;
    }

    if (lastIndex < content.length) {
      parts.push({ type: 'text', value: content.slice(lastIndex) });
    }

    return parts.length > 0 ? parts : [{ type: 'text', value: content }];
  }

  function handleMentionClick(publicKey: string, event: MouseEvent) {
    event.stopPropagation();
    editingContact = { publicKey, x: event.clientX, y: event.clientY };
  }

  function getContactPetname(publicKey: string): string | null {
    const contact = contactsStore.contacts.find(c => c.public_key_hex === publicKey);
    return contact?.petname ?? null;
  }

  function isMentioningMe(msg: ChatMessage): boolean {
    return (msg.mentions ?? []).includes(currentUserKey);
  }

  function formatTime(timestamp: number): string {
    const date = new Date(timestamp);
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false });
  }

  function formatDate(timestamp: number): string {
    const date = new Date(timestamp);
    return date.toLocaleDateString([], { year: 'numeric', month: 'short', day: 'numeric' });
  }

  function getDateKey(timestamp: number): string {
    const date = new Date(timestamp);
    return `${date.getFullYear()}-${date.getMonth()}-${date.getDate()}`;
  }

  function getSenderDisplay(msg: ChatMessage): string {
    return contactsStore.resolveName(msg.sender_key, msg.sender_name);
  }

  function isOwnMessage(msg: ChatMessage): boolean {
    return msg.sender_key === currentUserKey;
  }

  function shouldShowDateSeparator(index: number): boolean {
    if (index === 0) return true;
    return getDateKey(messages[index].timestamp) !== getDateKey(messages[index - 1].timestamp);
  }

  function isToday(timestamp: number): boolean {
    const today = new Date();
    const date = new Date(timestamp);
    return date.getFullYear() === today.getFullYear()
      && date.getMonth() === today.getMonth()
      && date.getDate() === today.getDate();
  }

  function handleMessageContextMenu(senderKey: string, event: MouseEvent) {
    event.preventDefault();
    if (senderKey !== currentUserKey) {
      contextMenu = { publicKey: senderKey, x: event.clientX, y: event.clientY };
    }
  }

  function handleSetTier(publicKey: string, tier: 'mute' | 'hide' | 'block') {
    moderationStore.setTier(publicKey, tier);
  }

  function handleRemoveTier(publicKey: string) {
    moderationStore.removeTier(publicKey);
  }

  function handleBlockConfirm(publicKey: string) {
    moderationStore.setTier(publicKey, 'block');
    showBlockConfirm = null;
  }

  function revealMessage(id: string) {
    // Create new Set for reactivity
    revealedIds = new Set([...revealedIds, id]);
  }
</script>

<div class="message-list" bind:this={scrollContainer}>
  {#if messages.length === 0}
    <div class="empty-state">
      <span class="empty-text">No messages yet. Start the conversation.</span>
    </div>
  {:else}
    {#each messages as msg, i}
      {#if shouldShowDateSeparator(i)}
        <div class="date-separator">
          <span class="date-label">
            {isToday(msg.timestamp) ? 'Today' : formatDate(msg.timestamp)}
          </span>
        </div>
      {/if}
      {#if moderationStore.isBlocked(msg.sender_key, swarmId)}
        <!-- Blocked: fully hidden, zero DOM footprint -->
      {:else if moderationStore.isHidden(msg.sender_key, swarmId) && !revealedIds.has(msg.id)}
        <div class="message-row message-hidden-placeholder">
          <span class="msg-time">{formatTime(msg.timestamp)}</span>
          <button class="hidden-msg-btn" onclick={() => revealMessage(msg.id)}>
            Message from hidden user
          </button>
        </div>
      {:else}
        <div class="message-row" class:mentioned={isMentioningMe(msg)}>
          <span class="msg-time">{formatTime(msg.timestamp)}</span>
          <span 
            class="msg-sender"
            oncontextmenu={(e) => handleMessageContextMenu(msg.sender_key, e)}
          >
            {getSenderDisplay(msg)}
            {#if isOwnMessage(msg)}
              <span class="you-badge">YOU</span>
            {/if}
          </span>
          <span class="msg-content">
            {#each renderMentionContent(msg.content, msg.mentions ?? []) as part}
              {#if part.type === 'mention'}
                <button
                  class="mention-link"
                  onclick={(e) => handleMentionClick(part.publicKey!, e)}
                >
                  {part.value}
                </button>
              {:else}
                {part.value}
              {/if}
            {/each}
          </span>
        </div>
      {/if}
    {/each}
  {/if}
</div>

{#if editingContact}
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div class="mention-editor-backdrop" onclick={() => editingContact = null}></div>
  <div class="mention-editor-popup" style="left: {editingContact.x}px; top: {editingContact.y}px;">
    <ContactEditor
      publicKey={editingContact.publicKey}
      currentPetname={getContactPetname(editingContact.publicKey)}
      onClose={() => editingContact = null}
    />
  </div>
{/if}

{#if contextMenu}
  <PeerContextMenu
    x={contextMenu.x}
    y={contextMenu.y}
    publicKey={contextMenu.publicKey}
    currentTier={moderationStore.getEffectiveTier(contextMenu.publicKey, swarmId)}
    onMute={() => { handleSetTier(contextMenu!.publicKey, 'mute'); contextMenu = null; }}
    onHide={() => { handleSetTier(contextMenu!.publicKey, 'hide'); contextMenu = null; }}
    onBlock={() => { showBlockConfirm = contextMenu!.publicKey; contextMenu = null; }}
    onRemove={() => { handleRemoveTier(contextMenu!.publicKey); contextMenu = null; }}
    onClose={() => contextMenu = null}
  />
{/if}

{#if showBlockConfirm}
  <BlockConfirmDialog
    peerName={contactsStore.resolveName(showBlockConfirm, '')}
    onConfirm={() => handleBlockConfirm(showBlockConfirm!)}
    onCancel={() => showBlockConfirm = null}
  />
{/if}

<style>
  .message-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-height: 0;
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .empty-text {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
    letter-spacing: 0.02em;
  }

  .date-separator {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem 0;
    margin: 0.25rem 0;
  }

  .date-label {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    letter-spacing: 0.05em;
    text-transform: uppercase;
    padding: 0.125rem 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 2px;
  }

  .message-row {
    display: flex;
    gap: 0.5rem;
    padding: 0.25rem 0;
    font-family: var(--font-mono);
    font-size: 0.85rem;
    line-height: 1.4;
    align-items: baseline;
  }

  .msg-time {
    color: var(--text-muted);
    font-size: 0.8rem;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .msg-sender {
    color: var(--accent-primary);
    font-weight: 600;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .msg-sender::after {
    content: ':';
  }

  .you-badge {
    font-size: 0.65rem;
    color: var(--accent-amber);
    padding: 0.05rem 0.25rem;
    border: 1px solid var(--accent-amber);
    border-radius: 2px;
    letter-spacing: 0.05em;
    margin-left: 0.25rem;
    font-weight: bold;
    vertical-align: middle;
  }

  .msg-content {
    color: var(--text-primary);
    word-break: break-word;
    min-width: 0;
  }

  .message-row.mentioned {
    border-left: 3px solid var(--accent-amber, #ffb000);
    background: rgba(255, 176, 0, 0.05);
    padding-left: calc(0.25rem - 3px);
    margin-left: -3px;
  }

  .mention-link {
    color: var(--accent-amber, #ffb000);
    font-weight: 600;
    cursor: pointer;
    background: none;
    border: none;
    padding: 0;
    font-family: inherit;
    font-size: inherit;
    line-height: inherit;
  }

  .mention-link:hover {
    text-decoration: underline;
  }

  .mention-editor-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 199;
  }

  .mention-editor-popup {
    position: fixed;
    z-index: 200;
  }

  .message-hidden-placeholder {
    opacity: 0.5;
    font-style: italic;
  }

  .hidden-msg-btn {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--text-muted);
    background: none;
    border: 1px dashed var(--border-color);
    padding: 0.2rem 0.5rem;
    cursor: pointer;
    font-style: italic;
  }

  .hidden-msg-btn:hover {
    color: var(--text-secondary);
    border-color: var(--text-muted);
  }
</style>
