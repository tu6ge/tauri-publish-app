/// <reference types="vitest" />
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import AutoImport from 'unplugin-auto-import/vite'
import { defineConfig } from 'vite'
import Components from 'unplugin-vue-components/vite'
import {
  AntDesignVueResolver,
} from 'unplugin-vue-components/resolvers'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    AutoImport({
      imports: ['vue'],
      dts: './src/auto-imports.d.ts',
      eslintrc: {
        enabled: true,
        filepath: resolve(__dirname, '.eslintrc-auto-import.json'),
      },
    }),
    Components({
      resolvers: [
        AntDesignVueResolver({
          //importCss:false,
        }),
      ]
    }),
  ],
  clearScreen: false,
  envPrefix: ['VITE_', 'TAURI_'],
  server: {
    port: 3000,
    strictPort: true,
  },
  resolve: {
    alias: {
      '@/': `${path.resolve(__dirname, 'src')}/`,
    }
  },
  build: {
    outDir: './dist',
    target: ['es2021', 'chrome97', 'safari13'],
    minify: !process.env.TAURI_DEBUG,
    sourcemap: !!process.env.TAURI_DEBUG,
    emptyOutDir: true,
  },
  test: {
    include: ['tests/unit/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
  },
})
