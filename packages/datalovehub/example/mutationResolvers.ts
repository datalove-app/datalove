import { createMutationOperationResolver } from '../src/activitystreams/resolvers/base';

export default {
  createBlogPost: createMutationOperationResolver<any>([
    Collection.create({ input: '$post.replies', exportName: 'replies' }),
    File.create({ input: '$post.coverImage', exportName: 'coverImage' }),
    Object.create({
      input: '$post.post',
      links: [
        { fromLink: 'replies.href', to: 'replies.id' },
        { fromLink: 'coverImage.href', to: 'coverImage.id' },
      ],
      exportName: 'post',
      return: true,
    }),
    Collection.appendLink({ name: 'blog', input: '$post.postLink', to: 'post' }),
  ]),
};
