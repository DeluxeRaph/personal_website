module.exports = {
  apps: [
    {
      name: '0xfarmer-site',
      script: 'target/release/personal_website',
      cwd: __dirname,
      interpreter: 'none',
      env: {
        HOST: '0.0.0.0',
        PORT: '3000',
        PUBLIC_BASE_URL: 'https://0xfarmer.com',
        SHOP_TREASURY_ADDRESS: '0x73731620Bd6bB9EDc3c77E5982665fCBA797D710',
        X402_FACILITATOR_URL: 'https://x402.org/facilitator',
        SHOP_FULFILLMENT_MODE: 'local'
      },
      max_restarts: 10,
      restart_delay: 3000
    }
  ]
};
