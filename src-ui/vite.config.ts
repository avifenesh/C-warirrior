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
	define: {
		'__API_URL__': JSON.stringify(process.env.API_URL || 'http://localhost:3000'),
		'__TAURI_BUILD__': JSON.stringify(!!process.env.TAURI_PLATFORM)
	},
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
