import { Architectuurlaag } from "./Architectuurlaag";
import { Activiteit } from "./Activiteit";
import { Guild } from "./Guild";

export type BeroepsProduct = {
	id: string;
	architectureLayerId: Architectuurlaag;
	activityId: Activiteit;
	guild: Guild;
	title: string;
};
