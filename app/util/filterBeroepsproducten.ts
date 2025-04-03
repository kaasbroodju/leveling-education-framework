import { Architectuurlaag } from "../types/Architectuurlaag";
import { Activiteit } from "../types/Activiteit";
import { BeroepsProducten } from "../types/HBOI";

export function filterBeroepsproducten(
	beroepstaken: BeroepsProducten,
	{
		architectuurlaag,
		activiteit,
	}: {
		architectuurlaag?: Architectuurlaag;
		activiteit?: Activiteit;
	},
): Partial<BeroepsProducten> {
	// Filter beroepstaken based on query paramaters
	let filteredBeroepstaken = beroepstaken as Partial<BeroepsProducten>;
	if (architectuurlaag) {
		filteredBeroepstaken = Object.fromEntries(
			Object.entries(filteredBeroepstaken).filter(([key]) =>
				key.includes(architectuurlaag),
			),
		);
	}

	if (activiteit) {
		filteredBeroepstaken = Object.fromEntries(
			Object.entries(filteredBeroepstaken).filter(([key]) =>
				key.includes(activiteit),
			),
		);
	}

	return filteredBeroepstaken;
}
