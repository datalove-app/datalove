export default {
  profile: createResolver([
    Object.get({ id: 'profile', options: '$options' }),
  ]),

  blogPosts: createResolver([
    Collection.get({ id: 'blog', options: '$options' }),
  ]),

  blogPostReplies: createResolver([
    Collection.get({ id: '$postID', from: 'replies', options: '$options' })
  ]),

  blogPost: createResolver([
    Object.get({ id: '$postID', options: '$options' })
  ]),

  blogPostReply: createResolver([
    Object.get({ id: '$replyID', options: '$options' })
  ]),
};
