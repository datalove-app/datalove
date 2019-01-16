export default function main(sources: { ACTION: any }) {
  const pong$ = sources.ACTION
    .filter((action: { type: string }) => action.type === 'PING')
    .constant({ type: 'PONG' });

  return {
    ACTION: pong$,
  };
}
