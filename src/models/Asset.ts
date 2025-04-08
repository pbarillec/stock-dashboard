export interface Asset {
  id: number;
  symbol: string;
  name: string;
  category: string;
  api_id?: string | null;
}
