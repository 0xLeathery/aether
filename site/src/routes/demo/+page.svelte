<script lang="ts">
  import {
    createPeerManager,
    generatePeerId,
    generatePeerName,
    generateRoomId,
    type DemoState,
    type ChatMessage,
    type PeerInfo,
    type PeerManager as PeerManagerType,
  } from '$lib/webrtc';
  import QrCreator from 'qr-creator';

  // ---------------------------------------------------------------------------
  // State (Svelte 5 runes)
  // ---------------------------------------------------------------------------

  let demoState: DemoState = $state('idle');
  let messages: ChatMessage[] = $state([]);
  let peers: PeerInfo[] = $state([]);
  let errorMessage: string = $state('');
  let inputText: string = $state('');
  let roomId: string = $state('');
  let myName: string = $state('');
  let copied: boolean = $state(false);
  let manager: PeerManagerType | null = $state(null);
  let qrContainer: HTMLDivElement | null = $state(null);
  let messagesContainer: HTMLDivElement | null = $state(null);

  let shareUrl: string = $derived(
    roomId && typeof window !== 'undefined'
      ? `${window.location.origin}/demo?room=${roomId}`
      : ''
  );

  let connectedPeerCount: number = $derived(
    peers.filter((p) => p.state === 'connected').length
  );

  // ---------------------------------------------------------------------------
  // Lifecycle
  // ---------------------------------------------------------------------------

  // Check URL for ?room= on mount and auto-join
  $effect(() => {
    if (typeof window === 'undefined') return;

    const params = new URLSearchParams(window.location.search);
    const room = params.get('room');
    if (room && demoState === 'idle') {
      roomId = room;
      startSession(false);
    }
  });

  // QR code rendering
  $effect(() => {
    if (shareUrl && qrContainer) {
      qrContainer.innerHTML = '';
      QrCreator.render(
        {
          text: shareUrl,
          radius: 0.4,
          ecLevel: 'M',
          fill: '#00ff41',
          background: '#111111',
          size: 200,
        },
        qrContainer
      );
    }
  });

  // Auto-scroll messages
  $effect(() => {
    if (messages.length && messagesContainer) {
      // Use tick-like delay so DOM updates first
      setTimeout(() => {
        if (messagesContainer) {
          messagesContainer.scrollTop = messagesContainer.scrollHeight;
        }
      }, 0);
    }
  });

  // Cleanup on unmount
  $effect(() => {
    return () => {
      manager?.destroy();
    };
  });

  // Cleanup on page unload
  $effect(() => {
    if (typeof window === 'undefined') return;

    const handleUnload = () => {
      manager?.destroy();
    };

    window.addEventListener('beforeunload', handleUnload);
    return () => {
      window.removeEventListener('beforeunload', handleUnload);
    };
  });

  // ---------------------------------------------------------------------------
  // Actions
  // ---------------------------------------------------------------------------

  function startSession(isInitiator: boolean) {
    if (!roomId) {
      roomId = generateRoomId();
    }

    const peerId = generatePeerId();
    myName = generatePeerName();

    manager = createPeerManager({
      roomId,
      peerId,
      peerName: myName,
      isInitiator,
      onStateChange: (state: DemoState) => {
        demoState = state;
      },
      onMessage: (msg: ChatMessage) => {
        messages = [...messages, msg];
      },
      onPeerJoin: (peer: PeerInfo) => {
        peers = [...peers.filter((p) => p.id !== peer.id), peer];
      },
      onPeerLeave: (peerId: string) => {
        peers = peers.map((p) =>
          p.id === peerId ? { ...p, state: 'disconnected' as const } : p
        );
      },
      onError: (error: string) => {
        errorMessage = error;
      },
    });

    manager.start();
  }

  function handleStartChat() {
    startSession(true);
  }

  function handleSendMessage() {
    const text = inputText.trim();
    if (!text || !manager) return;
    manager.sendMessage(text);
    inputText = '';
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSendMessage();
    }
  }

  async function handleCopyLink() {
    if (!shareUrl) return;
    try {
      await navigator.clipboard.writeText(shareUrl);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 2000);
    } catch {
      // Fallback: select text in a temporary input
      const input = document.createElement('input');
      input.value = shareUrl;
      document.body.appendChild(input);
      input.select();
      document.execCommand('copy');
      document.body.removeChild(input);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 2000);
    }
  }

  function handleReset() {
    manager?.destroy();
    manager = null;
    demoState = 'idle';
    messages = [];
    peers = [];
    errorMessage = '';
    inputText = '';
    roomId = '';
    myName = '';
    copied = false;

    // Clear URL params
    if (typeof window !== 'undefined') {
      window.history.replaceState({}, '', '/demo');
    }
  }

  function formatTime(timestamp: number): string {
    return new Date(timestamp).toLocaleTimeString([], {
      hour: '2-digit',
      minute: '2-digit',
    });
  }
