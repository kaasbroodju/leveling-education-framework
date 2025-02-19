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
import {PrismaClient} from "@prisma/client";
import {db} from "../lib/db";

export async function getBeroepstaken(
    locale?: "nl" | "en"
): Promise<BeroepsTaken> {
    if (!locale) locale = "nl"


    return (await db.hBOIDescription.findMany())
        .reduce((acc, beroepstaakNiveau) => {
            if (!acc[`${beroepstaakNiveau.architectureLayerId} ${beroepstaakNiveau.activityId}` as `${Architectuurlaag} ${Activiteit}`]) {
                acc[`${beroepstaakNiveau.architectureLayerId} ${beroepstaakNiveau.activityId}` as `${Architectuurlaag} ${Activiteit}`] = {};
            }

            // @ts-ignore
            acc[`${beroepstaakNiveau.architectureLayerId} ${beroepstaakNiveau.activityId}` as `${Architectuurlaag} ${Activiteit}`][`${beroepstaakNiveau.level}` as Niveau] = {
                title: beroepstaakNiveau.description,
                info: beroepstaakNiveau.sublament
            }
            return acc;
        }, {} as Partial<{ [key in `${Architectuurlaag} ${Activiteit}`]: Partial<{ [key in Niveau]: { title: string, info: string | null } }> }>) as BeroepsTaken
}
