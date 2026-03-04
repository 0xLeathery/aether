// ---------------------------------------------------------------------------
// WebRTC Connection Manager
//
// Self-contained module managing the full WebRTC lifecycle for the P2P demo.
// Zero external dependencies -- uses only native WebRTC APIs and fetch.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

export type DemoState =
	| 'idle'
	| 'creating'
	| 'waiting'
	| 'connecting'
	| 'connected'
	| 'disconnected'
	| 'failed';

export interface ChatMessage {
	id: string;
	from: string;
	fromName: string;
	text: string;
	timestamp: number;
	isLocal: boolean;
}

export interface PeerInfo {
	id: string;
	name: string;
	state: 'connecting' | 'connected' | 'disconnected';
}

export interface PeerManagerConfig {
	roomId: string;
	peerId: string;
	peerName: string;
	isInitiator: boolean;
	onStateChange: (state: DemoState) => void;
	onMessage: (msg: ChatMessage) => void;
	onPeerJoin: (peer: PeerInfo) => void;
	onPeerLeave: (peerId: string) => void;
	onError: (error: string) => void;
}

export interface PeerManager {
	start: () => Promise<void>;
	sendMessage: (text: string) => void;
	destroy: () => void;
}

// ---------------------------------------------------------------------------
// ICE configuration (STUN-only, no TURN -- stays true to "no relay" philosophy)
// ---------------------------------------------------------------------------

const ICE_CONFIG: RTCConfiguration = {
	iceServers: [
		{ urls: 'stun:stun.l.google.com:19302' },
		{ urls: 'stun:stun1.l.google.com:19302' },
	],
};

const ICE_GATHER_TIMEOUT_MS = 10_000;
const POLL_INTERVAL_MS = 1_500;

// ---------------------------------------------------------------------------
// Helper exports
// ---------------------------------------------------------------------------

export function generatePeerId(): string {
	return crypto.randomUUID();
}

export function generatePeerName(): string {
	const bytes = new Uint8Array(2);
	crypto.getRandomValues(bytes);
	const hex = Array.from(bytes)
		.map((b) => b.toString(16).padStart(2, '0'))
		.join('');
	return `Peer-${hex}`;
}

export function generateRoomId(): string {
	return crypto.randomUUID();
}

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

interface PeerConnection {
	pc: RTCPeerConnection;
	channel: RTCDataChannel | null;
}

interface SignalPayload {
	from: string;
	type: 'offer' | 'answer';
	data: string;
	timestamp: number;
}

// ---------------------------------------------------------------------------
// ICE gathering helper
// ---------------------------------------------------------------------------

function gatherICECandidates(pc: RTCPeerConnection): Promise<RTCSessionDescription> {
	return new Promise<RTCSessionDescription>((resolve, reject) => {
		// If already complete, resolve immediately
		if (pc.iceGatheringState === 'complete') {
			if (pc.localDescription) {
				resolve(pc.localDescription);
			} else {
				reject(new Error('ICE gathering complete but no local description'));
			}
			return;
		}

		let settled = false;

		const finish = () => {
			if (settled) return;
			settled = true;
			clearTimeout(timer);
			pc.removeEventListener('icegatheringstatechange', onGatherChange);

			if (pc.localDescription) {
				resolve(pc.localDescription);
			} else {
				reject(new Error('ICE gathering produced no local description'));
			}
		};

		const onGatherChange = () => {
			if (pc.iceGatheringState === 'complete') {
				finish();
			}
		};

		pc.addEventListener('icegatheringstatechange', onGatherChange);

		const timer = setTimeout(() => {
			finish();
		}, ICE_GATHER_TIMEOUT_MS);
	});
}

// ---------------------------------------------------------------------------
// Factory
// ---------------------------------------------------------------------------

