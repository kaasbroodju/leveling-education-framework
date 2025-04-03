import { NextApiRequest, NextApiResponse } from "next";
import { db } from "../../../lib/db";
import { getServerSession } from "next-auth";
import { authOptions } from "../auth/[...nextauth]";

export default async function handler(
	req: NextApiRequest,
	res: NextApiResponse,
) {
	switch (req.method) {
		case "POST":
			const session = await getServerSession(req, res, authOptions);

			if (!session || session.user.role !== "teacher") {
				return res.status(403).json({ error: "Unauthorized" });
			}

			return createBeroepsproduct(req, res);
		case "GET":
			return getBeroepsproducten(req, res);
		default:
			return res.status(405).json({ message: "Method Not Allowed" });
	}
}

async function createBeroepsproduct(req: NextApiRequest, res: NextApiResponse) {
	const { title, layer, activity, guild } = req.body;

	const example = await db.hBOIExample.create({
		data: {
			title,
			architectureLayerId: layer,
			activityId: activity,
			guild,
		},
	});

	return res.status(200).json(example);
}

async function getBeroepsproducten(_: NextApiRequest, res: NextApiResponse) {
	const examples = await db.hBOIExample.findMany();

	return res.status(200).json(examples);
}
