import { Lock as OLock } from 'lock';
import once from 'ramda/src/once';

export type IRelease = () => Promise<void>;
export interface ILock {
  (key: string): Promise<IRelease>,
  isLocked(): boolean,
}

export default function Lock(): ILock {
  const olock = OLock();
  const lock = async (key: string) => new Promise((resolve) => {
    olock(key, (_release) => {
      resolve(once(function release() {
        return new Promise((releaseResolve) => {
          _release(releaseResolve)();
        });
      }));
    });
  });

  (lock as ILock).isLocked = olock.isLocked;
  return (lock as ILock);
}
