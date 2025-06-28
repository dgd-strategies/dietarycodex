self.addEventListener('install', e => e.waitUntil(self.skipWaiting()));
self.addEventListener('activate', e => e.waitUntil(self.clients.claim()));

self.addEventListener('fetch', event => {
  const url = new URL(event.request.url);
  if (url.pathname === '/api/time') {
    const body = JSON.stringify({ time: new Date().toISOString() });
    event.respondWith(new Response(body, { headers: { 'Content-Type': 'application/json' } }));
  }
});
