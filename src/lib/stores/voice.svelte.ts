import { joinVoice, leaveVoice, getVoiceStatus, toggleMute, onVoiceSessionJoined, onVoiceSessionLeft, onVoiceMuteChanged, type VoiceStatus, type UnlistenFn } from '../tauri';

let active = $state(false);
let muted = $state(false);
let participants = $state<string[]>([]);
let participantCount = $state(0);
let maxParticipants = 8;
let joining = $state(false);
let error = $state<string | null>(null);
let initialized = $state(false);

let unlistenJoined: UnlistenFn | null = null;
let unlistenLeft: UnlistenFn | null = null;
let unlistenMuteChanged: UnlistenFn | null = null;

async function initialize() {
  if (initialized) return;

  try {
    // Get initial voice status
    const status = await getVoiceStatus();
    active = status.active;
    muted = status.muted;
    participants = status.participants;
    participantCount = status.participant_count;

    // Set up event listeners
    unlistenJoined = await onVoiceSessionJoined((peerIds: string[]) => {
      active = true;
      participants = peerIds;
      participantCount = peerIds.length;
      error = null;
    });

    unlistenLeft = await onVoiceSessionLeft(() => {
      active = false;
      muted = false;
      participants = [];
      participantCount = 0;
      error = null;
    });

    unlistenMuteChanged = await onVoiceMuteChanged((isMuted: boolean) => {
      muted = isMuted;
    });

    initialized = true;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to initialize voice';
    console.error('Voice initialization error:', err);
  }
}

async function join() {
  if (joining || active) return;

  joining = true;
  error = null;

  try {
    const status = await joinVoice();
    active = status.active;
    participants = status.participants;
    participantCount = status.participant_count;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to join voice session';
    console.error('Voice join error:', err);
    throw err;
  } finally {
    joining = false;
  }
}

async function leave() {
  if (!active) return;

  error = null;

  try {
    await leaveVoice();
    active = false;
    muted = false;
    participants = [];
    participantCount = 0;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to leave voice session';
    console.error('Voice leave error:', err);
    throw err;
  }
}

async function toggleMuteAction() {
  if (!active) return;
  try {
    const newMuted = await toggleMute();
    muted = newMuted;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to toggle mute';
    console.error('Mute toggle error:', err);
  }
}

function cleanup() {
  if (unlistenJoined) {
    unlistenJoined();
    unlistenJoined = null;
  }
  if (unlistenLeft) {
    unlistenLeft();
    unlistenLeft = null;
  }
  if (unlistenMuteChanged) {
    unlistenMuteChanged();
    unlistenMuteChanged = null;
  }
  active = false;
  muted = false;
  participants = [];
  participantCount = 0;
  joining = false;
  error = null;
  initialized = false;
}

export const voiceStore = {
  get active() { return active; },
  get muted() { return muted; },
  get participants() { return participants; },
  get participantCount() { return participantCount; },
  get maxParticipants() { return maxParticipants; },
  get joining() { return joining; },
  get error() { return error; },
  get initialized() { return initialized; },
  initialize,
  join,
  leave,
  toggleMute: toggleMuteAction,
  cleanup,
};
