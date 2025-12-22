import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '')
  
  return {
    plugins: [react()],
    server: {
      proxy: {
        '/api': {
          target: `http://localhost:${env.BACKEND_PORT || '3000'}`,
          changeOrigin: true,
          secure: false,
        },
      }
    }
  }
})
