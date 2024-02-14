import exp from "constants";

export interface IStockItem {
  barcode: string;
  product: string;
  quantity: number;
}

export interface IDelivery extends IStockItem {
  receiver: string;
  time: number;
}

export interface IProcurement {
  time: number;
}

export interface IReceiver {
  department: string;
  name: string;
}
