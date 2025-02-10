import path from "path";
import fsPromises from "fs/promises";
import {architecture_layers, Architectuurlaag} from "../types/Architectuurlaag";
import {Activiteit, activities} from "../types/Activiteit";
import {Niveau, niveaus} from "../types/Niveau";
import {BeroepsProduct} from "../types/BeroepsProduct";

export async function getBeroepsproducten(): Promise<{
    [key in `${Architectuurlaag} ${Activiteit}`]: {
        [key in Niveau]: BeroepsProduct[]
    }
}> {
    const output = {}


    for (let architectureLayer of architecture_layers) {
        for (let activity of activities) {
            const filePath = path.join(process.cwd(), `datav2/examples/${architectureLayer} ${activity}`);

            const beroepspoduct =  {}
            for (const level of niveaus) {
                beroepspoduct[level] = JSON.parse(await fsPromises.readFile(path.join(filePath, `${level}.json`))) as BeroepsProduct[]
            }

            output[`${architectureLayer} ${activity}`] = beroepspoduct
        }
    }

    return output
}
