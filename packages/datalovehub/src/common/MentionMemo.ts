import * as MentionCount from './core/MentionCount';
import * as ObjectMemo from './ObjectMemo';

/**
 * MentionMemo
 */

export type T = string;
export interface IOptions extends ObjectMemo.IOptions {
  mentionCount?: MentionCount.T,
}
export type ISplitParts = ISplitParts1 | ISplitParts2;
export interface ISplitParts1 extends ObjectMemo.ISplitParts {
  mentionCount: string,
}
export interface ISplitParts2 {
  objectMemo: ObjectMemo.T,
  mentionCount: string,
}

export const LENGTH = ObjectMemo.LENGTH + MentionCount.LENGTH;

export const validateParts = ({ mentionCount, ...parts }: ISplitParts): boolean =>
  ObjectMemo.validateParts(parts as ObjectMemo.ISplitParts)
    && MentionCount.validateDecoded(mentionCount);

export const _combine = (parts: ISplitParts): T => (
  (typeof (parts as ISplitParts2).objectMemo === 'string')
    ? (parts as ISplitParts2).objectMemo.concat(parts.mentionCount)
    : ObjectMemo._combine(parts as ISplitParts1).concat(parts.mentionCount)
);

export const _split = (memo: T): ISplitParts2 => ({
  objectMemo: memo.slice(0, ObjectMemo.LENGTH),
  mentionCount: memo.substr(ObjectMemo.LENGTH, MentionCount.LENGTH),
});

export const from = ({ mentionCount = 1, ...options }: IOptions): T | null => {
  const objectMemo = ObjectMemo.from(options);
  const mentionCountString = MentionCount.encode(mentionCount);
  return (objectMemo === null || mentionCountString === null)
    ? null
    : _combine({ mentionCount: mentionCountString, objectMemo });
};

export const to = (memo: T): IOptions | null => {
  const parts = _split(memo);
  const objectMemoParts = ObjectMemo.to(parts.objectMemo);
  const mentionCount = MentionCount.decode(parts.mentionCount);
  return (objectMemoParts !== null && mentionCount !== null)
    ? Object.assign(parts, objectMemoParts, { mentionCount })
    : null;
};
