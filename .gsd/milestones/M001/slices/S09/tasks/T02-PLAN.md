# T02: 08-unread-mentions 02

**Slice:** S09 — **Milestone:** M001

## Description

Add the mentions field to ChatMessage (Rust CRDT + TypeScript type) and build the @mention autocomplete picker in the message input, enabling users to mention peers by typing @ and selecting from a filtered popup.

Purpose: Mentions let users direct attention to specific peers. Storing public key references ensures mentions survive name changes. The autocomplete picker provides a discoverable, Discord-like UX.

Output: ChatMessage schema with mentions field (backward-compatible), MentionPicker component, MessageInput with @ detection and mention insertion.

## Must-Haves

- [ ] "User can type '@' in the message input and see an autocomplete popup of peers"
- [ ] "Autocomplete entries show resolved name + truncated public key"
- [ ] "Autocomplete filters live as user types characters after @"
- [ ] "Selecting a peer from autocomplete inserts @DisplayName into the input"
- [ ] "Sent messages store mentioned public keys in a mentions array"
- [ ] "Existing messages without mentions field load without errors (backward compat)"

## Files

- `src-tauri/src/chat/message.rs`
- `src-tauri/src/commands/chat.rs`
- `src/lib/tauri.ts`
- `src/lib/components/chat/MentionPicker.svelte`
- `src/lib/components/chat/MessageInput.svelte`
