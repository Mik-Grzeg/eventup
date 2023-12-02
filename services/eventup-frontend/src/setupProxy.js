// src/setupProxy.js
/*
const { createProxyMiddleware } = require('http-proxy-middleware');

const proxy = {
  target: 'http://localhost:8080', // Update with your backend URL
  changeOrigin: true,
};

module.exports = function (app) {
  app.use(
    '/api',
    createProxyMiddleware(proxy, {
      onProxyRes: (proxyRes, req, res) => {
        // Add CORS headers to the proxy response
        res.setHeader('Access-Control-Allow-Origin', '*');
        res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE');
        res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
      },
    })
  );
};

*/