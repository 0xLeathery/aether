import { json } from "@sveltejs/kit";
const ROOM_TTL_MS = 5 * 60 * 1e3;
const MAX_ROOM_MESSAGES = 50;
const rooms = /* @__PURE__ */ new Map();
function cleanExpiredRooms() {
  const now = Date.now();
  for (const [id, room] of rooms) {
    if (now - room.created > ROOM_TTL_MS) {
      rooms.delete(id);
    }
  }
}
const POST = async ({ params, request }) => {
  cleanExpiredRooms();
  const roomId = params.room;
  let body;
  try {
    body = await request.json();
  } catch {
    return json({ error: "Invalid JSON" }, { status: 400 });
  }
  const { from, type, data } = body;
  if (typeof from !== "string" || from.length === 0) {
    return json({ error: "Invalid signal message" }, { status: 400 });
  }
  if (type !== "offer" && type !== "answer") {
    return json({ error: "Invalid signal message" }, { status: 400 });
  }
  if (typeof data !== "string" || data.length === 0) {
    return json({ error: "Invalid signal message" }, { status: 400 });
  }
  if (!rooms.has(roomId)) {
    rooms.set(roomId, { created: Date.now(), messages: [] });
  }
  const room = rooms.get(roomId);
  if (room.messages.length >= MAX_ROOM_MESSAGES) {
    return json({ error: "Room full" }, { status: 429 });
  }
  room.messages.push({
    from,
    type,
    data,
    timestamp: Date.now()
  });
  return json({ ok: true });
};
const GET = async ({ params, url }) => {
  cleanExpiredRooms();
  const roomId = params.room;
  const after = Number(url.searchParams.get("after") ?? "0");
  const exclude = url.searchParams.get("exclude") ?? "";
  const room = rooms.get(roomId);
  if (!room) {
    return json({ messages: [] });
  }
  const messages = room.messages.filter(
    (msg) => msg.timestamp > after && msg.from !== exclude
  );
  return json({ messages });
};
export {
  GET,
  POST
};
