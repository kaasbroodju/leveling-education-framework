import { Architectuurlaag } from "../types/Architectuurlaag";
import { Activiteit } from "../types/Activiteit";
import { Niveau } from "../types/Niveau";
import { BeroepsProduct } from "../types/BeroepsProduct";
import { BeroepsProducten } from "../types/HBOI";
import { db } from "../lib/db";

export async function getBeroepsproducten(): Promise<BeroepsProducten> {
  return (await db.hBOIExample.findMany()).reduce(
    (acc, beroepstaakNiveau) => {
      if (
        !acc[
          `${beroepstaakNiveau.architectureLayerId} ${beroepstaakNiveau.activityId}` as `${Architectuurlaag} ${Activiteit}`
        ]
      ) {
        acc[
          `${beroepstaakNiveau.architectureLayerId} ${beroepstaakNiveau.activityId}` as `${Architectuurlaag} ${Activiteit}`
        ] = {
          1: [],
          2: [],
          3: [],
          4: [],
        };
      }

      // @ts-ignore
      acc[
        `${beroepstaakNiveau.architectureLayerId} ${beroepstaakNiveau.activityId}` as `${Architectuurlaag} ${Activiteit}`
      ][`${beroepstaakNiveau.level}` as Niveau].push(beroepstaakNiveau);
      return acc;
    },
    {} as Partial<{
      [key in `${Architectuurlaag} ${Activiteit}`]: Partial<{
        [key in Niveau]: BeroepsProduct[];
      }>;
    }>,
  ) as BeroepsProducten;
}
