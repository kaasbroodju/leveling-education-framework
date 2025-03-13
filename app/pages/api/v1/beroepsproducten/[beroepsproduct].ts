import { NextApiRequest, NextApiResponse } from "next";
import { db } from "../../../../lib/db";
import { getServerSession } from "next-auth";
import { authOptions } from "../../auth/[...nextauth]";

export default async function handler(
	req: NextApiRequest,
	res: NextApiResponse,
) {
	const session = await getServerSession(req, res, authOptions);

	if (!session || session.user.role !== "teacher") {
		return res.status(403).json({ error: "Unauthorized" });
	}

	switch (req.method) {
		case "PUT":
			return updateBeroeopsProduct(req, res);
		case "DELETE":
			return deleteBeroeopsProduct(req, res);
		default:
			return res.status(405).json({ message: "Method Not Allowed" });
	}
}

async function updateBeroeopsProduct(
	req: NextApiRequest,
	res: NextApiResponse,
) {
	const { beroepsproduct } = req.query as { beroepsproduct: string };
	const { title, layer, activity, guild, level, sublament } = req.body;
	const result = await db.hBOIExample.update({
		where: {
			id: beroepsproduct,
		},
		data: {
			title,
			architectureLayerId: layer,
			activityId: activity,
			guild,
			level,
			sublament,
		},
	});

	return res.status(200).json(result);
}

async function deleteBeroeopsProduct(
	req: NextApiRequest,
	res: NextApiResponse,
) {
	const { beroepsproduct } = req.query as { beroepsproduct: string };

	const deletedObj = await db.hBOIExample.delete({
		where: { id: beroepsproduct },
	});

	return res.status(200).json(deletedObj);
}
