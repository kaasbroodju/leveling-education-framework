import path from "path";
import fsPromises from "fs/promises";
import { BeroepstakenOrVaardigheden } from "../types/BeroepstakenOrVaardigheden";
import {Skill, skills} from "../types/Vaardigheid";
import {architecture_layers} from "../types/Architectuurlaag";
import {activities} from "../types/Activiteit";
import * as fs from "node:fs";
import {Niveau, niveaus} from "../types/Niveau";

export async function getVaardigheden(
    locale?: "nl" | "en"
): Promise<{
    [key in Skill]: {
        [key in Niveau]: {
            title: string;
            info: string;
        }
    }
}> {
    const output = {}
    if (!locale) locale = "nl"

    for (let skill of skills) {
        // for (let activity of activities) {
            const filePath = path.join(process.cwd(), `datav2/vaardigheden/${locale}/${skill}`);

            const beroepspoduct =  {}
            for (const level of niveaus) {
                const description = await fsPromises.readFile(path.join(filePath, level, "description.txt"), "utf-8");

                let info = null;
                if (fs.existsSync(path.join(filePath, level, "info.txt"))) {
                    info = await fsPromises.readFile(path.join(filePath, level, "info.txt"), "utf-8");
                }

                beroepspoduct[level] = {
                    title: description,
                    info
                }
            }

            output[skill] = beroepspoduct
        // }
    }

    return output
}
