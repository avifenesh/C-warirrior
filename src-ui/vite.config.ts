import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { resolve } from 'path';

export default defineConfig({
	plugins: [sveltekit()],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true
	},
	envPrefix: ['VITE_', 'TAURI_'],
	resolve: {
		alias: {
			'$sprites': resolve('./static/sprites'),
			'$tiles': resolve('./static/tiles'),
			'$ui': resolve('./static/ui'),
			'$fonts': resolve('./static/fonts')
		}
	},
	assetsInclude: ['**/*.png', '**/*.json']
});
