import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { resolve } from 'path';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
	plugins: [
		sveltekit(),
		wasm(),
		topLevelAwait()
	],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true
	},
	envPrefix: ['VITE_'],
	define: {
		'__API_URL__': JSON.stringify(process.env.API_URL || 'http://localhost:3000')
	},
	resolve: {
		alias: {
			'$sprites': resolve('./static/sprites'),
			'$tiles': resolve('./static/tiles'),
			'$ui': resolve('./static/ui'),
			'$fonts': resolve('./static/fonts'),
			'code-warrior-wasm': resolve('../src-wasm/pkg')
		}
	},
	assetsInclude: ['**/*.png', '**/*.json', '**/*.wasm'],
	optimizeDeps: {
		exclude: ['code-warrior-wasm']
	}
});