</script>

<svelte:head>
  <title>Demo - Aether</title>
</svelte:head>

<div class="mx-auto max-w-4xl px-4 py-16 sm:px-6 lg:px-8">

  {#if demoState === 'idle'}
    <!-- ================================================================== -->
    <!-- IDLE STATE: Landing                                                 -->
    <!-- ================================================================== -->
    <div class="text-center">
      <h1 class="font-mono text-3xl font-bold text-accent-green sm:text-4xl">
        Try Aether in Your Browser
      </h1>
      <p class="mx-auto mt-4 max-w-2xl text-lg leading-relaxed text-text-secondary">
        Experience peer-to-peer communication without downloading anything. This
        interactive demo connects you directly to another person using WebRTC.
      </p>

      <button
        onclick={handleStartChat}
        class="mt-10 rounded-lg bg-accent-green px-8 py-4 font-mono text-lg font-semibold text-bg-primary transition-opacity hover:opacity-90"
      >
        Start Chat
      </button>

      <p class="mt-4 text-sm text-text-muted">
        No account needed. Fully ephemeral. Messages disappear on page close.
      </p>
    </div>

  {:else}
    <!-- ================================================================== -->
    <!-- CHAT WINDOW: All non-idle states                                    -->
    <!-- ================================================================== -->
    <h1 class="mb-6 font-mono text-2xl font-bold text-accent-green sm:text-3xl">
      Try Aether in Your Browser
    </h1>

    <div class="overflow-hidden rounded-lg border border-border">
      <!-- Status bar -->
      <div class="border-b border-border bg-bg-tertiary px-4 py-3">
        <div class="flex items-center gap-2">
          {#if demoState === 'creating'}
            <span class="h-3 w-3 animate-pulse rounded-full bg-accent-green/50"></span>
            <span class="font-mono text-sm text-text-muted">Creating session...</span>

          {:else if demoState === 'waiting'}
            <span class="h-3 w-3 animate-pulse rounded-full bg-accent-green"></span>
            <span class="font-mono text-sm text-text-muted">Waiting for peers...</span>

          {:else if demoState === 'connecting'}
            <span class="h-3 w-3 animate-pulse rounded-full bg-accent-amber"></span>
            <span class="font-mono text-sm text-text-muted">Exchanging connection info...</span>

          {:else if demoState === 'connected'}
            <span class="h-3 w-3 rounded-full bg-accent-green"></span>
            <span class="font-mono text-sm text-text-muted">
              Connected &mdash; messages are now direct P2P
            </span>
            {#if connectedPeerCount > 0}
              <span class="ml-auto rounded-full bg-accent-green/10 px-2 py-0.5 font-mono text-xs text-accent-green">
                {connectedPeerCount} peer{connectedPeerCount !== 1 ? 's' : ''}
              </span>
            {/if}

          {:else if demoState === 'disconnected'}
            <span class="h-3 w-3 rounded-full bg-text-muted"></span>
            <span class="font-mono text-sm text-text-muted">Peer disconnected</span>

          {:else if demoState === 'failed'}
            <span class="h-3 w-3 rounded-full bg-red-500"></span>
            <span class="font-mono text-sm text-red-400">Connection failed</span>
          {/if}
        </div>
      </div>

      <!-- Main content area -->
      <div class="relative flex flex-col bg-bg-secondary" style="min-height: 400px;">

        {#if demoState === 'waiting' || demoState === 'creating'}
          <!-- ============================================================ -->
          <!-- WAITING / CREATING: Share panel                               -->
          <!-- ============================================================ -->
          <div class="flex flex-1 flex-col items-center justify-center p-8">
            {#if demoState === 'creating'}
              <p class="font-mono text-sm text-text-muted">Setting up session...</p>
            {:else}
              <div class="w-full max-w-md space-y-6 text-center">
                <p class="font-mono text-sm text-text-muted">
                  Share this link to connect:
                </p>

                <!-- Share URL display + copy -->
                <div class="flex items-stretch gap-2">
                  <div class="flex-1 truncate rounded-lg border border-border bg-bg-tertiary px-4 py-3 font-mono text-sm text-text-primary">
                    {shareUrl}
                  </div>
                  <button
                    onclick={handleCopyLink}
                    class="shrink-0 rounded-lg border border-border bg-bg-tertiary px-4 py-3 font-mono text-sm text-text-primary transition-colors hover:border-accent-green hover:text-accent-green"
                  >
                    {copied ? 'Copied!' : 'Copy Link'}
                  </button>
                </div>

                <!-- QR Code -->
                <div class="flex flex-col items-center gap-3">
                  <div
                    bind:this={qrContainer}
                    class="overflow-hidden rounded-lg"
                  ></div>
                  <p class="text-sm text-text-muted">
                    Scan to connect from another device
                  </p>
                </div>
              </div>
            {/if}
          </div>

        {:else if demoState === 'connecting'}
          <!-- ============================================================ -->
          <!-- CONNECTING: Share panel (dimmed) + signaling status            -->
          <!-- ============================================================ -->
          <div class="flex flex-1 flex-col items-center justify-center p-8 opacity-60">
            <div class="w-full max-w-md space-y-6 text-center">
              <p class="font-mono text-sm text-text-muted">
                Share this link to connect:
              </p>
              <div class="flex items-stretch gap-2">
                <div class="flex-1 truncate rounded-lg border border-border bg-bg-tertiary px-4 py-3 font-mono text-sm text-text-primary">
                  {shareUrl}
                </div>
                <button
                  onclick={handleCopyLink}
                  class="shrink-0 rounded-lg border border-border bg-bg-tertiary px-4 py-3 font-mono text-sm text-text-primary transition-colors hover:border-accent-green hover:text-accent-green"
                >
                  {copied ? 'Copied!' : 'Copy Link'}
                </button>
              </div>
              {#if qrContainer}
                <div class="flex flex-col items-center gap-3">
                  <div bind:this={qrContainer} class="overflow-hidden rounded-lg"></div>
                </div>
              {/if}
            </div>
          </div>
          <p class="pb-4 text-center text-xs text-text-muted">
            Signaling complete once direct connection is established
          </p>

        {:else if demoState === 'connected'}
          <!-- ============================================================ -->
          <!-- CONNECTED: Chat messages + invite link                        -->
          <!-- ============================================================ -->

          <!-- Invite more peers (collapsed share) -->
          {#if connectedPeerCount < 4}
            <div class="border-b border-border bg-bg-tertiary/50 px-4 py-2 text-center">
              <button
                onclick={handleCopyLink}
                class="font-mono text-xs text-accent-green/70 transition-colors hover:text-accent-green"
              >
                {copied ? 'Link copied!' : 'Invite more peers'}
              </button>
            </div>
          {/if}

          <!-- Messages -->
          <div
            bind:this={messagesContainer}
            class="flex-1 space-y-3 overflow-y-auto p-4"
            style="min-height: 300px; max-height: 400px;"
          >
            {#each messages as msg (msg.id)}
              {#if 'system' in msg}
                <!-- System message -->
                <div class="text-center">
                  <span class="text-xs italic text-text-muted">{msg.text}</span>
                </div>
              {:else if msg.isLocal}
                <!-- Local message (right-aligned) -->
                <div class="flex justify-end">
                  <div class="max-w-[75%] rounded-lg bg-bg-tertiary px-3 py-2">
                    <div class="flex items-center justify-end gap-2">
                      <span class="text-xs text-text-muted">{formatTime(msg.timestamp)}</span>
                      <span class="font-mono text-xs text-text-muted">{msg.fromName}</span>
                    </div>
                    <p class="mt-1 text-sm text-text-primary">{msg.text}</p>
                  </div>
                </div>
              {:else}
                <!-- Remote message (left-aligned) -->
                <div class="flex justify-start">
                  <div class="max-w-[75%] rounded-lg bg-bg-secondary px-3 py-2 ring-1 ring-border">
                    <div class="flex items-center gap-2">
                      <span class="font-mono text-xs text-accent-green">{msg.fromName}</span>
                      <span class="text-xs text-text-muted">{formatTime(msg.timestamp)}</span>
                    </div>
                    <p class="mt-1 text-sm text-text-primary">{msg.text}</p>
                  </div>
                </div>
              {/if}
            {/each}

            {#if messages.length === 0}
              <div class="flex h-full items-center justify-center">
                <p class="text-sm text-text-muted">
                  Connected! Send a message to start chatting.
                </p>
              </div>
            {/if}
          </div>

        {:else if demoState === 'disconnected'}
          <!-- ============================================================ -->
          <!-- DISCONNECTED: Read-only messages + session ended               -->
          <!-- ============================================================ -->
          <div
            class="flex-1 space-y-3 overflow-y-auto p-4"
            style="min-height: 300px; max-height: 400px;"
          >
            {#each messages as msg (msg.id)}
              {#if msg.isLocal}
                <div class="flex justify-end">
                  <div class="max-w-[75%] rounded-lg bg-bg-tertiary px-3 py-2 opacity-70">
                    <div class="flex items-center justify-end gap-2">
                      <span class="text-xs text-text-muted">{formatTime(msg.timestamp)}</span>
                      <span class="font-mono text-xs text-text-muted">{msg.fromName}</span>
                    </div>
                    <p class="mt-1 text-sm text-text-primary">{msg.text}</p>
                  </div>
                </div>
              {:else}
                <div class="flex justify-start">
                  <div class="max-w-[75%] rounded-lg bg-bg-secondary px-3 py-2 opacity-70 ring-1 ring-border">
                    <div class="flex items-center gap-2">
                      <span class="font-mono text-xs text-accent-green">{msg.fromName}</span>
                      <span class="text-xs text-text-muted">{formatTime(msg.timestamp)}</span>
                    </div>
                    <p class="mt-1 text-sm text-text-primary">{msg.text}</p>
                  </div>
                </div>
              {/if}
            {/each}
          </div>

        {:else if demoState === 'failed'}
          <!-- ============================================================ -->
          <!-- FAILED: Educational failure message                           -->
          <!-- ============================================================ -->
          <div class="flex flex-1 flex-col items-center justify-center p-8">
            <div class="w-full max-w-lg rounded-lg border border-border bg-bg-tertiary p-6">
              <h2 class="font-mono text-lg font-semibold text-accent-amber">
                Why did this happen?
              </h2>
              <p class="mt-3 text-sm leading-relaxed text-text-secondary">
                Direct peer-to-peer connections sometimes fail behind corporate
                firewalls or certain types of NAT (network address translation).
                This is an inherent challenge of true P2P communication &mdash;
                the same reason Aether exists as a desktop app with more advanced
                connectivity options.
              </p>
              <ul class="mt-4 space-y-2 text-sm text-text-secondary">
                <li class="flex items-start gap-2">
                  <span class="mt-1 text-accent-amber">&bull;</span>
                  Try from a different network (mobile hotspot often works)
                </li>
                <li class="flex items-start gap-2">
                  <span class="mt-1 text-accent-amber">&bull;</span>
                  Make sure both peers aren't behind the same corporate firewall
                </li>
                <li class="flex items-start gap-2">
                  <span class="mt-1 text-accent-amber">&bull;</span>
                  Download the desktop app for better NAT traversal
                </li>
              </ul>
              <button
                onclick={handleReset}
                class="mt-6 rounded-lg border border-accent-green px-6 py-2.5 font-mono text-sm font-semibold text-accent-green transition-colors hover:bg-accent-green-bg"
              >
                Try Again
              </button>
            </div>
            {#if errorMessage}
              <p class="mt-4 max-w-lg text-xs text-text-muted">
                Technical details: {errorMessage}
              </p>
            {/if}
          </div>
        {/if}

        <!-- Input area (connected state only) -->
        {#if demoState === 'connected'}
          <div class="border-t border-border bg-bg-tertiary px-4 py-3">
            <div class="flex items-center gap-2">
              <input
                type="text"
                bind:value={inputText}
                onkeydown={handleKeydown}
                placeholder="Type a message..."
                class="flex-1 rounded-lg border border-border bg-bg-tertiary px-4 py-2.5 text-sm text-text-primary placeholder-text-muted outline-none transition-colors focus:border-accent-green"
              />
              <button
                onclick={handleSendMessage}
                disabled={!inputText.trim()}
                class="rounded-lg bg-accent-green px-4 py-2.5 font-mono text-sm font-semibold text-bg-primary transition-opacity disabled:opacity-30"
              >
                Send
              </button>
            </div>
          </div>
        {/if}

        <!-- Disconnected input replacement -->
        {#if demoState === 'disconnected'}
          <div class="border-t border-border bg-bg-tertiary px-4 py-4">
            <p class="text-center text-sm text-text-muted">
              Session ended. Messages are ephemeral and will disappear when you leave this page.
            </p>
            <div class="mt-3 text-center">
              <button
                onclick={handleReset}
                class="rounded-lg border border-accent-green px-6 py-2.5 font-mono text-sm font-semibold text-accent-green transition-colors hover:bg-accent-green-bg"
              >
                Start New Chat
              </button>
            </div>
          </div>
        {/if}

        <!-- Download banner (visible in all non-idle states) -->
        <div class="border-t border-border bg-bg-tertiary/80 px-4 py-2 text-center backdrop-blur">
          <p class="text-sm text-text-muted">
            For the complete experience with voice chat, channels, and persistent messages &mdash;
            <a
              href="/download"
              class="font-mono text-accent-green transition-colors hover:text-accent-green-dim"
            >
              Download Aether
            </a>
          </p>
        </div>
      </div>
    </div>
  {/if}
</div>
