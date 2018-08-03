# TODO:
  - blockstack profiles
    - linking stellar addresses and blockstack profiles
  - stellar schemas, queries and resolvers (mutations come later)
  - stellar scalars/directives for signatures, public keys, etc
  - subscriptions
    - activitystreams/blockstack: polling
    - stellar: SSE from nebula? 

# mutation resolver
  - gets from `context`:
    - `blockstackClient`
    - `mutationState`:
      - a `Map` of `export`ed variable name to FileManager
      - created if none exists already
    - `mutationTx`:
      - the series of tasks that commit FileManager changes
      - created if none exists already
  - creates a FileManager
    - gets FileManagers from previous directives from `context.mutationState`
      - performs updates on them (like linking), if necessary
    - performs updates on current FileManager (like linking)
    - if `export`ed, sets current FileManager to `context.mutationState` 
  - adds task to `context.mutationTx`:
    - calls `FileManager.commit()`
  - if `commit === true`
    - call and return `context.mutationTx.proceed()`

# createBlogPost(post)
  + order of operations:
    - `@collection(type, value, export)`
      - create a collection object (with random id)
      - `export` adds collection to `context` as `'replies'`
      - creates task to commit collection (calls `commit()` on collection)
    - `@file(type, value, export)`
      - creates a file (with random id)
      - `export` adds file to `context` as `'coverImage'`
      - adds task to commit file (calls `commit()` on file)
    - `@object(type, value, links, export)`
      - creates an object (with random id)
      - sets links for post properties `replies` and `coverImage`:
        - gets `#replies` and `#coverImage` from `context`
        - set property value `post[from].href` to object property `[to].url`
      - `export` adds object to `context` as `#post`
      - adds task to commit object (calls `commit()` on object)
    - `@link(type, id, value, linksTo, commit)`
      - gets collection with `id=blog`
      - get `#post` from `context`
      - set link:
        - `link.href` to `post.url`
      - adds link to collection
      - adds task to commit collection (calls `commit()` on collection)
      - `commit: true`
        - gets array of tasks from `context`
        - 
