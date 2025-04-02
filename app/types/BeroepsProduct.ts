import { Architectuurlaag } from "./Architectuurlaag";
import { Activiteit } from "./Activiteit";

export type BeroepsProduct = {
	id: string;
	architectureLayerId: Architectuurlaag;
	activityId: Activiteit;
	guild: string;
	title: string;
};
