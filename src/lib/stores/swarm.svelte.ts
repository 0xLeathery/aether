import { createSwarm, joinSwarm, listSwarms, switchSwarm, type SwarmMetadata } from '../tauri';

let swarms = $state<SwarmMetadata[]>([]);
let activeSwarm = $state<SwarmMetadata | null>(null);
let loading = $state(false);
let error = $state<string | null>(null);

async function initialize() {
  loading = true;
  error = null;

  try {
    // Load swarm metadata WITHOUT auto-selecting (defers network restart)
    swarms = await listSwarms();
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
};
