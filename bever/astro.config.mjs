import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
    integrations: [
        starlight({
            title: 'The BELLE website',
            social: {
                github: 'https://github.com/BlueGummi/belle',
            },
            sidebar: [
                {
                    label: 'Documentation',
                    items: [
                        { label: 'Overview', slug: '' },
			{ label: 'Assember', slug: 'basm'},
			{ label: 'Emulator', slug: 'belle'},
			{ label: 'Diassembler', slug: 'bdump'},
			{ label: 'Utilities', slug: 'btils'},
			{ label: 'Instruction Set', slug: 'isa'},
                    ],
                },
                {
                    label: 'Reference',
                    autogenerate: { directory: 'reference' },
                },
            ],
        }),
    ],
});
