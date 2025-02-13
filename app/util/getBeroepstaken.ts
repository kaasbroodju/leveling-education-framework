import path from "path";
import fsPromises from "fs/promises";
import { BeroepstakenOrVaardigheden } from "../types/BeroepstakenOrVaardigheden";
import { Skill } from "../types/Vaardigheid";
import {architecture_layers, Architectuurlaag} from "../types/Architectuurlaag";
import {Activiteit, activities} from "../types/Activiteit";
import * as fs from "node:fs";
import {Niveau, niveaus} from "../types/Niveau";
import {BeroepsTaken} from "../types/HBOI";
import {BeroepsProduct} from "../types/BeroepsProduct";

export async function getBeroepstaken(
    locale?: "nl" | "en"
): Promise<BeroepsTaken> {
    const output: Partial<BeroepsTaken> = {}
    if (!locale) locale = "nl"

    for (let architectureLayer of architecture_layers) {
        for (let activity of activities) {
            const filePath = path.join(process.cwd(), `datav2/hboi/${locale}/${architectureLayer} ${activity}`);

            const beroepspoduct: Partial<{ [key in Niveau]: { title: string, info: string | null }}> =  {}
            for (const level of niveaus) {
                const description = await fsPromises.readFile(path.join(filePath, level, "description.txt"), "utf-8");

                let info = null;
                if (fs.existsSync(path.join(filePath, level, "info.txt"))) {
                    info = await fsPromises.readFile(path.join(filePath, level, "info.txt"), "utf-8");
                }

                beroepspoduct[level as Niveau] = {
                    title: description,
                    info
                }
            }

            output[`${architectureLayer} ${activity}`] = beroepspoduct as { [key in Niveau]: { title: string, info: string | null }}
        }
    }

    return output as BeroepsTaken
}
