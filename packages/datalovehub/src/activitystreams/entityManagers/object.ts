import {
  IEntityManager,
  IEntityManagerStatic,
} from '../transactionManagers/base';
import {ITransactionEngine} from "./base";
import {IGetFileOptions} from "../../blockstack";

export interface IState {

}
export interface OState {

}
export interface View {

}

const ObjectManager: IEntityManagerStatic<IState, OState, View> = class ObjectManager<IState, OState, View> implements IEntityManager<IState, OState, View> {
  public static getByName(name: string, options: IGetFileOptions, TransactionEngine: ITransactionEngine): Promise<ObjectManager> {},
  public static getByFileURN(fileURN: string, TransactionEngine: ITransactionEngine): Promise<ObjectManager> {},

  private _state: any;

  constructor(): void {}

  // public id(): string {}
  // public view(): any {}
  //
  // public update(fn: (state: any) => Promise<any>): Promise<this> {
  //   return Promise.resolve(this);
  // }
  // public commit(): Promise<any> {}
};

export default ObjectManager;
