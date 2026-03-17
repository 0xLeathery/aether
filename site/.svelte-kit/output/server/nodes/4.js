

export const index = 4;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/demo/_page.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false
};
export const universal_id = "src/routes/demo/+page.js";
export const imports = ["_app/immutable/nodes/4.AliJBGzW.js","_app/immutable/chunks/DFIm6B6S.js","_app/immutable/chunks/0fw90vus.js","_app/immutable/chunks/CdNLware.js","_app/immutable/chunks/Bh0EakZh.js","_app/immutable/chunks/CsaWQ_OT.js","_app/immutable/chunks/DB5xlG3i.js","_app/immutable/chunks/DAmNXEaF.js","_app/immutable/chunks/OsX3oE0t.js","_app/immutable/chunks/BAAzKEpc.js"];
export const stylesheets = [];
export const fonts = [];
