import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface SignalMessage {
	from: string;
	type: 'offer' | 'answer';
	data: string;
	timestamp: number;
}

interface Room {
	created: number;
	messages: SignalMessage[];
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const ROOM_TTL_MS = 5 * 60 * 1000; // 5 minutes
const MAX_ROOM_MESSAGES = 50;

// ---------------------------------------------------------------------------
// In-memory room store (survives between Vercel Fluid Compute invocations)
// ---------------------------------------------------------------------------

const rooms: Map<string, Room> = new Map();

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function cleanExpiredRooms(): void {
	const now = Date.now();
	for (const [id, room] of rooms) {
		if (now - room.created > ROOM_TTL_MS) {
			rooms.delete(id);
		}
	}
}

// ---------------------------------------------------------------------------
// POST  /api/signal/[room]
// ---------------------------------------------------------------------------

export const POST: RequestHandler = async ({ params, request }) => {
	cleanExpiredRooms();

	const roomId = params.room;
	let body: unknown;

	try {
		body = await request.json();
	} catch {
		return json({ error: 'Invalid JSON' }, { status: 400 });
	}

	const { from, type, data } = body as Record<string, unknown>;

	// Validate
	if (typeof from !== 'string' || from.length === 0) {
		return json({ error: 'Invalid signal message' }, { status: 400 });
	}
	if (type !== 'offer' && type !== 'answer') {
		return json({ error: 'Invalid signal message' }, { status: 400 });
	}
	if (typeof data !== 'string' || data.length === 0) {
		return json({ error: 'Invalid signal message' }, { status: 400 });
	}

	// Create room if it doesn't exist
	if (!rooms.has(roomId)) {
		rooms.set(roomId, { created: Date.now(), messages: [] });
	}

	const room = rooms.get(roomId)!;

	// Enforce message limit
	if (room.messages.length >= MAX_ROOM_MESSAGES) {
		return json({ error: 'Room full' }, { status: 429 });
	}

	room.messages.push({
		from,
		type: type as 'offer' | 'answer',
		data: data as string,
		timestamp: Date.now(),
	});

	return json({ ok: true });
};

// ---------------------------------------------------------------------------
// GET  /api/signal/[room]?after=0&exclude=peerId
// ---------------------------------------------------------------------------

export const GET: RequestHandler = async ({ params, url }) => {
	cleanExpiredRooms();

	const roomId = params.room;
	const after = Number(url.searchParams.get('after') ?? '0');
	const exclude = url.searchParams.get('exclude') ?? '';

	const room = rooms.get(roomId);
	if (!room) {
		return json({ messages: [] });
	}

	const messages = room.messages.filter(
		(msg) => msg.timestamp > after && msg.from !== exclude,
	);

	return json({ messages });
};