export function createPeerManager(config: PeerManagerConfig): PeerManager {
	const {
		roomId,
		peerId,
		peerName,
		isInitiator,
		onStateChange,
		onMessage,
		onPeerJoin,
		onPeerLeave,
		onError,
	} = config;

	const connections = new Map<string, PeerConnection>();
	let pollTimer: ReturnType<typeof setInterval> | null = null;
	let lastTimestamp = 0;
	let destroyed = false;

	// -------------------------------------------------------------------
	// Signaling client (internal)
	// -------------------------------------------------------------------

	async function publish(
		type: 'offer' | 'answer',
		data: RTCSessionDescription,
	): Promise<void> {
		await fetch(`/api/signal/${roomId}`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				from: peerId,
				type,
				data: JSON.stringify(data),
			}),
		});
	}

	function startPolling(onSignal: (msg: SignalPayload) => void): void {
		if (pollTimer) return;

		pollTimer = setInterval(async () => {
			if (destroyed) {
				stopPolling();
				return;
			}

			try {
				const res = await fetch(
					`/api/signal/${roomId}?after=${lastTimestamp}&exclude=${peerId}`,
				);
				if (!res.ok) return;

				const { messages } = (await res.json()) as { messages: SignalPayload[] };
				for (const msg of messages) {
					if (msg.timestamp > lastTimestamp) {
						lastTimestamp = msg.timestamp;
					}
					onSignal(msg);
				}
			} catch {
				// Silently ignore transient network errors during polling
			}
		}, POLL_INTERVAL_MS);
	}

	function stopPolling(): void {
		if (pollTimer) {
			clearInterval(pollTimer);
			pollTimer = null;
		}
	}

	// -------------------------------------------------------------------
	// DataChannel event wiring
	// -------------------------------------------------------------------

	function wireDataChannel(
		remotePeerId: string,
		remotePeerName: string,
		channel: RTCDataChannel,
		conn: PeerConnection,
	): void {
		channel.onopen = () => {
			conn.channel = channel;
			const peerInfo: PeerInfo = {
				id: remotePeerId,
				name: remotePeerName,
				state: 'connected',
			};
			onPeerJoin(peerInfo);
			onStateChange('connected');

			// Broadcast join system message
			try {
				channel.send(
					JSON.stringify({
						from: peerId,
						fromName: peerName,
						text: `${peerName} joined`,
						timestamp: Date.now(),
						system: true,
					}),
				);
			} catch {
				// Channel may close during send
			}
		};

		channel.onmessage = (event) => {
			try {
				const parsed = JSON.parse(event.data as string) as {
					from: string;
					fromName: string;
					text: string;
					timestamp: number;
				};
				const chatMsg: ChatMessage = {
					id: crypto.randomUUID(),
					from: parsed.from,
					fromName: parsed.fromName,
					text: parsed.text,
					timestamp: parsed.timestamp,
					isLocal: false,
				};
				onMessage(chatMsg);
			} catch {
				// Ignore malformed messages
			}
		};

		channel.onclose = () => {
			conn.channel = null;
			onPeerLeave(remotePeerId);
		};
	}

	// -------------------------------------------------------------------
	// Connection state monitoring
	// -------------------------------------------------------------------

	function monitorConnection(remotePeerId: string, pc: RTCPeerConnection): void {
		pc.addEventListener('connectionstatechange', () => {
			if (destroyed) return;

			switch (pc.connectionState) {
				case 'failed':
					onStateChange('failed');
					onError(
						'Connection failed. This usually happens behind corporate firewalls or symmetric NAT. ' +
							'Try a different network (like a mobile hotspot) or download the desktop app for better connectivity.',
					);
					break;
				case 'disconnected':
					onPeerLeave(remotePeerId);
					onStateChange('disconnected');
					break;
			}
		});
	}

	// -------------------------------------------------------------------
	// Initiator flow (room creator)
	// -------------------------------------------------------------------

	async function startAsInitiator(): Promise<void> {
		onStateChange('creating');

		// Signal room creation by noting our presence (handled via polling)
		onStateChange('waiting');

		startPolling(async (signal) => {
			if (destroyed) return;
			if (signal.type !== 'offer') return;

			// Another peer sent an offer -- respond with an answer
			const remotePeerId = signal.from;

			// Avoid duplicate connections
			if (connections.has(remotePeerId)) return;

			try {
				const pc = new RTCPeerConnection(ICE_CONFIG);
				const conn: PeerConnection = { pc, channel: null };
				connections.set(remotePeerId, conn);

				monitorConnection(remotePeerId, pc);

				// Listen for DataChannel created by the joiner
				pc.ondatachannel = (event) => {
					wireDataChannel(remotePeerId, `Peer-${remotePeerId.slice(0, 4)}`, event.channel, conn);
				};

				// Set remote offer
				const offerDesc = JSON.parse(signal.data) as RTCSessionDescriptionInit;
				await pc.setRemoteDescription(new RTCSessionDescription(offerDesc));

				// Create and set answer
				const answer = await pc.createAnswer();
				await pc.setLocalDescription(answer);

				onStateChange('connecting');

				// Wait for ICE gathering
				const fullAnswer = await gatherICECandidates(pc);

				// Publish the answer
				await publish('answer', fullAnswer);
			} catch (err) {
				onError(`Failed to handle incoming offer: ${err instanceof Error ? err.message : String(err)}`);
			}
		});
	}

	// -------------------------------------------------------------------
	// Joiner flow (clicking a share link)
	// -------------------------------------------------------------------

	async function startAsJoiner(): Promise<void> {
		onStateChange('connecting');

		try {
			const pc = new RTCPeerConnection(ICE_CONFIG);
			const conn: PeerConnection = { pc, channel: null };

			// We don't know the remote peer's ID yet; use a placeholder key
			const placeholderKey = '__initiator__';
			connections.set(placeholderKey, conn);

			monitorConnection(placeholderKey, pc);

			// Create DataChannel (joiner creates, initiator receives)
			const channel = pc.createDataChannel('chat', { ordered: true });
			wireDataChannel(placeholderKey, 'Initiator', channel, conn);

			// Create and set offer
			const offer = await pc.createOffer();
			await pc.setLocalDescription(offer);

			// Wait for ICE gathering
			const fullOffer = await gatherICECandidates(pc);

			// Publish the offer
			await publish('offer', fullOffer);

			// Poll for the answer
			startPolling(async (signal) => {
				if (destroyed) return;
				if (signal.type !== 'answer') return;

				try {
					const answerDesc = JSON.parse(signal.data) as RTCSessionDescriptionInit;
					await pc.setRemoteDescription(new RTCSessionDescription(answerDesc));

					// Re-key the connection with the actual remote peer ID
					if (connections.has(placeholderKey)) {
						connections.delete(placeholderKey);
						conn.channel = channel;
						connections.set(signal.from, { pc, channel });
					}

					// Stop polling once we have the answer
					stopPolling();
				} catch (err) {
					onError(`Failed to process answer: ${err instanceof Error ? err.message : String(err)}`);
				}
			});
		} catch (err) {
			onStateChange('failed');
			onError(`Failed to create offer: ${err instanceof Error ? err.message : String(err)}`);
		}
	}

	// -------------------------------------------------------------------
	// Public API
	// -------------------------------------------------------------------

	async function start(): Promise<void> {
		if (destroyed) return;

		if (isInitiator) {
			await startAsInitiator();
		} else {
			await startAsJoiner();
		}
	}

	function sendMessage(text: string): void {
		if (destroyed) return;

		const payload = JSON.stringify({
			from: peerId,
			fromName: peerName,
			text,
			timestamp: Date.now(),
		});

		// Send to all connected peers
		for (const [, conn] of connections) {
			if (conn.channel && conn.channel.readyState === 'open') {
				try {
					conn.channel.send(payload);
				} catch {
					// Ignore send failures on closed channels
				}
			}
		}

		// Also fire local callback so UI displays the message
		const localMsg: ChatMessage = {
			id: crypto.randomUUID(),
			from: peerId,
			fromName: peerName,
			text,
			timestamp: Date.now(),
			isLocal: true,
		};
		onMessage(localMsg);
	}

	function destroy(): void {
		if (destroyed) return;
		destroyed = true;

		stopPolling();

		for (const [, conn] of connections) {
			if (conn.channel) {
				try {
					conn.channel.close();
				} catch {
					// Ignore
				}
			}
			try {
				conn.pc.close();
			} catch {
				// Ignore
			}
		}

		connections.clear();
		onStateChange('idle');
	}

	return { start, sendMessage, destroy };
}
