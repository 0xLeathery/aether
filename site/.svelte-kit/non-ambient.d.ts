
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	export interface AppTypes {
		RouteId(): "/" | "/api" | "/api/signal" | "/api/signal/[room]" | "/demo" | "/docs" | "/docs/architecture" | "/docs/architecture/crdts" | "/docs/architecture/encryption" | "/docs/architecture/identity" | "/docs/architecture/networking" | "/docs/architecture/voice" | "/docs/getting-started" | "/docs/guides" | "/docs/guides/channels" | "/docs/guides/inviting-peers" | "/docs/guides/moderation" | "/docs/guides/swarms" | "/docs/guides/voice-chat" | "/docs/search.json" | "/download";
		RouteParams(): {
			"/api/signal/[room]": { room: string }
		};
		LayoutParams(): {
			"/": { room?: string };
			"/api": { room?: string };
			"/api/signal": { room?: string };
			"/api/signal/[room]": { room: string };
			"/demo": Record<string, never>;
			"/docs": Record<string, never>;
			"/docs/architecture": Record<string, never>;
			"/docs/architecture/crdts": Record<string, never>;
			"/docs/architecture/encryption": Record<string, never>;
			"/docs/architecture/identity": Record<string, never>;
			"/docs/architecture/networking": Record<string, never>;
			"/docs/architecture/voice": Record<string, never>;
			"/docs/getting-started": Record<string, never>;
			"/docs/guides": Record<string, never>;
			"/docs/guides/channels": Record<string, never>;
			"/docs/guides/inviting-peers": Record<string, never>;
			"/docs/guides/moderation": Record<string, never>;
			"/docs/guides/swarms": Record<string, never>;
			"/docs/guides/voice-chat": Record<string, never>;
			"/docs/search.json": Record<string, never>;
			"/download": Record<string, never>
		};
		Pathname(): "/" | `/api/signal/${string}` & {} | "/demo" | "/docs" | "/docs/architecture" | "/docs/architecture/crdts" | "/docs/architecture/encryption" | "/docs/architecture/identity" | "/docs/architecture/networking" | "/docs/architecture/voice" | "/docs/getting-started" | "/docs/guides" | "/docs/guides/channels" | "/docs/guides/inviting-peers" | "/docs/guides/moderation" | "/docs/guides/swarms" | "/docs/guides/voice-chat" | "/docs/search.json" | "/download";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): "/favicon.ico" | string & {};
	}
}