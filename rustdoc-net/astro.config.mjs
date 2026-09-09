// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

const base = (process.env.BASE_PATH || '/nu_plugin_net').replace(/\/$/, '') || '/';

export default defineConfig({
	base,
	site: 'https://averyfreeman.github.io',
	integrations: [
		starlight({
			title: 'nu_plugin_net',
			description: 'Documentation for the Nushell network-interface plugin.',
			disable404Route: true,
			favicon: '/Nushell.svg',
			customCss: ['./src/styles/custom.css'],
			components: {
				Header: './src/components/DocsHeader.astro',
			},
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/averyfreeman/nu_plugin_net' }],
			sidebar: [
				{
					label: 'Start here',
					items: [
						{ label: 'Overview', link: '/' },
						{ label: 'Install', slug: 'guides/install' },
						{ label: 'Use `net`', slug: 'guides/use-net' },
					],
				},
				{
					label: 'Reference',
					items: [
						{ label: 'Output schema', slug: 'reference/output' },
						{ label: 'Rust API', slug: 'api' },
					],
				},
				{
					label: 'Contributing',
					items: [{ label: 'Development', slug: 'development' }],
				},
			],
		}),
	],
});
