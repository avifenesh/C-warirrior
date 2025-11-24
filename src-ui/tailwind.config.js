/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				slate: {
					950: '#0a0f1a'
				}
			}
		}
	},
	plugins: []
};
