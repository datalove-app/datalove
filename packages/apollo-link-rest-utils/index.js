/* global window */

const DEFAULT_FETCH = window.fetch;

export function mergeConfigs(baseConfig, ...endpointConfigs) {
  const endpoints = createEndpointsMap(endpointConfigs);
  const customFetch = createRoutedFetchFn(endpointConfigs);

  return {
    ...baseConfig,
    endpoints,
    customFetch,
  };
}

function createRoutedFetchFn(endpointConfigs) {
  const fetchers = endpointConfigs
    .reduce((acc, endpointConfig) => {
      const config = Object.values(endpointConfig)[0];
      acc[config.url] = config.customFetch;
      return acc;
    }, {});

  return function routedFetch(url, options) {
    const fetch = fetchers[url] || DEFAULT_FETCH;
    return fetch(url, options);
  };
}

function createEndpointsMap(endpointConfigs) {
  return endpointConfigs
    .reduce((acc, endpointConfig) => {
      const endpoint = Object.keys(endpointConfig)[0];
      acc[endpoint] = endpointConfig[endpoint].url;
      return acc;
    }, {});
}
