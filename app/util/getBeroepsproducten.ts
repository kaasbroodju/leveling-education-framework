import path from "path";
import fsPromises from "fs/promises";
import {architecture_layers, Architectuurlaag} from "../types/Architectuurlaag";
import {Activiteit, activities} from "../types/Activiteit";
import {Niveau, niveaus} from "../types/Niveau";
import {BeroepsProduct} from "../types/BeroepsProduct";
import {BeroepsProducten, BeroepsTaken} from "../types/HBOI";
import {PrismaClient} from "@prisma/client";

export async function getBeroepsproducten(): Promise<BeroepsProducten> {
    const output: Partial<BeroepsProducten> = {}

    const prisma = new PrismaClient();
    return (await prisma.hBOIExample.findMany())
        .reduce((acc, beroepstaakNiveau) => {
            if (!acc[`${beroepstaakNiveau.architectureLayerId} ${beroepstaakNiveau.activityId}` as `${Architectuurlaag} ${Activiteit}`]) {
                acc[`${beroepstaakNiveau.architectureLayerId} ${beroepstaakNiveau.activityId}` as `${Architectuurlaag} ${Activiteit}`] = {
                    1: [],
                    2: [],
                    3: [],
                    4: [],
                };
            }

            // @ts-ignore
            acc[`${beroepstaakNiveau.architectureLayerId} ${beroepstaakNiveau.activityId}` as `${Architectuurlaag} ${Activiteit}`][`${beroepstaakNiveau.level}` as Niveau].push({
                guild: beroepstaakNiveau.guild,
                product: beroepstaakNiveau.title,
                desc: beroepstaakNiveau.sublament
            })
            return acc;
        }, {} as Partial<{ [key in `${Architectuurlaag} ${Activiteit}`]: Partial<{ [key in Niveau]: BeroepsProduct[] }> }>) as BeroepsProducten
}
