import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({ fallback: '/404.html' })
	},
	preprocess: [
		preprocess({
			postcss: true
		})
	]
	// vitePlugin: {
	// 	experimental: {
	// 		inspector: {
	// 			holdMode: true,
	// 		}
	// 	}
	// }
};

export default config;
