import * as universal from '../entries/pages/_layout.js';

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/+layout.js";
export const imports = ["_app/immutable/nodes/0.rimU13EY.js","_app/immutable/chunks/DFIm6B6S.js","_app/immutable/chunks/0fw90vus.js","_app/immutable/chunks/B4qh9rw2.js","_app/immutable/chunks/Bh0EakZh.js","_app/immutable/chunks/CdNLware.js","_app/immutable/chunks/CsaWQ_OT.js","_app/immutable/chunks/DAmNXEaF.js","_app/immutable/chunks/Dw-fUg1R.js","_app/immutable/chunks/IXtJyCqN.js","_app/immutable/chunks/CgMBreGP.js","_app/immutable/chunks/CQHQo5ti.js","_app/immutable/chunks/D677U3-6.js","_app/immutable/chunks/DDc_57IC.js"];
export const stylesheets = ["_app/immutable/assets/0.CR_Ozq28.css"];
export const fonts = ["_app/immutable/assets/jetbrains-mono-cyrillic-wght-normal.D73BlboJ.woff2","_app/immutable/assets/jetbrains-mono-greek-wght-normal.Bw9x6K1M.woff2","_app/immutable/assets/jetbrains-mono-vietnamese-wght-normal.Bt-aOZkq.woff2","_app/immutable/assets/jetbrains-mono-latin-ext-wght-normal.DBQx-q_a.woff2","_app/immutable/assets/jetbrains-mono-latin-wght-normal.B9CIFXIH.woff2"];
