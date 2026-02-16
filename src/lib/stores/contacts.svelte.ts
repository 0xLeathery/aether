import { setPetname, removePetname, getContacts, type Contact } from '../tauri';

let contacts = $state<Contact[]>([]);
let loading = $state(false);
let error = $state<string | null>(null);
let initialized = $state(false);

async function initialize() {
  if (initialized) return;

  loading = true;
  error = null;

  try {
    contacts = await getContacts();
    initialized = true;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to load contacts';
    console.error('Contacts initialization error:', err);
  } finally {
    loading = false;
  }
}

function resolveName(publicKey: string, fallbackName: string): string {
  const contact = contacts.find(c => c.public_key_hex === publicKey);
  if (contact?.petname) {
    return contact.petname;
  }
  if (fallbackName && fallbackName.length > 0) {
    return fallbackName;
  }
  return publicKey.substring(0, 8) + '...';
}

async function setPetnameAction(publicKey: string, petname: string) {
  error = null;

  try {
    await setPetname(publicKey, petname);

    // Update local state reactively (no re-fetch needed)
    const existing = contacts.find(c => c.public_key_hex === publicKey);
    if (existing) {
      existing.petname = petname;
      // Trigger reactivity by reassigning array
      contacts = [...contacts];
    } else {
      const now = Math.floor(Date.now() / 1000);
      contacts = [...contacts, {
        public_key_hex: publicKey,
        petname,
        notes: null,
        added_at: now,
      }];
    }
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to set petname';
    console.error('Set petname error:', err);
    throw err;
  }
}

async function removePetnameAction(publicKey: string) {
  error = null;

  try {
    await removePetname(publicKey);

    // Update local state reactively
    const existing = contacts.find(c => c.public_key_hex === publicKey);
    if (existing) {
      if (existing.notes === null) {
        // Remove contact entirely if no notes either
        contacts = contacts.filter(c => c.public_key_hex !== publicKey);
      } else {
        existing.petname = null;
        contacts = [...contacts];
      }
    }
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to remove petname';
    console.error('Remove petname error:', err);
    throw err;
  }
}

async function refresh() {
  loading = true;
  error = null;

  try {
    contacts = await getContacts();
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to refresh contacts';
    console.error('Contacts refresh error:', err);
  } finally {
    loading = false;
  }
}

export const contactsStore = {
  get contacts() { return contacts; },
  get loading() { return loading; },
  get error() { return error; },
  get initialized() { return initialized; },
  initialize,
  resolveName,
  setPetname: setPetnameAction,
  removePetname: removePetnameAction,
  refresh,
};
