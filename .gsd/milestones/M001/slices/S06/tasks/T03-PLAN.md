# T03: 05-persistent-chat 03

**Slice:** S06 — **Milestone:** M001

## Description

Build the terminal-aesthetic chat UI: message list, text input, and integration into the main content area.

Purpose: This is the user-facing layer that makes persistent chat tangible. Users see messages, type replies, and watch conversations sync in real-time. The terminal aesthetic maintains visual consistency with the rest of Aether.

Output: Three chat components (ChatPanel, MessageList, MessageInput) integrated into MainContent, with a human verification checkpoint to confirm the full send/receive/persist flow works.

## Must-Haves

- [ ] "User can see a scrollable list of messages with sender name, content, and timestamp"
- [ ] "User can type a message and send it by pressing Enter or clicking Send"
- [ ] "Sent message appears immediately in the message list"
- [ ] "User sees display name (or truncated public key) next to each message"
- [ ] "Messages persist across app restarts (reloading shows previous messages)"
- [ ] "Chat panel appears when a swarm and channel are active"

## Files

- `src/lib/components/chat/ChatPanel.svelte`
- `src/lib/components/chat/MessageList.svelte`
- `src/lib/components/chat/MessageInput.svelte`
- `src/lib/components/layout/MainContent.svelte`
