Assumptions:

- each query with a `@rest` directive will use the `endpoint` param to 
identify the actual URL to use
    - (when used with multiple `RestLink`s) the top-level `customFetch` will 
    use the URL to route requests to the appropriate `customFetch` function

Exports:

- `mergeConfigs(baseConfig, ...endpointConfigs)`:
    - merges a base config, with multiple endpoint-specific configs to create
     one `apollo-link-rest` config object
    - the new config will contain the merged endpoints and a 
    top-level `customFetch` function that routes requests based on the 
    query's `endpoint` param
    - ```
        type BaseConfig = {
            headers?
            credentials?
            typePatcher: {...}
        }
        ```
    - ```
        type EndpointConfig = {
            [endpointName]: {
                endpointURL
                customFetch
                
            }
        }
        ```
