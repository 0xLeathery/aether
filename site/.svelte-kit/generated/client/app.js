export { matchers } from './matchers.js';

export const nodes = [
	() => import('./nodes/0'),
	() => import('./nodes/1'),
	() => import('./nodes/2'),
	() => import('./nodes/3'),
	() => import('./nodes/4'),
	() => import('./nodes/5'),
	() => import('./nodes/6'),
	() => import('./nodes/7'),
	() => import('./nodes/8'),
	() => import('./nodes/9'),
	() => import('./nodes/10'),
	() => import('./nodes/11'),
	() => import('./nodes/12'),
	() => import('./nodes/13'),
	() => import('./nodes/14'),
	() => import('./nodes/15'),
	() => import('./nodes/16'),
	() => import('./nodes/17'),
	() => import('./nodes/18'),
	() => import('./nodes/19')
];

export const server_loads = [];

export const dictionary = {
		"/": [3],
		"/demo": [4],
		"/docs": [5,[2]],
		"/docs/architecture": [6,[2]],
		"/docs/architecture/crdts": [7,[2]],
		"/docs/architecture/encryption": [8,[2]],
		"/docs/architecture/identity": [9,[2]],
		"/docs/architecture/networking": [10,[2]],
		"/docs/architecture/voice": [11,[2]],
		"/docs/getting-started": [12,[2]],
		"/docs/guides": [13,[2]],
		"/docs/guides/channels": [14,[2]],
		"/docs/guides/inviting-peers": [15,[2]],
		"/docs/guides/moderation": [16,[2]],
		"/docs/guides/swarms": [17,[2]],
		"/docs/guides/voice-chat": [18,[2]],
		"/download": [19]
	};

export const hooks = {
	handleError: (({ error }) => { console.error(error) }),
	
	reroute: (() => {}),
	transport: {}
};

export const decoders = Object.fromEntries(Object.entries(hooks.transport).map(([k, v]) => [k, v.decode]));
export const encoders = Object.fromEntries(Object.entries(hooks.transport).map(([k, v]) => [k, v.encode]));

export const hash = false;

export const decode = (type, value) => decoders[type](value);

export { default as root } from '../root.js';