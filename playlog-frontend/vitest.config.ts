import {defineConfig} from 'vitest/config';

export default defineConfig({
	test: {
		server: {
			deps: {
				inline: ['zone.js/testing'],
			},
		},
	},
	css: {
		preprocessorOptions: {
			scss: {
				silenceDeprecations: ['import', 'global-builtin'],
			},
		},
	},
});
