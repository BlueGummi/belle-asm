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
                    label: 'Hardware and ISA',
                    items: [
                        { label: 'Instruction Set', slug: 'isa'},
                    ],
                },
            ],
        }),
    ],
});
