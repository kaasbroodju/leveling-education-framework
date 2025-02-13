import {Architectuurlaag} from "./Architectuurlaag";
import {Activiteit} from "./Activiteit";
import {Niveau} from "./Niveau";
import {BeroepsProduct} from "./BeroepsProduct";

export type HBOILevels<T> = {
    [key in `${Architectuurlaag} ${Activiteit}`]: {
        [key in Niveau]: T
    }
}

export type BeroepsTaken = HBOILevels<{
    title: string;
    info: string | null;
}>

export type BeroepsProducten = HBOILevels<BeroepsProduct[]>