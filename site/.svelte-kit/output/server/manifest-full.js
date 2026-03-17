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
			__memo(() => import('./nodes/2.js')),
			__memo(() => import('./nodes/3.js')),
			__memo(() => import('./nodes/4.js')),
			__memo(() => import('./nodes/5.js')),
			__memo(() => import('./nodes/6.js')),
			__memo(() => import('./nodes/7.js')),
			__memo(() => import('./nodes/8.js')),
			__memo(() => import('./nodes/9.js')),
			__memo(() => import('./nodes/10.js')),
			__memo(() => import('./nodes/11.js')),
			__memo(() => import('./nodes/12.js')),
			__memo(() => import('./nodes/13.js')),
			__memo(() => import('./nodes/14.js')),
			__memo(() => import('./nodes/15.js')),
			__memo(() => import('./nodes/16.js')),
			__memo(() => import('./nodes/17.js')),
			__memo(() => import('./nodes/18.js')),
			__memo(() => import('./nodes/19.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			},
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
				page: { layouts: [0,], errors: [1,], leaf: 4 },
				endpoint: null
			},
			{
				id: "/docs",
				pattern: /^\/docs\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 5 },
				endpoint: null
			},
			{
				id: "/docs/architecture",
				pattern: /^\/docs\/architecture\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 6 },
				endpoint: null
			},
			{
				id: "/docs/architecture/crdts",
				pattern: /^\/docs\/architecture\/crdts\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 7 },
				endpoint: null
			},
			{
				id: "/docs/architecture/encryption",
				pattern: /^\/docs\/architecture\/encryption\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 8 },
				endpoint: null
			},
			{
				id: "/docs/architecture/identity",
				pattern: /^\/docs\/architecture\/identity\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 9 },
				endpoint: null
			},
			{
				id: "/docs/architecture/networking",
				pattern: /^\/docs\/architecture\/networking\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 10 },
				endpoint: null
			},
			{
				id: "/docs/architecture/voice",
				pattern: /^\/docs\/architecture\/voice\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 11 },
				endpoint: null
			},
			{
				id: "/docs/getting-started",
				pattern: /^\/docs\/getting-started\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 12 },
				endpoint: null
			},
			{
				id: "/docs/guides",
				pattern: /^\/docs\/guides\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 13 },
				endpoint: null
			},
			{
				id: "/docs/guides/channels",
				pattern: /^\/docs\/guides\/channels\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 14 },
				endpoint: null
			},
			{
				id: "/docs/guides/inviting-peers",
				pattern: /^\/docs\/guides\/inviting-peers\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 15 },
				endpoint: null
			},
			{
				id: "/docs/guides/moderation",
				pattern: /^\/docs\/guides\/moderation\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 16 },
				endpoint: null
			},
			{
				id: "/docs/guides/swarms",
				pattern: /^\/docs\/guides\/swarms\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 17 },
				endpoint: null
			},
			{
				id: "/docs/guides/voice-chat",
				pattern: /^\/docs\/guides\/voice-chat\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 18 },
				endpoint: null
			},
			{
				id: "/docs/search.json",
				pattern: /^\/docs\/search\.json\/?$/,
				params: [],
				page: null,
				endpoint: __memo(() => import('./entries/endpoints/docs/search.json/_server.ts.js'))
			},
			{
				id: "/download",
				pattern: /^\/download\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 19 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
