import { Architectuurlaag } from "../types/Architectuurlaag";
import { Activiteit } from "../types/Activiteit";
import { BeroepsProduct } from "../types/BeroepsProduct";
import { BeroepsProducten } from "../types/HBOI";
import { db } from "../lib/db";

export async function getBeroepsproducten(): Promise<BeroepsProducten> {
	return (
		await db.hBOIExample.findMany({
			orderBy: [{ guild: "asc" }, { title: "asc" }],
		})
	).reduce(
		(acc, beroepsproduct) => {
			if (
				!acc[
					`${beroepsproduct.architectureLayerId} ${beroepsproduct.activityId}` as `${Architectuurlaag} ${Activiteit}`
				]
			) {
				acc[
					`${beroepsproduct.architectureLayerId} ${beroepsproduct.activityId}` as `${Architectuurlaag} ${Activiteit}`
				] = [];
			}

			// @ts-expect-error undefined
			acc[
				`${beroepsproduct.architectureLayerId} ${beroepsproduct.activityId}` as `${Architectuurlaag} ${Activiteit}`
			].push(beroepsproduct as BeroepsProduct);
			return acc;
		},
		{} as Partial<{
			[key in `${Architectuurlaag} ${Activiteit}`]: BeroepsProduct[];
		}>,
	) as BeroepsProducten;
}
