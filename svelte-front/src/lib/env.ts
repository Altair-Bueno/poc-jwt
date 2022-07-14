export default {
  springPingConf: {
    basePath: import.meta.env.VITE_SPRINGPING_URL,
  },
  authServConf: {
    basePath: import.meta.env.VITE_AUTHSERV_URL,
  },
  credentialsConf: {
    key: import.meta.env.VITE_CREDENTIALS_KEY,
  },
  axumTransformConf: {
    basePath: import.meta.env.VITE_AXUMTRANSFORM_URL,
  },
};
