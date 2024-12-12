import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
    integrations: [
        starlight({
            title: 'BELLE site',
            social: {
                github: 'https://github.com/BlueGummi/belle',
            },
            sidebar: [
                {
                    label: 'Program Documentation',
                    items: [
                        { label: 'Overview', slug: '' },
			            { label: 'Assembler', slug: 'basm'},
			            { label: 'Emulator', slug: 'belle'},
			            { label: 'Diassembler', slug: 'bdump'},
			            { label: 'Utilities', slug: 'btils'},
                    ],
                },
                {
                    label: 'ISA reference',
                    items: [
                        { label: 'Overview', slug: 'overview'},
                        { label: 'Encoding', slug: 'encoding'},
                        { label: 'Instructions', slug: 'instructions'},
                    ],
                },
                {
                    label: 'Hardware',
                    items: [
                        { label: 'CPU specification', slug: 'cpu-core'},
                        { label: 'Flags', slug: 'cpu-flags'},
                        { label: 'Errors', slug: 'cpu-errors'},
                        { label: 'Memory', slug: 'memory'},
                    ],
                },
            ],
        }),
    ],
});
