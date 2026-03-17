export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.ico"]),
	mimeTypes: {},
	_: {
		client: {start:"_app/immutable/entry/start.CdLdvnN2.js",app:"_app/immutable/entry/app.Djex80FQ.js",imports:["_app/immutable/entry/start.CdLdvnN2.js","_app/immutable/chunks/CgMBreGP.js","_app/immutable/chunks/0fw90vus.js","_app/immutable/chunks/CQHQo5ti.js","_app/immutable/entry/app.Djex80FQ.js","_app/immutable/chunks/Dp1pzeXC.js","_app/immutable/chunks/0fw90vus.js","_app/immutable/chunks/CdNLware.js","_app/immutable/chunks/DFIm6B6S.js","_app/immutable/chunks/CQHQo5ti.js","_app/immutable/chunks/Bh0EakZh.js","_app/immutable/chunks/BAAzKEpc.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/4.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/api/signal/[room]",
				pattern: /^\/api\/signal\/([^/]+?)\/?$/,
				params: [{"name":"room","optional":false,"rest":false,"chained":false}],
				page: null,
				endpoint: __memo(() => import('./entries/endpoints/api/signal/_room_/_server.ts.js'))
			},
			{
				id: "/demo",
				pattern: /^\/demo\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		prerendered_routes: new Set(["/","/docs","/docs/architecture","/docs/architecture/crdts","/docs/architecture/encryption","/docs/architecture/identity","/docs/architecture/networking","/docs/architecture/voice","/docs/getting-started","/docs/guides","/docs/guides/channels","/docs/guides/inviting-peers","/docs/guides/moderation","/docs/guides/swarms","/docs/guides/voice-chat","/docs/search.json","/download"]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
