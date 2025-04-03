import { Architectuurlaag } from "../types/Architectuurlaag";
import { Activiteit } from "../types/Activiteit";
import { BeroepsProduct } from "../types/BeroepsProduct";
import { BeroepsProducten } from "../types/HBOI";
import { db } from "../lib/db";

export async function getBeroepsproducten(): Promise<BeroepsProducten> {
	console.log(process.env.USE_MOCK_DATA, process.env.USE_MOCK_DATA === "true")
	if (process.env.USE_MOCK_DATA === "true") {
		return {} as BeroepsProducten;
	}
	const products = (await db.hBOIExample.findMany({
		orderBy: [{ guild: "asc" }, { title: "asc" }],
	})) as BeroepsProduct[];

	return products.reduce<
		Partial<{
			[key in `${Architectuurlaag} ${Activiteit}`]: BeroepsProduct[];
		}>
	>((acc, beroepsproduct) => {
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
	}, {}) as BeroepsProducten;
}
