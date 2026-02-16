import {
  createSwarm,
  joinSwarm,
  listSwarms,
  switchSwarm,
  renameSwarm,
  leaveSwarm,
  getInviteUri,
  onSwarmDeleted,
  type SwarmMetadata,
  type UnlistenFn,
} from '../tauri';

let swarms = $state<SwarmMetadata[]>([]);
let activeSwarm = $state<SwarmMetadata | null>(null);
let loading = $state(false);
let error = $state<string | null>(null);
let unlistenDeleted: UnlistenFn | null = null;

async function initialize() {
  loading = true;
  error = null;

  try {
    // Load swarm metadata WITHOUT auto-selecting (defers network restart)
    swarms = await listSwarms();

    // Listen for swarm-deleted events (e.g. from leave_swarm backend)
    if (!unlistenDeleted) {
      unlistenDeleted = await onSwarmDeleted((deletedId: string) => {
        swarms = swarms.filter(s => s.id !== deletedId);
        if (activeSwarm?.id === deletedId) {
          activeSwarm = null;
        }
      });
    }
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to initialize swarms';
    console.error('Swarm initialization error:', err);
  } finally {
    loading = false;
  }
}

async function activateDefaultSwarm() {
  // Auto-select first swarm if available (called after network starts)
  if (swarms.length > 0 && !activeSwarm) {
    await selectSwarm(swarms[0].id);
  }
}

async function createNewSwarm(name: string): Promise<string> {
  loading = true;
  error = null;

  try {
    const uri = await createSwarm(name);

    // Re-fetch swarm list
    swarms = await listSwarms();

    // Auto-select the newly created swarm (find by name + most recent created_at)
    const newSwarm = swarms
      .filter(s => s.name === name)
      .sort((a, b) => b.created_at - a.created_at)[0];

    if (newSwarm) {
      await selectSwarm(newSwarm.id);
    }

    return uri;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to create swarm';
    console.error('Swarm creation error:', err);
    throw err;
  } finally {
    loading = false;
  }
}

async function joinExistingSwarm(uri: string, name: string): Promise<void> {
  loading = true;
  error = null;

  try {
    const swarmId = await joinSwarm(uri, name);

    // Re-fetch swarm list
    swarms = await listSwarms();

    // Auto-select the newly joined swarm
    await selectSwarm(swarmId);
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to join swarm';
    console.error('Swarm join error:', err);
    throw err;
  } finally {
    loading = false;
  }
}

async function selectSwarm(swarmId: string): Promise<void> {
  loading = true;
  error = null;

  try {
    // Find swarm in local list
    const swarm = swarms.find(s => s.id === swarmId);
    if (!swarm) {
      throw new Error(`Swarm ${swarmId} not found`);
    }

    // Call Tauri command to switch swarm (restarts network with PSK)
    await switchSwarm(swarmId);

    // Set as active swarm
    activeSwarm = swarm;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to select swarm';
    console.error('Swarm selection error:', err);
    throw err;
  } finally {
    loading = false;
  }
}

async function renameSwarmAction(swarmId: string, newName: string): Promise<void> {
  try {
    await renameSwarm(swarmId, newName);

    // Update local swarms array
    swarms = swarms.map(s =>
      s.id === swarmId ? { ...s, name: newName } : s
    );

    // Update activeSwarm if it's the one being renamed
    if (activeSwarm?.id === swarmId) {
      activeSwarm = { ...activeSwarm, name: newName };
    }
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to rename swarm';
    console.error('Swarm rename error:', err);
    throw err;
  }
}

async function leaveSwarmAction(swarmId: string): Promise<void> {
  try {
    await leaveSwarm(swarmId);

    // Remove swarm from local array
    swarms = swarms.filter(s => s.id !== swarmId);

    // Clear activeSwarm if it was the one we left
    if (activeSwarm?.id === swarmId) {
      activeSwarm = null;
    }
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to leave swarm';
    console.error('Swarm leave error:', err);
    throw err;
  }
}

async function getInviteUriAction(swarmId: string): Promise<string> {
  try {
    return await getInviteUri(swarmId);
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to get invite URI';
    console.error('Invite URI error:', err);
    throw err;
  }
}

export const swarmStore = {
  get swarms() { return swarms; },
  get activeSwarm() { return activeSwarm; },
  get loading() { return loading; },
  get error() { return error; },
  initialize,
  activateDefaultSwarm,
  createNewSwarm,
  joinExistingSwarm,
  selectSwarm,
  renameSwarm: renameSwarmAction,
  leaveSwarm: leaveSwarmAction,
  getInviteUri: getInviteUriAction,
};
