import path from "path";
import fsPromises from "fs/promises";
import {architecture_layers, Architectuurlaag} from "../types/Architectuurlaag";
import {Activiteit, activities} from "../types/Activiteit";
import {Niveau, niveaus} from "../types/Niveau";
import {BeroepsProduct} from "../types/BeroepsProduct";
import {BeroepsProducten} from "../types/HBOI";

export async function getBeroepsproducten(): Promise<BeroepsProducten> {
    const output: Partial<BeroepsProducten> = {}


    for (let architectureLayer of architecture_layers) {
        for (let activity of activities) {
            const filePath = path.join(process.cwd(), `datav2/examples/${architectureLayer} ${activity}`);

            const beroepspoduct: Partial<{ [key in Niveau]: BeroepsProduct[]}> = {}
            for (const level of niveaus) {
                const fileContent = await fsPromises.readFile(path.join(filePath, `${level}.json`), "utf-8");
                beroepspoduct[level as Niveau] = JSON.parse(fileContent) as BeroepsProduct[];
            }

            output[`${architectureLayer} ${activity}`] = beroepspoduct as { [key in Niveau]: BeroepsProduct[]};
        }
    }

    return output as BeroepsProducten
}
