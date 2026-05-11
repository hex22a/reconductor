import { reactRouter } from '@react-router/dev/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
import relay from 'vite-plugin-relay';

export default defineConfig({
    plugins: [tailwindcss(), reactRouter(), relay],
    resolve: {
        tsconfigPaths: true,
    },
});
