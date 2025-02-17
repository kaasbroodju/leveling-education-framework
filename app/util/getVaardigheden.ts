import path from "path";
import fsPromises from "fs/promises";
import { BeroepstakenOrVaardigheden } from "../types/BeroepstakenOrVaardigheden";
import {Skill, SkillLevels, skills} from "../types/Vaardigheid";
import {architecture_layers} from "../types/Architectuurlaag";
import {activities} from "../types/Activiteit";
import * as fs from "node:fs";
import {Niveau, niveaus} from "../types/Niveau";
import {PrismaClient} from "@prisma/client";

export async function getVaardigheden(
    locale?: "nl" | "en"
): Promise<SkillLevels> {
    const prisma = new PrismaClient();
    return (await prisma.skillDescription.findMany())
        .reduce((acc, vaardigheid) => {
            if (!acc[vaardigheid.skillId as Skill]) {
                acc[vaardigheid.skillId as Skill] = {};
            }

            // @ts-ignore
            acc[vaardigheid.skillId as Skill][`${vaardigheid.level}` as Niveau] = {
                title: vaardigheid.description,
                info: vaardigheid.sublament
            }
            return acc;
        }, {} as Partial<{ [key in Skill]: Partial<{ [key in Niveau]: { title: string, info: string | null } }> }>
        ) as SkillLevels
}
