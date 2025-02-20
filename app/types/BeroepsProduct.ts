import { Architectuurlaag } from "./Architectuurlaag";
import { Activiteit } from "./Activiteit";

export type BeroepsProduct = {
  id: string;
  architectureLayerId: Architectuurlaag;
  activityId: Activiteit;
  level: number;
  guild: string;
  title: string;
  sublament: string | null;
};
